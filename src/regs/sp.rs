// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2019 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! The stack pointer

use register::cpu::RegisterReadWrite;

pub struct Reg;

impl RegisterReadWrite<u64, ()> for Reg {
    read_raw!(u64, "sp");
    write_raw!(u64, "sp");
}

pub static SP: Reg = Reg {};
