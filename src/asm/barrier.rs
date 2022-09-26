// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2022 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! Barrier functions.

mod sealed {
    pub trait Dmb {
        fn __dmb(&self);
    }

    pub trait Dsb {
        fn __dsb(&self);
    }

    pub trait Isb {
        fn __isb(&self);
    }
}

macro_rules! dmb_dsb {
    ($A:ident) => {
        impl sealed::Dmb for $A {
            #[inline(always)]
            fn __dmb(&self) {
                match () {
                    #[cfg(target_arch = "aarch64")]
                    () => {
                        unsafe {
                            core::arch::asm!(concat!("DMB ", stringify!($A)), options(nostack))
                        }
                    }

                    #[cfg(not(target_arch = "aarch64"))]
                    () => unimplemented!(),
                }
            }
        }
        impl sealed::Dsb for $A {
            #[inline(always)]
            fn __dsb(&self) {
                match () {
                    #[cfg(target_arch = "aarch64")]
                    () => {
                        unsafe {
                            core::arch::asm!(concat!("DSB ", stringify!($A)), options(nostack))
                        }
                    }

                    #[cfg(not(target_arch = "aarch64"))]
                    () => unimplemented!(),
                }
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
    fn __isb(&self) {
        match () {
            #[cfg(target_arch = "aarch64")]
            () => {
                unsafe {
                    core::arch::asm!("ISB SY", options(nostack))
                }
            }

            #[cfg(not(target_arch = "aarch64"))]
            () => unimplemented!(),
        }
    }
}

#[inline(always)]
pub fn dmb<A>(arg: A)
where
    A: sealed::Dmb,
{
    arg.__dmb()
}

#[inline(always)]
pub fn dsb<A>(arg: A)
where
    A: sealed::Dsb,
{
    arg.__dsb()
}

#[inline(always)]
pub fn isb<A>(arg: A)
where
    A: sealed::Isb,
{
    arg.__isb()
}
