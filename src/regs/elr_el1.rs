// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2019 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! Exception Link Register - EL1
//!
//! When taking an exception to EL1, holds the address to return to.

use register::cpu::RegisterReadWrite;

pub struct Reg;

impl RegisterReadWrite<u64, ()> for Reg {
    sys_coproc_read_raw!(u64, "ELR_EL1");
    sys_coproc_write_raw!(u64, "ELR_EL1");
}

pub static ELR_EL1: Reg = Reg {};
