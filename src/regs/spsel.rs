// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2019 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! Stack Pointer Select
//!
//! Allows the Stack Pointer to be selected between SP_EL0 and SP_ELx.

use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u32,
    SPSel [
        /// Stack pointer to use. Possible values of this bit are:
        ///
        /// 0 Use SP_EL0 at all Exception levels.
        /// 1 Use SP_ELx for Exception level ELx.
        ///
        /// When this register has an architecturally-defined reset value, this field resets to 1.
        SP OFFSET(0) NUMBITS(1) [
            EL0 = 0,
            ELx = 1
        ]
    ]
}

pub struct Reg;

impl RegisterReadWrite<u32, SPSel::Register> for Reg {
    sys_coproc_read_raw!(u32, "SPSEL");
    sys_coproc_write_raw!(u32, "SPSEL");
}

#[allow(non_upper_case_globals)]
pub static SPSel: Reg = Reg {};
