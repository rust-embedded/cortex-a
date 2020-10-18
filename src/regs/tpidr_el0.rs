// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2020 by the author(s)
//
// Author(s):
//   - Erik Verbruggen <erikjv@me.com>

//! Read/Write Software Thread ID Register - EL0.
//!
//! Provides a location where software executing at EL0 can store thread identifying information,
//! for OS management purposes.

use register::cpu::RegisterReadWrite;

pub struct Reg;

impl RegisterReadWrite<u64, ()> for Reg {
    sys_coproc_read_raw!(u64, "TPIDR_EL0", "x");
    sys_coproc_write_raw!(u64, "TPIDR_EL0", "x");
}

pub static TPIDR_EL0: Reg = Reg {};
