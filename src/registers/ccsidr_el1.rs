// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2022 by the author(s)
//
// Author(s):
//   - Valentin B. <valentin.be@protonmail.com>

//! Current Cache Size ID Register - EL1
//!
//! Provides information about the architecture of the currently selected cache.

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields,
};

register_bitfields! {u64,
    /// The representation of `CCSIDR_EL1` when `FEAT_CCIDX` is implemented.
    pub CCSIDR_EL1_WITH_FEAT_CCIDX [
        /// Number of sets in cache.
        ///
        /// A value of 0 indicates 1 set in the cache. The number does not
        /// necessarily have to be a power of 2.
        NumSets OFFSET(32) NUMBITS(24) [],

        /// Associativity of cache.
        ///
        /// A value of 0 indicates an associativity of 1. The value does not
        /// necessarily have to be a power of 2.
        Associativity OFFSET(3) NUMBITS(21) [],

        /// Log2(Number of bytes in cache lline) - 4.
        ///
        /// **Examples:**
        ///
        /// - For a line length of 16 bytes: Log2(16) - 4 = 0. This is the minimum line length.
        ///
        /// - For a line length of 32 bytes: Log2(32) - 4 = 1.
        LineSize OFFSET(0) NUMBITS(3) []
    ],

    /// The representation of `CCSIDR_EL1` otherwise.
    pub CCSIDR_EL1 [
        /// Number of sets in cache.
        ///
        /// A value of 0 indicates 1 set in the cache. The number does not
        /// necessarily have to be a power of 2.
        NumSets OFFSET(13) NUMBITS(15) [],

        /// Associativity of cache.
        ///
        /// A value of 0 indicates an associativity of 1. The value does not
        /// necessarily have to be a power of 2.
        Associativity OFFSET(3) NUMBITS(10) [],

        /// Log2(Number of bytes in cache lline) - 4.
        ///
        /// **Examples:**
        ///
        /// - For a line length of 16 bytes: Log2(16) - 4 = 0. This is the minimum line length.
        ///
        /// - For a line length of 32 bytes: Log2(32) - 4 = 1.
        LineSize OFFSET(0) NUMBITS(3) []
    ]
}

pub struct RegWithFeatCcidx;
pub struct Reg;

impl Readable for RegWithFeatCcidx {
    type T = u64;
    type R = CCSIDR_EL1_WITH_FEAT_CCIDX::Register;

    sys_coproc_read_raw!(u64, "CCSIDR_EL1", "x");
}

impl Writeable for RegWithFeatCcidx {
    type T = u64;
    type R = CCSIDR_EL1_WITH_FEAT_CCIDX::Register;

    sys_coproc_write_raw!(u64, "CCSIDR_EL1", "x");
}

impl Readable for Reg {
    type T = u64;
    type R = CCSIDR_EL1::Register;

    sys_coproc_read_raw!(u64, "CCSIDR_EL1", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = CCSIDR_EL1::Register;

    sys_coproc_write_raw!(u64, "CCSIDR_EL1", "x");
}

pub const CCSIDR_EL1_WITH_FEAT_CCIDX: RegWithFeatCcidx = RegWithFeatCcidx;
pub const CCSIDR_EL1: Reg = Reg;
