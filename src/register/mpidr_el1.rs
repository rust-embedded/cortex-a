/*
 * MIT License
 *
 * Copyright (c) 2018 Jorge Aparicio
 * Copyright (c) 2018 Andre Richter <andre.o.richter@gmail.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */


//! Multiprocessor Affinity Register - EL1
//!
//! The processor and cluster IDs, in multi-core or cluster systems.

/// MPIDR_EL1
#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub struct MPIDR_EL1 {
    bits: u64,
}

impl MPIDR_EL1 {
    /// Returns the contents of the register as raw bits
    pub fn bits(&self) -> u64 {
        self.bits
    }

    /// Affinity level 0. Lowest level affinity field.
    #[inline]
    pub fn aff0(&self) -> u8 {
        (self.bits & 0xff) as u8
    }

    /// The core number
    #[inline]
    pub fn core_id(&self) -> u8 {
        self.aff0() & 0x3
    }
}

/// Reads the CPU register
#[inline]
pub fn read() -> MPIDR_EL1 {
    match () {
        #[cfg(target_arch = "aarch64")]
        () => {
            let r: u64;
            unsafe { asm!("mrs $0, MPIDR_EL1" : "=r"(r) ::: "volatile") }
            MPIDR_EL1 { bits: r }
        }

        #[cfg(not(target_arch = "aarch64"))]
        () => unimplemented!(),
    }
}
