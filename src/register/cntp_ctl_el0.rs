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
 *   - Andre Richter <andre.o.richter@gmail.com>
 */

//! Counter-timer Physical Timer Control register - EL0

bitflags! {
    /// CNTP_CTL_EL0
    #[allow(non_camel_case_types)]
    pub struct CNTP_CTL_EL0: u64 {
        /// Enables the timer. Permitted values are:
        ///
        /// 0 Timer disabled.
        /// 1 Timer enabled.
        const ENABLE = 1;

        /// Timer interrupt mask bit. Permitted values are:
        ///
        /// 0 Timer interrupt is not masked by the IMASK bit.
        /// 1 Timer interrupt is masked by the IMASK bit.
        const IMASK = 1 << 1;

        /// The status of the timer. This bit indicates whether the
        /// timer condition is met:
        ///
        /// 0 Timer condition is not met.
        /// 1 Timer condition is met.
        const ISTATUS = 1 << 2;
    }
}

impl CNTP_CTL_EL0 {
    sys_coproc_read_raw!(u64, "CNTP_CTL_EL0");
    sys_coproc_write_raw!(u64, "CNTP_CTL_EL0");
    modify_raw!();
    read_flags!();
    write_flags!();
    modify_flags!();
}
