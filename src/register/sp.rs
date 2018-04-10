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

//! The stack pointer

/// Reads the CPU register
#[inline]
pub fn read() -> u64 {
    match () {
        #[cfg(target_arch = "aarch64")]
        () => {
            let r;
            unsafe { asm!("mov $0, sp" : "=r"(r) ::: "volatile") }
            r
        }

        #[cfg(not(target_arch = "aarch64"))]
        () => unimplemented!(),
    }
}

/// Writes `bits` to the CPU register
#[cfg_attr(not(target_arch = "aarch64"), allow(unused_variables))]
#[inline]
pub unsafe fn write(bits: u64) {
    match () {
        #[cfg(target_arch = "aarch64")]
        () => asm!("mov sp, $0" :: "r"(bits) :: "volatile"),

        #[cfg(not(target_arch = "aarch64"))]
        () => unimplemented!(),
    }
}
