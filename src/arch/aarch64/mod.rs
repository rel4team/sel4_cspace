use sel4_common::{
    arch::maskVMRights, cap_rights::seL4_CapRights_t, plus_define_bitfield,
    structures::exception_t, utils::pageBitsForSize, vm_rights::vm_rights_from_word, MASK,
};

use crate::cte::{cte_t, deriveCap_ret};

/// Cap 在内核态中的种类枚举
#[derive(Eq, PartialEq, Debug)]
pub enum CapTag {
    CapNullCap = 0,
    CapUntypedCap = 2,
    CapEndpointCap = 4,
    CapNotificationCap = 6,
    CapReplyCap = 8,
    CapCNodeCap = 10,
    CapThreadCap = 12,
    CapIrqControlCap = 14,
    CapIrqHandlerCap = 16,
    CapZombieCap = 18,
    CapDomainCap = 20,
    CapFrameCap = 1,
    CapPageTableCap = 3,
    CapPageDirectoryCap = 5,
    CapPageUpperDirectoryCap = 7,
    CapPageGlobalDirectoryCap = 9,
    CapASIDControlCap = 11,
    CapASIDPoolCap = 13,
}

plus_define_bitfield! {
    cap_t, 2, 0, 59, 5 => {
        new_null_cap, CapTag::CapNullCap as usize => {},
        new_untyped_cap, CapTag::CapUntypedCap as usize => {
            capFreeIndex, get_untyped_free_index, set_untyped_free_index, 1, 16, 48, 0, false,
            capIsDevice, get_untyped_is_device, set_untyped_is_device, 1, 6, 1, 0, false,
            capBlockSize, get_untyped_block_size, set_untyped_block_size, 1, 0, 6, 0, false,
            capPtr, get_untyped_ptr, set_untyped_ptr, 0, 0, 48, 0, true
        },
        new_endpoint_cap, CapTag::CapEndpointCap as usize => {
            capEPBadge, get_ep_badge, set_ep_badge, 1, 0, 64, 0, false,
            capCanGrantReply, get_ep_can_grant_reply, set_ep_can_grant_reply, 0, 58, 1, 0, false,
            capCanGrant, get_ep_can_grant, set_ep_can_grant, 0, 57, 1, 0, false,
            capCanSend, get_ep_can_send, set_ep_can_send, 0, 55, 1, 0, false,
            capCanReceive, get_ep_can_receive, set_ep_can_receive, 0, 56, 1, 0, false,
            capEPPtr, get_ep_ptr, set_ep_ptr, 0, 0, 48, 0, true
        },
        new_notification_cap, CapTag::CapNotificationCap as usize => {
            capNtfnBadge, get_nf_badge, set_nf_badge, 1, 0, 64, 0, false,
            capNtfnCanReceive, get_nf_can_receive, set_nf_can_receive, 0, 58, 1, 0, false,
            capNtfnCanSend, get_nf_can_send, set_nf_can_send, 0, 57, 1, 0, false,
            capNtfnPtr, get_nf_ptr, set_nf_ptr, 0, 0, 48, 0, true
        },
        new_reply_cap, CapTag::CapReplyCap as usize => {
            capReplyCanGrant, get_reply_can_grant, set_reply_can_grant, 0, 1, 1, 0, false,
            capReplyMaster, get_reply_master, set_reply_master, 0, 0, 1, 0, false,
            capTCBPtr, get_reply_tcb_ptr, set_reply_tcb_ptr, 1, 0, 64, 0, false
        },
        new_cnode_cap, CapTag::CapCNodeCap as usize => {
            capCNodeRadix, get_cnode_radix, set_cnode_radix, 0, 47, 6, 0, false,
            capCNodeGuardSize, get_cnode_guard_size, set_cnode_guard_size, 0, 53, 6, 0, false,
            capCNodeGuard, get_cnode_guard, set_cnode_guard, 1, 0, 64, 0, false,
            capCNodePtr, get_cnode_ptr, set_cnode_ptr, 0, 0, 47, 1, true
        },
        new_thread_cap, CapTag::CapThreadCap as usize => {
            capTCBPtr, get_tcb_ptr, set_tcb_ptr, 0, 0, 48, 0, true
        },
        new_irq_control_cap, CapTag::CapIrqControlCap as usize => {},
        new_irq_handler_cap, CapTag::CapIrqHandlerCap as usize => {
            capIRQ, get_irq_handler, set_irq_handler, 1, 0, 12, 0, false
        },
        new_zombie_cap, CapTag::CapZombieCap as usize => {
            capZombieID, get_zombie_id, set_zombie_id, 1, 0, 64, 0, false,
            capZombieType, get_zombie_type, set_zombie_type, 0, 0, 7, 0, false
        },
        new_domain_cap, CapTag::CapDomainCap as usize => {},
        new_frame_cap, CapTag::CapFrameCap as usize => {
            capFIsDevice, get_frame_is_device,set_frame_is_device, 0, 6, 1, 0, false,
            capFVMRights,get_frame_vm_rights, set_frame_vm_rights, 0, 7, 2, 0, false,
            capFMappedAddress, get_frame_mapped_address, set_frame_mapped_address, 0, 9, 48, 0, true,
            capFSize, get_frame_size, set_frame_size, 0, 57, 2, 0, false,
            capFMappedASID, get_frame_mapped_asid, set_frame_mapped_asid, 1, 48, 16, 0, false,
            capFBasePtr, get_frame_base_ptr, set_frame_base_ptr, 1, 0, 48, 0, true

        },
        new_page_table_cap, CapTag::CapPageTableCap as usize => {
            capPTMappedASID, get_pt_mapped_asid, set_pt_mapped_asid, 1, 48, 16, 0, false,
            capPTBasePtr, get_pt_base_ptr, set_pt_base_ptr, 1, 0, 48, 0, true,
            capPTIsMapped, get_pt_is_mapped, set_pt_is_mapped, 0, 48, 1, 0, false,
            capPTMappedAddress, get_pt_mapped_address, set_pt_mapped_address, 0, 20, 28, 0, true
        },
        new_page_directory_cap, CapTag::CapPageDirectoryCap as usize => {
            capPDMappedASID, get_pd_mapped_asid, set_pd_mapped_asid, 1, 48, 16, 0, false,
            capPDBasePtr, get_pd_base_ptr, set_pd_base_ptr, 1, 0, 48, 0, true,
            capPDIsMapped, get_pd_is_mapped, set_pd_is_mapped, 0, 48, 1, 0, false,
            capPDMappedAddress, get_pd_mapped_address, set_pd_mapped_address, 0, 29, 19, 0, true
        },
        new_page_upper_directory_cap, CapTag::CapPageUpperDirectoryCap as usize => {
            capPUDMappedASID, get_pud_mapped_asid, set_pud_mapped_asid, 1, 48, 16, 0, false,
            capPUDBasePtr, get_pud_base_ptr, set_pud_base_ptr, 1, 0, 48, 0, true,
            capPUDIsMapped, get_pud_is_mapped, set_pud_is_mapped, 0, 58, 1, 0, false,
            capPUDMappedAddress, get_pud_mapped_address, set_pud_mapped_address, 0, 48, 10, 0, true
        },
        new_page_global_directory_cap, CapTag::CapPageGlobalDirectoryCap as usize => {
            capPGDMappedASID, get_pgd_mapped_asid, set_pgd_mapped_asid, 1, 48, 16, 0, false,
            capPGDBasePtr, get_pgd_base_ptr, set_pgd_base_ptr, 1, 0, 48, 0, true,
            capPGDIsMapped, get_pgd_is_mapped, set_pgd_is_mapped, 0, 58, 1, 0, false
        },
        new_asid_control_cap, CapTag::CapASIDControlCap as usize => {},
        new_asid_pool_cap, CapTag::CapASIDPoolCap as usize => {
            capASIDBase, get_asid_base, set_asid_base, 0, 43, 16, 0, false,
            // FIXED: asid_pool need to shift left 11 bits.
            capASIDPool, get_asid_pool, set_asid_pool, 0, 0, 37, 11, true
        }
    }
}

impl cap_t {
    pub fn get_cap_ptr(&self) -> usize {
        match self.get_cap_type() {
            CapTag::CapUntypedCap => self.get_untyped_ptr(),
            CapTag::CapEndpointCap => self.get_ep_ptr(),
            CapTag::CapNotificationCap => self.get_nf_ptr(),
            CapTag::CapCNodeCap => self.get_cnode_ptr(),
            CapTag::CapThreadCap => self.get_tcb_ptr(),
            CapTag::CapZombieCap => self.get_zombie_ptr(),
            CapTag::CapFrameCap => self.get_frame_base_ptr(),
            CapTag::CapPageTableCap => self.get_pt_base_ptr(),
            CapTag::CapPageDirectoryCap => self.get_pd_base_ptr(),
            CapTag::CapPageUpperDirectoryCap => self.get_pud_base_ptr(),
            CapTag::CapPageGlobalDirectoryCap => self.get_pgd_base_ptr(),
            CapTag::CapASIDControlCap => 0,
            CapTag::CapASIDPoolCap => self.get_asid_pool(),
            _ => 0,
        }
    }

    #[inline]
    pub fn is_vtable_root(&self) -> bool {
        self.get_cap_type() == CapTag::CapPageGlobalDirectoryCap
    }

    #[inline]
    pub fn is_valid_native_root(&self) -> bool {
        self.is_vtable_root() && self.get_pgd_is_mapped() != 0
    }

    #[inline]
    pub fn is_valid_vtable_root(&self) -> bool {
        self.is_valid_native_root()
    }
}

impl cte_t {
    pub fn arch_derive_cap(&mut self, cap: &cap_t) -> deriveCap_ret {
        let mut ret = deriveCap_ret {
            status: exception_t::EXCEPTION_NONE,
            cap: cap_t::default(),
        };
        match cap.get_cap_type() {
            CapTag::CapPageGlobalDirectoryCap => {
                if cap.get_pgd_is_mapped() != 0 {
                    ret.cap = cap.clone();
                    ret.status = exception_t::EXCEPTION_NONE;
                } else {
                    ret.cap = cap_t::new_null_cap();
                    ret.status = exception_t::EXCEPTION_SYSCALL_ERROR;
                }
            }
            CapTag::CapPageUpperDirectoryCap => {
                if cap.get_pud_is_mapped() != 0 {
                    ret.cap = cap.clone();
                    ret.status = exception_t::EXCEPTION_NONE;
                } else {
                    ret.cap = cap_t::new_null_cap();
                    ret.status = exception_t::EXCEPTION_SYSCALL_ERROR;
                }
            }
            CapTag::CapPageDirectoryCap => {
                if cap.get_pud_is_mapped() != 0 {
                    ret.cap = cap.clone();
                    ret.status = exception_t::EXCEPTION_NONE;
                } else {
                    ret.cap = cap_t::new_null_cap();
                    ret.status = exception_t::EXCEPTION_SYSCALL_ERROR;
                }
            }
            CapTag::CapPageTableCap => {
                if cap.get_pud_is_mapped() != 0 {
                    ret.cap = cap.clone();
                    ret.status = exception_t::EXCEPTION_NONE;
                } else {
                    ret.cap = cap_t::new_null_cap();
                    ret.status = exception_t::EXCEPTION_SYSCALL_ERROR;
                }
            }
            CapTag::CapFrameCap => {
                let mut newCap = cap.clone();
                newCap.set_frame_mapped_asid(0);
                ret.cap = newCap;
            }
            CapTag::CapASIDControlCap | CapTag::CapASIDPoolCap => {
                ret.cap = cap.clone();
            }
            _ => {
                panic!(" Invalid arch cap type : {}", cap.get_cap_type() as usize);
            }
        }
        ret
    }
}

pub fn arch_mask_cap_rights(rights: seL4_CapRights_t, cap: &cap_t) -> cap_t {
    if cap.get_cap_type() == CapTag::CapFrameCap {
        let mut vm_rights = vm_rights_from_word(cap.get_frame_vm_rights());
        vm_rights = maskVMRights(vm_rights, rights);
        let mut new_cap = cap.clone();
        new_cap.set_frame_vm_rights(vm_rights as usize);
        return new_cap;
    } else {
        cap.clone()
    }
}

pub fn arch_same_region_as(cap1: &cap_t, cap2: &cap_t) -> bool {
    match cap1.get_cap_type() {
        CapTag::CapFrameCap => {
            if cap2.get_cap_type() == CapTag::CapFrameCap {
                let botA = cap1.get_frame_base_ptr();
                let botB = cap2.get_frame_base_ptr();
                let topA = botA + MASK!(pageBitsForSize(cap1.get_frame_size()));
                let topB = botB + MASK!(pageBitsForSize(cap2.get_frame_size()));
                return (botA <= botB) && (topA >= topB) && (botB <= topB);
            }
        }
        CapTag::CapPageTableCap => {
            if cap2.get_cap_type() == CapTag::CapPageTableCap {
                return cap1.get_pt_base_ptr() == cap2.get_pt_base_ptr();
            }
        }
        CapTag::CapPageDirectoryCap => {
            if cap2.get_cap_type() == CapTag::CapPageDirectoryCap {
                return cap1.get_pd_base_ptr() == cap2.get_pd_base_ptr();
            }
        }
        CapTag::CapPageUpperDirectoryCap => {
            if cap2.get_cap_type() == CapTag::CapPageUpperDirectoryCap {
                return cap1.get_pud_base_ptr() == cap2.get_pud_base_ptr();
            }
        }
        CapTag::CapPageGlobalDirectoryCap => {
            // FIXED: Here should be CapTag::CapPageGlobalDirectoryCap
            if cap2.get_cap_type() == CapTag::CapPageGlobalDirectoryCap {
                return cap1.get_pgd_base_ptr() == cap2.get_pgd_base_ptr();
            }
        }
        CapTag::CapASIDControlCap => {
            return cap2.get_cap_type() == CapTag::CapASIDControlCap;
        }
        CapTag::CapASIDPoolCap => {
            if cap2.get_cap_type() == CapTag::CapASIDPoolCap {
                return cap1.get_asid_pool() == cap2.get_asid_pool();
            }
        }
        _ => panic!("unknown cap"),
    }
    false
}

pub fn arch_same_object_as(cap1: &cap_t, cap2: &cap_t) -> bool {
    if cap1.get_cap_type() == CapTag::CapFrameCap {
        if cap2.get_cap_type() == CapTag::CapFrameCap {
            return cap1.get_frame_base_ptr() == cap2.get_frame_base_ptr()
                && cap1.get_frame_size() == cap2.get_frame_size()
                && cap1.get_frame_is_device() == cap2.get_frame_is_device();
        }
    }
    arch_same_region_as(cap1, cap2)
}
