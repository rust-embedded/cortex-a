// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2020 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! System Control Register - EL1
//!
//! Provides top level control of the system, including its memory system, at EL1 and EL0.

use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u64,
    pub SCTLR_EL1 [
        /// Instruction access Cacheability control, for accesses at EL0 and
        /// EL1:
        ///
        /// 0 All instruction access to Normal memory from EL0 and EL1 are Non-cacheable for all
        ///   levels of instruction and unified cache.
        ///
        ///   If the value of SCTLR_EL1.M is 0, instruction accesses from stage 1 of the EL1&0
        ///   translation regime are to Normal, Outer Shareable, Inner Non-cacheable, Outer
        ///   Non-cacheable memory.
        ///
        /// 1 This control has no effect on the Cacheability of instruction access to Normal memory
        ///   from EL0 and EL1.
        ///
        ///   If the value of SCTLR_EL1.M is 0, instruction accesses from stage 1 of the EL1&0
        ///   translation regime are to Normal, Outer Shareable, Inner Write-Through, Outer
        ///   Write-Through memory.
        ///
        /// When the value of the HCR_EL2.DC bit is 1, then instruction access to Normal memory from
        /// EL0 and EL1 are Cacheable regardless of the value of the SCTLR_EL1.I bit.
        ///
        /// When ARMv8.1-VHE is implemented, and the value of HCR_EL2.{E2H, TGE} is {1, 1}, this bit
        /// has no effect on the PE.
        ///
        /// When this register has an architecturally-defined reset value, this field resets to 0.
        I OFFSET(12) NUMBITS(1) [
            NonCacheable = 0,
            Cacheable = 1
        ],

        /// Non-aligned access. This bit controls generation of Alignment faults at EL1 and EL0 under certain conditions.
        ///
        /// LDAPR, LDAPRH, LDAPUR, LDAPURH, LDAPURSH, LDAPURSW, LDAR, LDARH, LDLAR, LDLARH,
        /// STLLR, STLLRH, STLR, STLRH, STLUR, and STLURH will or will not generate an Alignment
        /// fault if all bytes being accessed are not within a single 16-byte quantity,
        /// aligned to 16 bytes for accesses.
        NAA OFFSET(6) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],

        /// SP Alignment check enable for EL0.
        ///
        /// When set to 1, if a load or store instruction executed at EL0 uses the SP
        /// as the base address and the SP is not aligned to a 16-byte boundary,
        /// then a SP alignment fault exception is generated.
        SA0 OFFSET(4) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],

        /// SP Alignment check enable.
        ///
        /// When set to 1, if a load or store instruction executed at EL1 uses the SP
        /// as the base address and the SP is not aligned to a 16-byte boundary,
        /// then a SP alignment fault exception is generated.
        SA OFFSET(3) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],

        /// Cacheability control, for data accesses.
        ///
        /// 0 All data access to Normal memory from EL0 and EL1, and all Normal memory accesses to
        ///   the EL1&0 stage 1 translation tables, are Non-cacheable for all levels of data and
        ///   unified cache.
        ///
        /// 1 This control has no effect on the Cacheability of:
        ///   - Data access to Normal memory from EL0 and EL1.
        ///   - Normal memory accesses to the EL1&0 stage 1 translation tables.
        ///
        /// When the value of the HCR_EL2.DC bit is 1, the PE ignores SCLTR.C. This means that
        /// Non-secure EL0 and Non-secure EL1 data accesses to Normal memory are Cacheable.
        ///
        /// When ARMv8.1-VHE is implemented, and the value of HCR_EL2.{E2H, TGE} is {1, 1}, this bit
        /// has no effect on the PE.
        ///
        /// When this register has an architecturally-defined reset value, this field resets to 0.
        C OFFSET(2) NUMBITS(1) [
            NonCacheable = 0,
            Cacheable = 1
        ],

        /// Alignment check enable. This is the enable bit for Alignment fault checking at EL1 and EL0.
        ///
        /// Instructions that load or store one or more registers, other than load/store exclusive
        /// and load-acquire/store-release, will or will not check that the address being accessed
        /// is aligned to the size of the data element(s) being accessed depending on this flag.
        ///
        /// Load/store exclusive and load-acquire/store-release instructions have an alignment check
        /// regardless of the value of the A bit.
        A OFFSET(1) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],

        /// MMU enable for EL1 and EL0 stage 1 address translation. Possible values of this bit are:
        ///
        /// 0 EL1 and EL0 stage 1 address translation disabled.
        ///   - See the SCTLR_EL1.I field for the behavior of instruction accesses to Normal memory.
        ///
        /// 1 EL1 and EL0 stage 1 address translation enabled.
        M OFFSET(0) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ]
    ]
}

pub struct Reg;

impl RegisterReadWrite<u64, SCTLR_EL1::Register> for Reg {
    sys_coproc_read_raw!(u64, "SCTLR_EL1", "x");
    sys_coproc_write_raw!(u64, "SCTLR_EL1", "x");
}

pub static SCTLR_EL1: Reg = Reg {};
