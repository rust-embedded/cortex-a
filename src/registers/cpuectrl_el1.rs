// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2022 by the author(s)
//
// Author(s):
//   - Valentin B. <valentin.be@protonmail.com>

//! CPU Extended Control Register - EL1
//!
//! Provides additional implementation-defined configuration and control options
//! for the core.

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields,
};

register_bitfields! {u64,
    pub CPUECTRL_EL1 [
        /// Branch prediction structure invalidation.
        ///
        /// Software must write 1 to this bit to invalidate all entries from branch
        /// predictors. Writes have no effect on the value that is read on this bit,
        /// which is always 0.
        ///
        /// This feature can be used to mitigate attacks using branch injection
        /// vulnerability.
        GBPP OFFSET(63) NUMBITS(1) [],

        /// Threshold for direct stream to L4 cache on store.
        L4_STREAM OFFSET(22) NUMBITS(2) [
            /// 512KB threshold.
            ///
            /// NOTE: This is also the reset value.
            _512KB = 0b00,
            ///1024KB threshold.
            _1024KB = 0b01,
            /// 2048KB threshold.
            _2048KB = 0b10,
            /// Stream disabled.
            Disabled = 0b11
        ],

        /// Threshold for direct stream to L3 cache on store.
        L3_STREAM OFFSET(20) NUMBITS(2) [
            /// 64KB threshold.
            ///
            /// NOTE: This is also the reset value.
            _64KB = 0b00,
            /// 256KB  threshold.
            _256KB = 0b01,
            /// 512KB threshold.
            _512KB = 0b10,
            /// Stream disabled.
            Disabled = 0b11
        ],

        /// Threshold for direct stream to L2 cache on store.
        L2_STREAM OFFSET(18) NUMBITS(2) [
            /// 16KB threshold.
            ///
            /// NOTE: This is also the reset value.
            _16KB = 0b00,
            /// 64KB threshold.
            _64KB = 0b01,
            /// 128KB threshold.
            _128KB = 0b10,
            /// Stream disabled.
            Disabled = 0b11
        ],

        /// Enables L3 prefetch requests sent by the stride prefetcher.
        L3PF OFFSET(10) NUMBITS(1) [
            /// L3 prefetch requests are disabled.
            Disable = 0,
            /// L3 prefetch requests are enabled.
            ///
            /// NOTE: This is also the reset value.
            Enable = 1
        ],

        /// Enables L2 prefetch requests sent by the stride prefetcher.
        L2PF OFFSET(9) NUMBITS(1) [
            /// L2 prefetch requests are disabled.
            Disable = 0,
            /// L2 prefetch requests are enabled.
            ///
            /// NOTE: This is also the reset value.
            Enable = 1
        ],

        /// Enables L1 prefetch requests sent by the stride prefetcher.
        L1PF OFFSET(8) NUMBITS(1) [
            /// L1 prefetch requests are disabled.
            Disable = 0,
            /// L1 prefetch requests are enabled.
            ///
            /// NOTE: This is also the reset value.
            Enable = 1
        ],

        /// Enables L2 region prefetch requests.
        RPF OFFSET(7) NUMBITS(1) [
            /// L2 region prefetch requests are disabled.
            Disable = 0,
            /// L2 region prefetch requests are enabled.
            ///
            /// NOTE: This is also the reset value.
            Enable = 1
        ],

        /// Enables MMU prefetch requests.
        MMUPF OFFSET(6) NUMBITS(1) [
            /// MMU prefetch requests are disabled.
            Disable = 0,
            /// MMU prefetch requests are enabled.
            ///
            /// NOTE: This is also the reset value.
            Enable = 1
        ],

        /// L2 region prefetcher aggressibility.
        RPF_AGGRO OFFSET(5) NUMBITS(1) [
            /// The L2 region prefetcher is less aggressive, with a longer learning phase.
            LessAggro = 0,
            /// The L2 region prefetcher is more aggressive, with a shorter learning phase.
            ///
            /// NOTE: This is also the reset value.
            MoreAggro = 1
        ],

        /// Enables signaling of Cacheable Exclusive Loads on the internal interface between the
        /// core and the DSU.
        RNSD_EXCL OFFSET(1) NUMBITS(1) [
            /// Cacheable Exclusive Loads do not use the Exclusive attribute on the internal
            /// interface between the core and the DSU.
            NoExclusive = 0,
            /// Cacheable Exclusive Loads use the Exclusive attribute on the internal interface
            /// between the core and the DSU.
            Exclusive = 1
        ],

        EXTLLC OFFSET(0) NUMBITS(1) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_read_raw!(u64, "S3_0_C15_C1_4", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_write_raw!(u64, "S3_0_C15_C1_4", "x");
}

pub const CPUECTRL_EL1: Reg = Reg;
