cfg_if::cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        /// The architecture-specific nested page table for two-stage address translation.
        pub type ArchPagingMetatData = arch::ExtendedPageTableMetadata;
        pub type ArchPTE = arch::EPTEntry;
    } else if #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))] {
        /// The architecture-specific page table.
        pub type ArchPagingMetatData = page_table_multiarch::riscv::Sv39MetaData<crate::GuestPhysAddr>;
        pub type ArchPTE = page_table_entry::riscv::Rv64PTE;
    } else if #[cfg(target_arch = "aarch64")]{
        /// The architecture-specific nested page table for two-stage address translation.
        pub type ArchPagingMetatData = arch::A64HVPagingMetaData;
        pub type ArchPTE = arch::A64PTEHV;
    }
}

mod arch;
