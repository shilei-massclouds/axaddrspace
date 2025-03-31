//! [ArceOS-Hypervisor](https://github.com/arceos-hypervisor/) guest VM address space management module.

#![no_std]
#![feature(const_trait_impl)]

#[macro_use]
extern crate log;
extern crate alloc;

mod addr;
pub mod device;
mod frame;
mod hal;
mod npt;

pub use addr::*;
pub use page_table_entry::MappingFlags;

pub use frame::PhysFrame;
pub use hal::AxMmHal;

use npt::{ArchPagingMetatData, ArchPTE};

pub type AddrSpace<H> = aspace_generic::AddrSpace<ArchPagingMetatData, ArchPTE, H>;

/// Information about nested page faults.
#[derive(Debug)]
pub struct NestedPageFaultInfo {
    /// Access type that caused the nested page fault.
    pub access_flags: MappingFlags,
    /// Guest physical address that caused the nested page fault.
    pub fault_guest_paddr: GuestPhysAddr,
}
