// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2020 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! Main ID Register - EL1
//!
//! Provides identification information for the processor, including an implementer code for the device and a device ID number.

use register::cpu::RegisterReadOnly;

pub struct Reg;

impl RegisterReadOnly<u64, ()> for Reg {
    sys_coproc_read_raw!(u64, "MIDR_EL1");
}

pub static MIDR_EL1: Reg = Reg {};
