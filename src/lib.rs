// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2022 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! Low level access to Cortex-A processors.
//!
//! ## Currently Supported Execution States
//!
//! - [x] AArch64
//! - [ ] AArch32
//!
//! ## Minimum Supported Rust Version
//!
//! Requires a recent nightly of Rust.
//!
//! ## Usage
//!
//! Please note that for using this crate's [register definitions](src/registers) (as provided by
//! `cortex_a::registers::*`), you need to also include
//! [`tock-registers`](https://crates.io/crates/tock-registers) in your project. This is because the
//! `interface` traits provided by `tock-registers` are implemented by this crate. You should
//! include the same version of `tock-registers` as is being used by this crate to ensure sane
//! interoperatbility.
//!
//! For example, in the following snippet, `X.Y.Z` should be the same version of `tock-registers`
//! that is mentioned in `cortex-a`'s [`Cargo.toml`](Cargo.toml).
//!
//! ```toml
//! [package]
//! name = "Your embedded project"
//!
//! # Some parts omitted for brevity.
//!
//! [dependencies]
//! tock-registers = "X.Y.Z"
//! cortex-a = "A.B.C"       # <-- Includes tock-registers itself.
//! ```
//!
//! ### Example
//!
//! Check out
//! [rust-raspberrypi-OS-tutorials](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials)
//! for usage examples. Listed below is a snippet of `rust-raspberrypi-OS-tutorials`'s early boot
//! code.
//!
//! ```rust
//! # #[cfg(feature = "nightly")]
//! use cortex_a::{asm, registers::*};
//! # #[cfg(feature = "nightly")]
//! use tock_registers::interfaces::Writeable; // <-- Trait needed to use `write()` and `set()`.
//!
//! // Some parts omitted for brevity.
//!
//! # #[cfg(feature = "nightly")]
//! unsafe fn prepare_el2_to_el1_transition(
//!     virt_boot_core_stack_end_exclusive_addr: u64,
//!     virt_kernel_init_addr: u64,
//! ) {
//!     // Enable timer counter registers for EL1.
//!     CNTHCTL_EL2.write(CNTHCTL_EL2::EL1PCEN::SET + CNTHCTL_EL2::EL1PCTEN::SET);
//!
//!     // No offset for reading the counters.
//!     CNTVOFF_EL2.set(0);
//!
//!     // Set EL1 execution state to AArch64.
//!     HCR_EL2.write(HCR_EL2::RW::EL1IsAarch64);
//!
//!     // Set up a simulated exception return.
//!     SPSR_EL2.write(
//!         SPSR_EL2::D::Masked
//!             + SPSR_EL2::A::Masked
//!             + SPSR_EL2::I::Masked
//!             + SPSR_EL2::F::Masked
//!             + SPSR_EL2::M::EL1h,
//!     );
//! }
//! ```
//!
//! ## Disclaimer
//!
//! Descriptive comments in the source files are taken from the [ARM Architecture Reference Manual
//! ARMv8, for ARMv8-A architecture
//! profile](https://static.docs.arm.com/ddi0487/ca/DDI0487C_a_armv8_arm.pdf?_ga=2.266626254.1122218691.1534883460-1326731866.1530967873).

#![cfg_attr(feature = "nightly", feature(core_intrinsics))]
#![cfg_attr(feature = "nightly", feature(custom_inner_attributes))]
#![no_std]

pub mod asm;
#[cfg(feature = "nightly")]
pub mod registers;
