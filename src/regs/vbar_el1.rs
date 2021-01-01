// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2021 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! Vector Base Address Register - EL1
//!
//! Holds the vector base address for any exception that is taken to EL1.

use register::cpu::RegisterReadWrite;

pub struct Reg;

impl RegisterReadWrite<u64, ()> for Reg {
    sys_coproc_read_raw!(u64, "VBAR_EL1", "x");
    sys_coproc_write_raw!(u64, "VBAR_EL1", "x");
}

pub static VBAR_EL1: Reg = Reg {};
