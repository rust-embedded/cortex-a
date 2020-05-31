// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2020 by the author(s)
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
        () => unsafe { llvm_asm!("nop" :::: "volatile") },

        #[cfg(not(target_arch = "aarch64"))]
        () => unimplemented!(),
    }
}

/// Wait For Interrupt
///
/// For more details on wfi, refer to [here](http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0802a/CIHEGBBF.html)
#[inline(always)]
pub fn wfi() {
    match () {
        #[cfg(target_arch = "aarch64")]
        () => unsafe { llvm_asm!("wfi" :::: "volatile") },

        #[cfg(not(target_arch = "aarch64"))]
        () => unimplemented!(),
    }
}

/// Wait For Event
///
/// For more details of wfe - sev pair, refer to [here](http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0802a/CIHEGBBF.html)
#[inline(always)]
pub fn wfe() {
    match () {
        #[cfg(target_arch = "aarch64")]
        () => unsafe { llvm_asm!("wfe" :::: "volatile") },

        #[cfg(not(target_arch = "aarch64"))]
        () => unimplemented!(),
    }
}

/// Send EVent.Locally
///
/// SEV causes an event to be signaled to the local core within a multiprocessor system.
///
/// For more details of wfe - sev/sevl pair, refer to [here](http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0802a/CIHEGBBF.html)
#[inline(always)]
pub fn sevl() {
    match () {
        #[cfg(target_arch = "aarch64")]
        () => unsafe { llvm_asm!("sevl" :::: "volatile") },

        #[cfg(not(target_arch = "aarch64"))]
        () => unimplemented!(),
    }
}

/// Send EVent.
///
/// SEV causes an event to be signaled to all cores within a multiprocessor system.
///
/// For more details of wfe - sev pair, refer to [here](http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0802a/CIHEGBBF.html)
#[inline(always)]
pub fn sev() {
    match () {
        #[cfg(target_arch = "aarch64")]
        () => unsafe { llvm_asm!("sev" :::: "volatile") },

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
            llvm_asm!("eret" :::: "volatile");
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
            llvm_asm!("ret" :::: "volatile");
            core::intrinsics::unreachable()
        },

        #[cfg(not(target_arch = "aarch64"))]
        () => unimplemented!(),
    }
}
