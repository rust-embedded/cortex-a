// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2019 by the author(s)
//
// Author(s):
//   - Jorge Aparicio
//   - Andre Richter <andre.o.richter@gmail.com>

//! Miscellaneous assembly instructions

use core;

/// The classic no-op
#[inline(always)]
pub fn nop() {
    match () {
        #[cfg(target_arch = "aarch64")]
        () => unsafe { asm!("nop" :::: "volatile") },

        #[cfg(not(target_arch = "aarch64"))]
        () => unimplemented!(),
    }
}

/// Wait For Event
#[inline(always)]
pub fn wfe() {
    match () {
        #[cfg(target_arch = "aarch64")]
        () => unsafe { asm!("wfe" :::: "volatile") },

        #[cfg(not(target_arch = "aarch64"))]
        () => unimplemented!(),
    }
}

/// Exception return
///
/// Will jump to wherever the corresponding link register points to, and therefore never return.
#[inline(always)]
pub fn eret() -> ! {
    match () {
        #[cfg(target_arch = "aarch64")]
        () => unsafe {
            asm!("eret" :::: "volatile");
            core::intrinsics::unreachable()
        },

        #[cfg(not(target_arch = "aarch64"))]
        () => unimplemented!(),
    }
}

/// Function return
///
/// Will jump to wherever the corresponding link register points to, and therefore never return.
#[inline(always)]
pub fn ret() -> ! {
    match () {
        #[cfg(target_arch = "aarch64")]
        () => unsafe {
            asm!("ret" :::: "volatile");
            core::intrinsics::unreachable()
        },

        #[cfg(not(target_arch = "aarch64"))]
        () => unimplemented!(),
    }
}
