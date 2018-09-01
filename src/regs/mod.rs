//! Processor core registers

#[macro_use]
mod macros;

mod cntfrq_el0;
mod cnthctl_el2;
mod cntp_ctl_el0;
mod cntp_tval_el0;
mod cntvoff_el2;
mod currentel;
mod daif;
mod elr_el2;
mod hcr_el2;
mod mpidr_el1;
mod sp;
mod sp_el0;
mod sp_el1;
mod spsel;
mod spsr_el2;

// Export only the R/W traits and the static reg definitions
pub use register::cpu::*;

pub use self::cntfrq_el0::CNTFRQ_EL0;
pub use self::cnthctl_el2::CNTHCTL_EL2;
pub use self::cntp_ctl_el0::CNTP_CTL_EL0;
pub use self::cntp_tval_el0::CNTP_TVAL_EL0;
pub use self::cntvoff_el2::CNTVOFF_EL2;
pub use self::currentel::CurrentEL;
pub use self::daif::DAIF;
pub use self::elr_el2::ELR_EL2;
pub use self::hcr_el2::HCR_EL2;
pub use self::mpidr_el1::MPIDR_EL1;
pub use self::sp::SP;
pub use self::sp_el0::SP_EL0;
pub use self::sp_el1::SP_EL1;
pub use self::spsel::SPSel;
pub use self::spsr_el2::SPSR_EL2;
