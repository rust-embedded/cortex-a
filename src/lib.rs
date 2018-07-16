//! Low level access to Cortex-A processors
//!
//! This crate provides:
//!
//! - Safe wrappers around assembly instructions
//!
//! For now, there's not much. I will update it gradually.
//! If you want to contribute, feel free to reach out!

#![feature(asm)]
#![no_std]

#[macro_use]
extern crate register;

pub mod asm;
pub mod regs;
