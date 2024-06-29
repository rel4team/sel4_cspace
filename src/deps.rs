//! This module contains interfaces needed to be implemented by external module.

use crate::cap::cap_t;
use crate::structures::finaliseCap_ret;
use sel4_common::structures::exception_t;

extern "C" {
    /// Finalising an architectural cap to make it being the end of link list.
    pub fn Arch_finaliseCap(cap: &cap_t, final_: bool) -> finaliseCap_ret;

    /// Finalising a cap to make it being the end of link list.
    pub fn finaliseCap(cap: &cap_t, _final: bool, _exposed: bool) -> finaliseCap_ret;

    /// if the cap is CapIrqHandlerCap mask the interrupt number.
    pub fn post_cap_deletion(cap: &cap_t);

    /// Add 1 to ksWorkUnitsCompleted, and check whether ksWorkUnitsCompleted exceeds the limitation.
    pub fn preemptionPoint() -> exception_t;
}
