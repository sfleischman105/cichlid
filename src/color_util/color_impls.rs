#[cfg(feature = "no-std")]
use core::slice;
#[cfg(not(feature = "no-std"))]
use std::slice;

use crate::ColorRGB;

// Developer note:
//
// A specific trait is created for `&mut [ColorRGB]` rather than just using `ColorIterMut`,
// and having it be generic over `IntoIter<Item = &mut ColorRGB>`. This mostly because
// we can optimize methods if they are over an array of elements, rather than being generic
// over an iterator. Specialization would otherwise be used, but is not yet stable.
//
// Generally, `ColorIterMut` methods are those that are write only modification of each
// ColorRGB, while `ColorSliceMut` is for methods requiring both reading and writing.
// ColorSliceMut also helps optimize array accesses due the natural alignment of
// ColorRGBs. Being an awkward alignment of 24 bits (3 bytes), Its impossible to completely
// load a ColorRGB into memory at once without an unaligned load (slow) or also loading
// part of another ColorRGB.

impl<'a, T: Sized + IntoIterator<Item = &'a mut ColorRGB>> super::ColorIterMut for T {
    fn fill(self, color: ColorRGB) {
        self.into_iter().for_each(|p| *p = color);
    }
}

// Theory of Operation:
//
// fade_to_black(u8) - the same as applying rgb.scale(fade);
// The optimization is based on scaling 4 bytes at once, rather than individually (as would
// be accomplished with a regular iterator). Estimating, it seems this optimized way of scaling
// takes 2.25 per byte (11 instr per 4 bytes), including a single load and store. The old way
// would take around 4 instructions per byte, including the load/store.
//
// However, this relies on the array being 4 byte aligned. The beginning and end of the array
// are reduced to fit within a 4 byte alignment. These ends have to be individually scaled,
// but that's alright, as max 3 bytes will be manually scaled.
//
// This optimization is mostly for Cortex-M processors, but seems to work otherwise.
// Benchmarking on a x86_64 machine shows that batch scaling is faster in all cases without
// SIMD optimizations, and almost always faster with SIMD extensions
// enabled (RUSTFLAGS=-Ctarget-cpu=native). The one place where regular scaling is faster
// seems to be in very small arrays (less than 12 u8).
//
// ARM has a Load Multiple instruction, (LDM Rn, {<reglist>}), which hopefully this is
// optimized down to.

impl<'a> super::ColorSliceMut for &'a mut [ColorRGB] {
    fn blur(self, blur_amount: u8) {
        let keep: u8 = 255 - blur_amount;
        let seep: u8 = blur_amount >> 1;
        let mut carry: ColorRGB = ColorRGB::Black;
        let mut iter = self.into_iter().peekable();
        while let Some(cur) = iter.next() {
            cur.scale(keep);
            *cur += carry;
            if let Some(nxt) = iter.peek() {
                let mut part: ColorRGB = **nxt;
                part.scale(seep);
                *cur += part;
                carry = part;
            }
        }
    }

    fn fade_to_black(self, fade_by: u8) {
        let len: usize = self.len();
        let raw_bytes: &mut [u8] = unsafe {
            let ptr = self.as_mut().as_mut_ptr() as *mut u8;
            slice::from_raw_parts_mut(ptr, len * 3)
        };
        batch_scale_bytes(raw_bytes, fade_by);
    }

    fn blend(self, other: ColorRGB, amount_of_other: u8) {
        let p_other: u16 = amount_of_other as u16;
        let p_this: u16 = (255 - amount_of_other) as u16;

        let partial_r = other.r as u16 * p_other;
        let partial_g = other.g as u16 * p_other;
        let partial_b = other.b as u16 * p_other;

        self.iter_mut().for_each(|p| {
            p.r = (((p.r as u16 * p_this) + partial_r) >> 8) as u8;
            p.g = (((p.g as u16 * p_this) + partial_g) >> 8) as u8;
            p.b = (((p.b as u16 * p_this) + partial_b) >> 8) as u8;
        });
    }
}

#[inline(always)]
fn scale_post(i: u8, scale: u16) -> u8 {
    (((i as u16) * scale) >> 8) as u8
}

/// Scales down an entire array by `scale`.
///
/// Rather than scaling each byte individually, scaling is done to two bytes at ones.
/// See the method `batch_scale_inner` for how this works, but practically this seems to
/// be around twice as fast as a iterating with `ColorRGB::scale(u8)`.
#[doc(hidden)]
#[inline]
pub fn batch_scale_bytes(x: &mut [u8], scale: u8) {
    let scalar: u16 = (scale as u16) + 1;
    let (head, mid, tail) = split_align32(x);
    head.iter_mut().for_each(|m| *m = scale_post(*m, scalar));
    mid.iter_mut()
        .for_each(|m| *m = batch_scale(*m, scalar as u32));
    tail.iter_mut().for_each(|m| *m = scale_post(*m, scalar));
}

#[inline(always)]
fn split_align32(x: &mut [u8]) -> (&mut [u8], &mut [u32], &mut [u8]) {
    let len = x.len();
    let ptr = x.as_mut_ptr();
    let (a, b, c) = aligned_split_u32(ptr as usize, len);
    unsafe {
        let cap = slice::from_raw_parts_mut(ptr, a);
        let mid = slice::from_raw_parts_mut(ptr.add(a) as *mut u32, b / 4);
        let end = slice::from_raw_parts_mut(ptr.add(a + b), c);
        (cap, mid, end)
    }
}

#[inline(always)]
fn aligned_split_u32(ptr: usize, len: usize) -> (usize, usize, usize) {
    if len <= 3 {
        return (len, 0, 0);
    }
    let len_1 = (ptr.wrapping_sub(1) ^ 0b11) & 0b11;
    let len_2 = (len - len_1) & !0b11;
    let len_3 = (len - len_1) & 0b11;
    debug_assert_eq!(len, len_1 + len_2 + len_3);
    debug_assert_eq!(len_2 & 0b11, 0);
    debug_assert!(len_1 <= 3);
    debug_assert!(len_3 <= 3);
    (len_1, len_2, len_3)
}

#[inline(always)]
fn batch_scale(x: u32, scalar: u32) -> u32 {
    let mut bytes_02: u32 = x & 0x00FF00FF;
    let mut bytes_13: u32 = x & 0xFF00FF00;
    bytes_13 = bytes_13 >> 8;
    bytes_02 *= scalar;
    bytes_13 *= scalar;
    bytes_02 = bytes_02 >> 8;
    bytes_02 &= 0x00FF00FF;
    bytes_13 &= 0xFF00FF00;
    debug_assert_eq!(bytes_02 & 0xFF00FF00, 0);
    debug_assert_eq!(bytes_13 & 0x00FF00FF, 0);
    bytes_02 | bytes_13
}

#[cfg(test)]
mod test {
    use crate::color_util::color_impls::{batch_scale_bytes, scale_post};

    fn rand_change(seed: &mut u64) -> u64 {
        *seed ^= *seed >> 12;
        *seed ^= *seed << 25;
        *seed ^= *seed >> 27;
        seed.wrapping_mul(2685_8216_5773_6338_717)
    }

    fn collapse_u64(x: u64) -> u8 {
        ((x >> 15) ^ (x >> 40) ^ x) as u8
    }

    #[test]
    fn test_batch_scale_array() {
        const BUF_LEN: usize = 2048;
        let mut buffer = [0u8; BUF_LEN];
        let mut buf_batch = [0u8; BUF_LEN];
        let mut buf_reg = [0u8; BUF_LEN];

        buffer
            .iter_mut()
            .enumerate()
            .for_each(|(i, p)| *p = i as u8);

        buf_batch.clone_from_slice(&mut buffer);
        buf_reg.clone_from_slice(&mut buffer);

        for scale in 0..=255 {
            batch_scale_bytes(&mut buf_batch, scale);

            buf_reg
                .iter_mut()
                .for_each(|v| *v = scale_post(*v, (scale as u16) + 1));

            buf_reg
                .iter()
                .zip(buf_batch.iter())
                .enumerate()
                .for_each(|(i, bytes)| {
                    if bytes.0 != bytes.1 {
                        panic!(
                            "i: {:4} ({:3}) - reg: {:4}, batch: {:4}  - scale: {}",
                            i,
                            i % 256,
                            bytes.0,
                            bytes.1,
                            scale
                        );
                    }
                });

            buf_batch.clone_from_slice(&mut buffer);
            buf_reg.clone_from_slice(&mut buffer);
        }
    }

    #[cfg(not(feature = "no-std"))]
    #[test]
    fn test_batch_scale_many_buf_len() {
        for it in 0..=5000 {
            let buffer: Vec<u8> = (0..).take(it).map(|b| b as u8).collect();

            let mut buf_batch = buffer.clone();
            let mut buf_reg = buffer.clone();

            for scale in 0..=255 {
                batch_scale_bytes(&mut buf_batch, scale);
                buf_reg
                    .iter_mut()
                    .for_each(|v| *v = scale_post(*v, (scale as u16) + 1));

                buf_reg
                    .iter()
                    .zip(buf_batch.iter())
                    .enumerate()
                    .for_each(|(i, bytes)| {
                        if bytes.0 != bytes.1 {
                            panic!(
                                "it: {}, i: {:4} ({:3}) - reg: {:4}, batch: {:4}  - scale: {}",
                                it,
                                i,
                                i % 256,
                                bytes.0,
                                bytes.1,
                                scale
                            );
                        }
                    });

                buf_batch = buffer.clone();
                buf_reg = buffer.clone();
            }
        }
    }

    #[cfg(not(feature = "no-std"))]
    #[test]
    fn test_batch_scale_many_alignment() {
        let mut seed: u64 = 11140122341;
        (0..).take(40).for_each(|_| {
            rand_change(&mut seed);
        });

        for it in 30..=4903 {
            rand_change(&mut seed);
            let buffer: Vec<u8> = (0..)
                .take(it)
                .map(|_| rand_change(&mut seed))
                .map(|x| collapse_u64(x))
                .collect();

            let mut buf_batch = buffer.clone();
            let mut buf_reg = buffer.clone();

            for scale in 0..=255 {
                let post_start: usize = (it / 1) % 4;
                let post_end: usize = buf_reg.len() - ((it - 1) % 11);

                batch_scale_bytes(&mut buf_batch[post_start..post_end], scale);

                buf_reg[post_start..post_end]
                    .iter_mut()
                    .for_each(|v| *v = scale_post(*v, (scale as u16) + 1));

                buf_reg
                    .iter()
                    .zip(buf_batch.iter())
                    .enumerate()
                    .take(2390)
                    .for_each(|(i, bytes)| {
                        if bytes.0 != bytes.1 {
                            panic!(
                                "it: {}, i: {:4} ({:3}) - reg: {:4}, batch: {:4}  - scale: {}",
                                it,
                                i,
                                i % 256,
                                bytes.0,
                                bytes.1,
                                scale
                            );
                        }
                    });

                buf_batch = buffer.clone();
                buf_reg = buffer.clone();
            }
        }
    }
}

//    given:
//    scale
//    i1
//    i2
//
//    start
//    scale16 <= scale + 1
//    routine_individual: 4 instr per byte
//    R0 <- i1 [byte]
//    R0 <- R0 * scale16
//    R0 <- R0 shr $8
//    i1 <- R0 [byte]
//    R0 <- i2 [byte]
//    R0 <- R0 * scale16
//    R0 <- R0 shr $16
//    i2 <- R0 [byte]
//
//    start
//    scale16 <= scale + 1
//    MASK_1_3 = 0xFF00FF00
//    MASK_0_2 = 0x00FF00FF
//
//    routine_4_byte: 11 instr per 4 byte (2.25 per byte)
//    R0 <- i [4 byte]
//    R1 <- R0 AND $MASK_1_3
//    R0 <- R0 AND $MASK_0_2
//    R1 <- R1 shr 8
//    R0 <- R0 * scale16
//    R1 <- R1 * scale16
//    R0 <- R0 shl 8
//    R1 <- R1 AND $MASK_1_3
//    R0 <- R0 AND $MASK_0_2
//    R0 <- R0 OR R1
//    i <- R0 [4 byte]
