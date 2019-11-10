// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2019 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! Counter-timer Frequency register - EL0
//!
//! This register is provided so that software can discover the frequency of the system counter. It
//! must be programmed with this value as part of system initialization. The value of the register
//! is not interpreted by hardware.

use register::cpu::RegisterReadOnly;

pub struct Reg;

impl RegisterReadOnly<u32, ()> for Reg {
    sys_coproc_read_raw!(u32, "CNTFRQ_EL0");
}

pub static CNTFRQ_EL0: Reg = Reg {};
