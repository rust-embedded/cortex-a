/*
 * Copyright (c) 2018 by the author(s)
 *
 * =============================================================================
 *
 * Licensed under either of
 *   - Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
 *   - MIT License (http://opensource.org/licenses/MIT)
 * at your option.
 *
 * =============================================================================
 *
 * Author(s):
 *   - Jorge Aparicio
 *   - Andre Richter <andre.o.richter@gmail.com>
 */

//! The stack pointer

/// Reads the CPU register
#[inline]
pub fn read() -> u64 {
    match () {
        #[cfg(target_arch = "aarch64")]
        () => {
            let r;
            unsafe { asm!("mov $0, sp" : "=r"(r) ::: "volatile") }
            r
        }

        #[cfg(not(target_arch = "aarch64"))]
        () => unimplemented!(),
    }
}

/// Writes `bits` to the CPU register
#[cfg_attr(not(target_arch = "aarch64"), allow(unused_variables))]
#[inline]
pub unsafe fn write(bits: u64) {
    match () {
        #[cfg(target_arch = "aarch64")]
        () => asm!("mov sp, $0" :: "r"(bits) :: "volatile"),

        #[cfg(not(target_arch = "aarch64"))]
        () => unimplemented!(),
    }
}
