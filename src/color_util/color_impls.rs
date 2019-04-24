#[cfg(feature = "no-std")]
use core::slice;
#[cfg(not(feature = "no-std"))]
use std::slice;

use crate::{ColorRGB, HSV};

impl<'a, T: Sized + IntoIterator<Item = &'a mut ColorRGB>> super::ColorIterMut for T {
    fn fill(self, color: ColorRGB) {
        self.into_iter().for_each(|p| *p = color);
    }
}

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
        let raw_bytes: &mut [u8] = unsafe {rgb_as_raw_bytes(self.as_mut())};
        batch_scale_u8(raw_bytes, fade_by);
    }

    fn blend(self, other: ColorRGB, amount_of_other: u8) {
        let p_other: u16 = amount_of_other as u16;
        let p_this: u16 = (255 - amount_of_other) as u16;

        let partial_r = other.r as u16 * p_other;
        let partial_g = other.g as u16 * p_other;
        let partial_b = other.b as u16 * p_other;

        self.iter_mut()
            .for_each(|p| {
                p.r = (((p.r as u16 * p_this) + partial_r) >> 8) as u8;
                p.g = (((p.g as u16 * p_this) + partial_g) >> 8) as u8;
                p.b = (((p.b as u16 * p_this) + partial_b) >> 8) as u8;
            });
    }
}

unsafe fn rgb_as_raw_bytes(rgbs: &mut [ColorRGB]) -> &mut [u8] {
    slice::from_raw_parts_mut(rgbs.as_mut_ptr() as *mut u8, rgbs.len() * 3)
}

#[inline(always)]
fn scale_post(i: u8, scale: u16) -> u8 {
    (((i as u16) * scale) >> 8) as u8
}

#[doc(hidden)]
#[inline(always)]
pub fn batch_scale_u8(x: &mut [u8], scale: u8) {
    let len: usize = x.len();
    if len <= 8 {
        let scalar: u16 = (scale as u16) + 1;
        x.iter_mut().for_each(|m| *m = scale_post(*m, scalar));
    } else {
        let start = x.as_mut_ptr();
        unsafe {
            let end = start.add(len - 1);
            batch_scale_ptr(start, end, scale);
        }
    }
}

// Assumes length > 8. End is the pointer to the last byte
#[inline]
unsafe fn batch_scale_ptr(mut start: *mut u8, mut end: *mut u8, scale: u8) {
    debug_assert!((end as usize) - (start as usize) >= 8);
    let scalar: u16 = (scale as u16) + 1;

    while (start as usize) % 4 != 0  {
        *start = scale_post(*start, scalar);
        start = start.add(1);
    }

    while (end as usize) % 4 != 0b11 {
        *end = scale_post(*end, scalar);
        end = end.sub(1);
    }

    end = end.add(1);
    let scalar: u32 = scalar as u32;

    while start as usize != end as usize {
        let word_ptr: *mut u32 = start as *mut u32;
        *word_ptr = batch_scale_inner(*word_ptr, scalar);
        start = start.add(4);
    }
}


#[inline]
fn batch_scale_inner(x: u32, scalar: u32) -> u32 {
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
    use crate::color_util::color_impls::{batch_scale_u8, scale_post, batch_scale_inner};

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
            batch_scale_u8(&mut buf_batch, scale);

            buf_reg.iter_mut()
                .for_each(|v| *v = scale_post(*v, (scale as u16) + 1));

            buf_reg.iter()
                .zip(buf_batch.iter())
                .enumerate()
                .for_each(|(i, bytes)| {
                    if bytes.0 != bytes.1 {
                        panic!("i: {:4} ({:3}) - reg: {:4}, batch: {:4}  - scale: {}",
                                 i, i % 256, bytes.0,bytes.1,scale);
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
            let buffer: Vec<u8> = (0..)
                .take(it)
                .map(|b| b as u8)
                .collect();

            let mut buf_batch = buffer.clone();
            let mut buf_reg = buffer.clone();

            for scale in 0..=255 {
                batch_scale_u8(&mut buf_batch, scale);
                buf_reg
                    .iter_mut()
                    .for_each(|v| *v = scale_post(*v, (scale as u16) + 1));

                buf_reg
                    .iter()
                    .zip(buf_batch.iter())
                    .enumerate()
                    .for_each(|(i, bytes)| {
                        if bytes.0 != bytes.1 {
                            panic!("it: {}, i: {:4} ({:3}) - reg: {:4}, batch: {:4}  - scale: {}",
                                   it, i, i % 256, bytes.0, bytes.1, scale);
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
        (0..).take(40).for_each(|_| {rand_change(&mut seed);});

        for it in 30..=5000 {
            rand_change(&mut seed);
            let buffer: Vec<u8> = (0..)
                .take(it)
                .map(|_| rand_change(&mut seed))
                .map(|x| collapse_u64(x))
                .collect();

            let mut buf_batch = buffer.clone();
            let mut buf_reg = buffer.clone();

            for scale in 0..=255 {
                let post_end: usize = buf_reg.len() - ((it - 1) % 11);

                batch_scale_u8(&mut buf_batch[(it % 4)..post_end], scale);

                buf_reg[(it % 4)..post_end]
                    .iter_mut()
                    .for_each(|v| *v = scale_post(*v, (scale as u16) + 1));

                buf_reg
                    .iter()
                    .zip(buf_batch.iter())
                    .enumerate()
                    .take(2390)
                    .for_each(|(i, bytes)| {
                        if bytes.0 != bytes.1 {
                            panic!("it: {}, i: {:4} ({:3}) - reg: {:4}, batch: {:4}  - scale: {}",
                                   it, i, i % 256, bytes.0, bytes.1, scale);
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

