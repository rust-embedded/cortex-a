// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2020 by the author(s)
//
// Author(s):
//   - Erik Verbruggen <erikjv@me.com>

//! EL0 Read-Only Software Thread ID Register.
//!
//! Provides a location where software executing at EL1 or higher can store thread identifying
//! information that is visible to software executing at EL0, for OS management purposes.

use register::cpu::RegisterReadWrite;

pub struct Reg;

impl RegisterReadWrite<u64, ()> for Reg {
    sys_coproc_read_raw!(u64, "TPIDRRO_EL0");
    sys_coproc_write_raw!(u64, "TPIDRRO_EL0");
}

pub static TPIDRRO_EL0: Reg = Reg {};
