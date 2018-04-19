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

bitflags! {
    /// MPIDR_EL1
    #[allow(non_camel_case_types)]
    pub struct MPIDR_EL1: u64 {
        /// Core0
        const CORE0 = 1;

        /// Core1
        const CORE1 = 1 << 1;

        /// Core2
        const CORE2 = 1 << 2;

        /// Core3
        const CORE3 = 1 << 3;
    }
}

impl MPIDR_EL1 {
    sys_coproc_read_raw!(u64, "MPIDR_EL1");
    read_flags!();

    /// Affinity level 0. Lowest level affinity field.
    #[inline]
    pub fn aff0(&self) -> u8 {
        (self.bits & 0xff) as u8
    }
}
