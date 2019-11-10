// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2019 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! Counter-timer Physical Count register - EL0
//!
//! Holds the 64-bit physical count value.

use register::cpu::RegisterReadOnly;

pub struct Reg;

impl RegisterReadOnly<u64, ()> for Reg {
    sys_coproc_read_raw!(u64, "CNTPCT_EL0");
}

pub static CNTPCT_EL0: Reg = Reg {};
