//! Architecture specific math functions

#[cfg(feature = "no-std")]
use core::mem;
#[cfg(not(feature = "no-std"))]
use std::mem;

macro_rules! dsp_call {
    ($name:expr, $a:expr, $b:expr) => {
        mem::transmute($name(mem::transmute($a), mem::transmute($b)))
    };
}

#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct uint8x4_t(pub u8, pub u8, pub u8, pub u8);

#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct uint16x2_t(pub u16, pub u16);

// http://www.keil.com/support/man/docs/armasm/armasm_dom1361289919288.htm
#[inline(always)]
pub fn uqadd8(a: uint8x4_t, b: uint8x4_t) -> uint8x4_t {
    match () {
        #[cfg(armv7em)]
        () => unsafe {
            extern "C" {
                #[link_name = "llvm.arm.uqadd8"]
                fn arm_uqadd8(a: u32, b: u32) -> u32;
            }
            dsp_call!(arm_uqadd8, a, b)
        },

        #[cfg(not(armv7em))]
        () =>  uint8x4_t(a.0.saturating_add(b.0),
                         a.1.saturating_add(b.1),
                         a.2.saturating_add(b.2),
                         a.3.saturating_add(b.3)),
    }
}

// http://www.keil.com/support/man/docs/armasm/armasm_dom1361289919618.htm
#[inline(always)]
pub fn uqadd16(a: uint16x2_t, b: uint16x2_t) -> uint16x2_t {
    match () {
        #[cfg(armv7em)]
        () => unsafe {
            extern "C" {
                #[link_name = "llvm.arm.uqadd16"]
                fn arm_uqadd16(a: u32, b: u32) -> u32;
            }
            dsp_call!(arm_uqadd16, a, b)
        },

        #[cfg(not(armv7em))]
        () =>  uint16x2_t(a.0.saturating_add(b.0),
                          a.1.saturating_add(b.1)),
    }
}

// http://www.keil.com/support/man/docs/armasm/armasm_dom1361289920727.htm
#[inline(always)]
pub fn uqsub8(a: uint8x4_t, b: uint8x4_t) -> uint8x4_t {
    match () {
        #[cfg(armv7em)]
        () => unsafe {
            extern "C" {
                #[link_name = "llvm.arm.uqsub8"]
                fn arm_uqsub8(a: u32, b: u32) -> u32;
            }
            dsp_call!(arm_uqsub8, a, b)
        },

        #[cfg(not(armv7em))]
        () =>  uint8x4_t(a.0.saturating_sub(b.0),
                         a.1.saturating_sub(b.1),
                         a.2.saturating_sub(b.2),
                         a.3.saturating_sub(b.3)),
    }
}

// http://www.keil.com/support/man/docs/armasm/armasm_dom1361289921077.htm
// uqsub16
#[inline(always)]
pub fn uqsub16(a: uint16x2_t, b: uint16x2_t) -> uint16x2_t {
    match () {
        #[cfg(armv7em)]
        () => unsafe {
            extern "C" {
                #[link_name = "llvm.arm.uqsub16"]
                fn arm_uqsub16(a: u32, b: u32) -> u32;
            }
            dsp_call!(arm_uqsub16, a, b)
        },

        #[cfg(not(armv7em))]
        () =>  uint16x2_t(a.0.saturating_sub(b.0),
                          a.1.saturating_sub(b.1)),
    }
}
