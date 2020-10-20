// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2020 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

// Borrow implementations from the pending upstream ACLE implementation until it is merged.
// Afterwards, we'll probably just reexport them, hoping that the API doesn't change.
//
// https://github.com/rust-lang-nursery/stdsimd/pull/557

mod sealed {
    pub trait Dmb {
        unsafe fn __dmb(&self);
    }

    pub trait Dsb {
        unsafe fn __dsb(&self);
    }

    pub trait Isb {
        unsafe fn __isb(&self);
    }
}

macro_rules! dmb_dsb {
    ($A:ident) => {
        impl sealed::Dmb for $A {
            #[inline(always)]
            unsafe fn __dmb(&self) {
                llvm_asm!(concat!("DMB ", stringify!($A)) : : : "memory" : "volatile")
            }
        }
        impl sealed::Dsb for $A {
            #[inline(always)]
            unsafe fn __dsb(&self) {
                llvm_asm!(concat!("DSB ", stringify!($A)) : : : "memory" : "volatile")
            }
        }
    };
}

pub struct SY;
pub struct ISH;
pub struct ISHST;

dmb_dsb!(ISH);
dmb_dsb!(ISHST);
dmb_dsb!(SY);

impl sealed::Isb for SY {
    #[inline(always)]
    unsafe fn __isb(&self) {
        llvm_asm!("ISB SY" : : : "memory" : "volatile")
    }
}

/// # Safety
///
/// In your own hands, this is hardware land!
#[inline(always)]
pub unsafe fn dmb<A>(arg: A)
where
    A: sealed::Dmb,
{
    arg.__dmb()
}

/// # Safety
///
/// In your own hands, this is hardware land!
#[inline(always)]
pub unsafe fn dsb<A>(arg: A)
where
    A: sealed::Dsb,
{
    arg.__dsb()
}

/// # Safety
///
/// In your own hands, this is hardware land!
#[inline(always)]
pub unsafe fn isb<A>(arg: A)
where
    A: sealed::Isb,
{
    arg.__isb()
}
