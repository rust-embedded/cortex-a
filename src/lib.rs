// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2021 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! Low level access to Cortex-A processors.
//!
//! ## Currently Supported Architectures
//!
//! - [x] AArch64
//! - [ ] AArch32
//!
//! ## Usage
//!
//! Example from https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials
//!
//! ```rust
//! unsafe fn el2_to_el1_transition() -> ! {
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
//! ```
//!
//! ## Disclaimer
//!
//! Descriptive comments in the source files are taken from the
//! [ARM Architecture Reference Manual ARMv8, for ARMv8-A architecture profile](https://static.docs.arm.com/ddi0487/ca/DDI0487C_a_armv8_arm.pdf?_ga=2.266626254.1122218691.1534883460-1326731866.1530967873).

#![allow(clippy::clippy::upper_case_acronyms)]
#![feature(core_intrinsics)]
#![feature(custom_inner_attributes)]
#![feature(asm)]
#![no_std]

pub mod asm;
pub mod barrier;
pub mod regs;
