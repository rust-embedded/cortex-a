// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2021 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>
//   - Gregor Reitzenstein <me@dequbed.space>

//! Counter-timer Virtual Timer TimerValue register - EL0
//!
//! Holds the timer value for the EL1 virtual timer.

use register::cpu::RegisterReadWrite;

pub struct Reg;

impl RegisterReadWrite<u64, ()> for Reg {
    sys_coproc_read_raw!(u64, "CNTV_TVAL_EL0", "x");
    sys_coproc_write_raw!(u64, "CNTV_TVAL_EL0", "x");
}

pub static CNTV_TVAL_EL0: Reg = Reg {};
