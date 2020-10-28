// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2020 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! Low level access to Cortex-A processors
//!
//! This crate provides:
//!
//! - Safe wrappers around assembly instructions
//!
//! For now, there's not much. I will update it gradually.
//! If you want to contribute, feel free to reach out!
//!
//! # Minimum Supported Rust Version (MSRV)
//!
//! This crate is guaranteed to compile on stable Rust 1.39 and up. It *might* compile with older
//! versions but that may change in any new patch release.

#![feature(core_intrinsics)]
#![feature(custom_inner_attributes)]
#![feature(asm)]
#![no_std]

pub mod asm;
pub mod barrier;
pub mod regs;
