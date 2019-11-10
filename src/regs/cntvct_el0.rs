// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2019 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>
//   - Gregor Reitzenstein <me@dequbed.space>

//! Counter-timer Virtual Count register - EL0
//!
//! Holds the 64-bit virtual count value. The virtual count value is equal to the physical count
//! value in `CNTPCT_EL0` minus the virtual offset visible in `CNTVOFF_EL2`

use register::cpu::RegisterReadOnly;

pub struct Reg;

impl RegisterReadOnly<u64, ()> for Reg {
    sys_coproc_read_raw!(u64, "CNTVCT_EL0");
}

pub static CNTVCT_EL0: Reg = Reg {};
