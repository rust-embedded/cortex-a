//! Processor core registers

#[macro_use]
mod macros;

mod cntfrq_el0;
mod cntp_ctl_el0;
mod cntp_cval_el0;
mod cntp_tval_el0;
mod mpidr_el1;
mod sp;

pub use self::cntfrq_el0::CNTFRQ_EL0;
pub use self::cntp_ctl_el0::CNTP_CTL_EL0;
pub use self::cntp_cval_el0::CNTP_CVAL_EL0;
pub use self::cntp_tval_el0::CNTP_TVAL_EL0;
pub use self::mpidr_el1::MPIDR_EL1;
pub use self::sp::SP;
