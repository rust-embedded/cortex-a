macro_rules! __read_raw {
    ($width:ty, $asm_instr:tt, $asm_reg_name:tt) => {
        /// Reads the raw bits of the CPU register.
        #[inline]
        pub fn read_raw() -> $width {
            match () {
                #[cfg(target_arch = "aarch64")]
                () => {
                    let reg;
                    unsafe {
                        asm!(concat!($asm_instr, " $0, ", $asm_reg_name) : "=r"(reg) ::: "volatile");
                    }
                    reg
                }

                #[cfg(not(target_arch = "aarch64"))]
                () => unimplemented!(),
            }
        }
    };
}

macro_rules! __write_raw {
    ($width:ty, $asm_instr:tt, $asm_reg_name:tt) => {
        /// Writes raw bits to the CPU register.
        #[cfg_attr(not(target_arch = "aarch64"), allow(unused_variables))]
        #[inline]
        pub unsafe fn write_raw(reg: $width) {
            match () {
                #[cfg(target_arch = "aarch64")]
                () => {
                    asm!(concat!($asm_instr, " ", $asm_reg_name, ", $0") :: "r"(reg) :: "volatile")
                }

                #[cfg(not(target_arch = "aarch64"))]
                () => unimplemented!(),
            }
        }
    };
}

/// Raw read from system coprocessor registers
macro_rules! sys_coproc_read_raw {
    ($width:ty, $asm_reg_name:tt) => {
        __read_raw!($width, "mrs", $asm_reg_name);
    };
}

/// Raw write to system coprocessor registers
macro_rules! sys_coproc_write_raw {
    ($width:ty, $asm_reg_name:tt) => {
        __write_raw!($width, "msr", $asm_reg_name);
    };
}

/// Raw read from (ordinary) registers
macro_rules! read_raw {
    ($width:ty, $asm_reg_name:tt) => {
        __read_raw!($width, "mov", $asm_reg_name);
    };
}
/// Raw write to (ordinary) registers
macro_rules! write_raw {
    ($width:ty, $asm_reg_name:tt) => {
        __write_raw!($width, "mov", $asm_reg_name);
    };
}

macro_rules! modify_raw {
    () => {
        /// Modifies register using raw bits.
        #[inline]
        pub unsafe fn modify_raw<F>(f: F)
        where
            F: FnOnce(&mut u64),
        {
            let mut reg = Self::read_raw();
            f(&mut reg);
            Self::write_raw(reg);
        }
    };
}

macro_rules! read_flags {
    () => {
        /// Reads the current set of the register's flags.
        #[inline]
        pub fn read_flags() -> Self {
            Self::from_bits_truncate(Self::read_raw())
        }
    };
}

macro_rules! write_flags {
    () => {
        /// Write the flags to the CPU register.
        #[inline]
        pub unsafe fn write_flags(&self) {
            Self::write_raw(self.bits)
        }
    }
}

macro_rules! modify_flags {
    () => {
        /// Modifies register using flags-functions.
        #[inline]
        pub unsafe fn modify_flags<F>(f: F)
        where
            F: FnOnce(&mut Self),
        {
            let mut reg = Self::read_flags();
            f(&mut reg);
            reg.write_flags();
        }
    };
}
