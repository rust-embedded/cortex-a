// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2022 by the author(s)
//
// Author(s):
//   - Jorge Aparicio
//   - Andre Richter <andre.o.richter@gmail.com>

//! Wrappers around ARMv8-A instructions.

pub mod barrier;

/// The classic no-op
#[inline(always)]
pub fn nop() {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!("nop", options(nomem, nostack))
    }

    #[cfg(not(target_arch = "aarch64"))]
    unimplemented!()
}

/// Wait For Interrupt
///
/// For more details on wfi, refer to [here](http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0802a/CIHEGBBF.html).
#[inline(always)]
pub fn wfi() {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!("wfi", options(nomem, nostack))
    }

    #[cfg(not(target_arch = "aarch64"))]
    unimplemented!()
}

/// Wait For Event
///
/// For more details of wfe - sev pair, refer to [here](http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0802a/CIHEGBBF.html).
#[inline(always)]
pub fn wfe() {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!("wfe", options(nomem, nostack))
    }

    #[cfg(not(target_arch = "aarch64"))]
    unimplemented!()
}

/// Send EVent.Locally
///
/// SEV causes an event to be signaled to the local core within a multiprocessor system.
///
/// For more details of wfe - sev/sevl pair, refer to [here](http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0802a/CIHEGBBF.html).
#[inline(always)]
pub fn sevl() {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!("sevl", options(nomem, nostack))
    }

    #[cfg(not(target_arch = "aarch64"))]
    unimplemented!()
}

/// Send EVent.
///
/// SEV causes an event to be signaled to all cores within a multiprocessor system.
///
/// For more details of wfe - sev pair, refer to [here](http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0802a/CIHEGBBF.html).
#[inline(always)]
pub fn sev() {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!("sev", options(nomem, nostack))
    }

    #[cfg(not(target_arch = "aarch64"))]
    unimplemented!()
}

/// Exception return
///
/// Will jump to wherever the corresponding link register points to, and therefore never return.
#[inline(always)]
pub fn eret() -> ! {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!("eret", options(nomem, nostack));
        core::intrinsics::unreachable()
    }

    #[cfg(not(target_arch = "aarch64"))]
    unimplemented!()
}

/// Function return
///
/// Will jump to wherever the corresponding link register points to, and therefore never return.
#[inline(always)]
pub fn ret() -> ! {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!("ret", options(nomem, nostack));
        core::intrinsics::unreachable()
    }

    #[cfg(not(target_arch = "aarch64"))]
    unimplemented!()
}

/// Make an HVC32 call to the hypervisor, following the SMC Calling Convention version 1.3.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn hvc32(
    function: u32,
    arg1: u32,
    arg2: u32,
    arg3: u32,
    arg4: u32,
    arg5: u32,
    arg6: u32,
    arg7: u32,
) -> [u32; 8] {
    let mut ret = [0; 8];

    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!(
            "hvc #0",
            inout("w0") function => ret[0],
            inout("w1") arg1 => ret[1],
            inout("w2") arg2 => ret[2],
            inout("w3") arg3 => ret[3],
            inout("w4") arg4 => ret[4],
            inout("w5") arg5 => ret[5],
            inout("w6") arg6 => ret[6],
            inout("w7") arg7 => ret[7],
            options(nomem, nostack)
        )
    }

    #[cfg(not(target_arch = "aarch64"))]
    unimplemented!();

    ret
}

/// Make an HVC64 call to the hypervisor, following the SMC Calling Convention version 1.3.
#[inline(always)]
pub fn hvc64(function: u32, args: [u64; 17]) -> [u64; 18] {
    let mut ret = [0; 18];

    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!(
            "hvc #0",
            inout("x0") function as u64 => ret[0],
            inout("x1") args[0] => ret[1],
            inout("x2") args[1] => ret[2],
            inout("x3") args[2] => ret[3],
            inout("x4") args[3] => ret[4],
            inout("x5") args[4] => ret[5],
            inout("x6") args[5] => ret[6],
            inout("x7") args[6] => ret[7],
            inout("x8") args[7] => ret[8],
            inout("x9") args[8] => ret[9],
            inout("x10") args[9] => ret[10],
            inout("x11") args[10] => ret[11],
            inout("x12") args[11] => ret[12],
            inout("x13") args[12] => ret[13],
            inout("x14") args[13] => ret[14],
            inout("x15") args[14] => ret[15],
            inout("x16") args[15] => ret[16],
            inout("x17") args[16] => ret[17],
            options(nomem, nostack)
        )
    }

    #[cfg(not(target_arch = "aarch64"))]
    unimplemented!();

    ret
}

/// Make an SMC32 call to the firmware, following the SMC Calling Convention version 1.3.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn smc32(
    function: u32,
    arg1: u32,
    arg2: u32,
    arg3: u32,
    arg4: u32,
    arg5: u32,
    arg6: u32,
    arg7: u32,
) -> [u32; 8] {
    let mut ret = [0; 8];

    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!(
            "smc #0",
            inout("w0") function => ret[0],
            inout("w1") arg1 => ret[1],
            inout("w2") arg2 => ret[2],
            inout("w3") arg3 => ret[3],
            inout("w4") arg4 => ret[4],
            inout("w5") arg5 => ret[5],
            inout("w6") arg6 => ret[6],
            inout("w7") arg7 => ret[7],
            options(nomem, nostack)
        )
    }

    #[cfg(not(target_arch = "aarch64"))]
    unimplemented!();

    ret
}

/// Make an SMC64 call to the firmware, following the SMC Calling Convention version 1.3.
#[inline(always)]
pub fn smc64(function: u32, args: [u64; 17]) -> [u64; 18] {
    let mut ret = [0; 18];

    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!(
            "smc #0",
            inout("x0") function as u64 => ret[0],
            inout("x1") args[0] => ret[1],
            inout("x2") args[1] => ret[2],
            inout("x3") args[2] => ret[3],
            inout("x4") args[3] => ret[4],
            inout("x5") args[4] => ret[5],
            inout("x6") args[5] => ret[6],
            inout("x7") args[6] => ret[7],
            inout("x8") args[7] => ret[8],
            inout("x9") args[8] => ret[9],
            inout("x10") args[9] => ret[10],
            inout("x11") args[10] => ret[11],
            inout("x12") args[11] => ret[12],
            inout("x13") args[12] => ret[13],
            inout("x14") args[13] => ret[14],
            inout("x15") args[14] => ret[15],
            inout("x16") args[15] => ret[16],
            inout("x17") args[16] => ret[17],
            options(nomem, nostack)
        )
    }

    #[cfg(not(target_arch = "aarch64"))]
    unimplemented!();

    ret
}
