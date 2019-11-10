// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2019 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! The stack pointer - EL0
//!
//! Holds the stack pointer associated with EL0. At higher Exception levels, this is used as the
//! current stack pointer when the value of SPSel.SP is 0.

use register::cpu::RegisterReadWrite;

pub struct Reg;

impl RegisterReadWrite<u64, ()> for Reg {
    sys_coproc_read_raw!(u64, "SP_EL0");
    sys_coproc_write_raw!(u64, "SP_EL0");
}

pub static SP_EL0: Reg = Reg {};
