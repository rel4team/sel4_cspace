use crate::arch::cap_t;
use crate::structures::finaliseCap_ret;
use sel4_common::structures::exception_t;

extern "C" {
    pub fn finaliseCap(cap: &cap_t, _final: bool, _exposed: bool) -> finaliseCap_ret;

    pub fn post_cap_deletion(cap: &cap_t);

    pub fn preemptionPoint() -> exception_t;
}
