// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2019 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! Fault Address Register - EL2
//!
//! Holds the faulting Virtual Address for all synchronous Instruction or Data Abort, PC alignment
//! fault and Watchpoint exceptions that are taken to EL2.

use register::cpu::RegisterReadWrite;

pub struct Reg;

impl RegisterReadWrite<u64, ()> for Reg {
    sys_coproc_read_raw!(u64, "FAR_EL2");
    sys_coproc_write_raw!(u64, "FAR_EL2");
}

pub static FAR_EL2: Reg = Reg {};
