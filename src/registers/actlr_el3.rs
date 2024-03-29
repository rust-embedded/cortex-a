// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2022 by the author(s)
//
// Author(s):
//   - Valentin B. <valentin.be@protonmail.com>

//! Auxiliary Control Register - EL3
//!
//! Provides implementation-defined configuration and control options for execution
//! at EL3.

use tock_registers::interfaces::{Readable, Writeable};

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_read_raw!(u64, "ACTLR_EL3", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_write_raw!(u64, "ACTLR_EL3", "x");
}

pub const ACTLR_EL3: Reg = Reg;
