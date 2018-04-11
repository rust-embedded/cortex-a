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

//! Multiprocessor Affinity Register - EL1
//!
//! The processor and cluster IDs, in multi-core or cluster systems.

/// MPIDR_EL1
#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub struct MPIDR_EL1 {
    bits: u64,
}

impl MPIDR_EL1 {
    /// Returns the contents of the register as raw bits
    pub fn bits(&self) -> u64 {
        self.bits
    }

    /// Affinity level 0. Lowest level affinity field.
    #[inline]
    pub fn aff0(&self) -> u8 {
        (self.bits & 0xff) as u8
    }

    /// The core number
    #[inline]
    pub fn core_id(&self) -> u8 {
        self.aff0() & 0x3
    }
}

/// Reads the CPU register
#[inline]
pub fn read() -> MPIDR_EL1 {
    match () {
        #[cfg(target_arch = "aarch64")]
        () => {
            let r: u64;
            unsafe { asm!("mrs $0, MPIDR_EL1" : "=r"(r) ::: "volatile") }
            MPIDR_EL1 { bits: r }
        }

        #[cfg(not(target_arch = "aarch64"))]
        () => unimplemented!(),
    }
}
