/*
 * Copyright (c) 2018-2019 by the author(s)
 *
 * =============================================================================
 *
 * Licensed under either of
 *   - Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
 *   - MIT License (http://opensource.org/licenses/MIT)
 * at your option.
 *
 * =============================================================================
 *
 * Author(s):
 *   - Andre Richter <andre.o.richter@gmail.com>
 *   - Alban Seurat <alban.seurat@me.com>
 */

//! The link register

use register::cpu::RegisterReadWrite;

pub struct Reg;

impl RegisterReadWrite<u64, ()> for Reg {
    read_raw!(u64, "lr");
    write_raw!(u64, "lr");
}

pub static LR: Reg = Reg {};
