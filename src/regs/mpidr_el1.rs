// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2019 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! Multiprocessor Affinity Register - EL1
//!
//! In a multiprocessor system, provides an additional PE identification mechanism for scheduling
//! purposes.

use register::cpu::RegisterReadOnly;

pub struct Reg;

impl RegisterReadOnly<u64, ()> for Reg {
    sys_coproc_read_raw!(u64, "MPIDR_EL1");
}

pub static MPIDR_EL1: Reg = Reg {};
