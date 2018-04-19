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

//! Counter-timer Frequency register - EL0

/// CNTFRQ_EL0
#[allow(non_camel_case_types)]
pub struct CNTFRQ_EL0;

impl CNTFRQ_EL0 {
    sys_coproc_read_raw!(u64, "CNTFRQ_EL0");
}
