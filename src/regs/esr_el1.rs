// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2019 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! Exception Syndrome Register - EL1
//!
//! Holds syndrome information for an exception taken to EL1.

use register::{cpu::RegisterReadOnly, register_bitfields};

register_bitfields! {u32,
    ESR_EL1 [
        /// Exception Class. Indicates the reason for the exception that this register holds
        /// information about.
        ///
        /// For each EC value, the table references a subsection that gives information about:
        ///   - The cause of the exception, for example the configuration required to enable the
        ///     trap.
        ///   - The encoding of the associated ISS.
        ///
        /// Incomplete listing - to be done.
        EC  OFFSET(26) NUMBITS(6) [
            Unknown               = 0b00_0000,
            TrappedWFIorWFE       = 0b00_0001,
            TrappedFP             = 0b00_0111,
            IllegalExecutionState = 0b00_1110,
            SVC64                 = 0b01_0101,
            HVC64                 = 0b01_0110,
            SMC64                 = 0b01_0111,
            TrappedMsrMrs         = 0b01_1000,
            InstrAbortLowerEL     = 0b10_0000,
            InstrAbortCurrentEL   = 0b10_0001,
            PCAlignmentFault      = 0b10_0010,
            DataAbortLowerEL      = 0b10_0100,
            DataAbortCurrentEL    = 0b10_0101,
            SPAlignmentFault      = 0b10_0110,
            TrappedFP64           = 0b10_1100
        ],

        /// Instruction Length for synchronous exceptions.
        IL  OFFSET(25) NUMBITS(1) [],

        /// Instruction Specific Syndrome. Architecturally, this field can be defined independently
        /// for each defined Exception class. However, in practice, some ISS encodings are used for
        /// more than one Exception class.
        ISS OFFSET(0)  NUMBITS(25) []
    ]
}

pub struct Reg;

impl RegisterReadOnly<u32, ESR_EL1::Register> for Reg {
    sys_coproc_read_raw!(u32, "ESR_EL1");
}

#[allow(non_upper_case_globals)]
pub static ESR_EL1: Reg = Reg {};
