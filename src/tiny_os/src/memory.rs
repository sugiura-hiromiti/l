use bootloader::bootinfo::MemoryMap;
use bootloader::bootinfo::MemoryRegionType;
use x86_64::PhysAddr;
use x86_64::VirtAddr;
use x86_64::structures::paging::FrameAllocator;
use x86_64::structures::paging::OffsetPageTable;
use x86_64::structures::paging::PageTable;
use x86_64::structures::paging::PhysFrame;
use x86_64::structures::paging::Size4KiB;

///Return usable frame from memory map of bootloader
pub struct BootInfoFrameAllocator {
	memory_map: &'static MemoryMap,
	next:       usize,
}

impl BootInfoFrameAllocator {
	/// - **Create FrameAllocator from given `memory_map`**
	///
	///# unsafe
	///
	///caller have to guarantees given `memory_map` is valid. Especially `USABLE` frame must be
	///actually unused.
	pub unsafe fn init(memory_map: &'static MemoryMap,) -> Self {
		BootInfoFrameAllocator { memory_map, next: 0, }
	}

	fn usable_frames(&self,) -> impl Iterator<Item = PhysFrame,> {
		//get usable region from memory_map
		let regions = self.memory_map.iter();
		let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable,);
		//translate each ranges to address range
		let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr(),);
		//translate iterator of start address of frame
		let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096,),);
		//create `PhysFrame` type from `frame_addresses`
		frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr,),),)
	}
}

unsafe impl FrameAllocator<Size4KiB,> for BootInfoFrameAllocator {
	fn allocate_frame(&mut self,) -> Option<PhysFrame<Size4KiB,>,> {
		let frame = self.usable_frames().nth(self.next,);
		self.next += 1;
		frame
	}
}

///Return valid mutable reference to the level 4 table
///---
///
/// # unsafe
///
/// This `fn` is `unsafe` because caller of this `fn` have to guarantees all physical memory is
/// offsetted by `physical_memory_offset` which is given.
///
/// # Attention
///
/// This `fn` must be called **only once**.  
/// Because calling this `fn` may cause `&mut` reference have several name.
///
/// >this is called *mutable aliasing* which is undefined.
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr,) -> &'static mut PageTable {
	use x86_64::registers::control::Cr3;

	//read valid level 4 table frame from CR3 register
	let (level_4_table_frame, _,) = Cr3::read();
	let phys = level_4_table_frame.start_address();
	let virt = physical_memory_offset + phys.as_u64();
	let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

	//unsafe
	//Rust treat unsafe fn itself as unsafe block
	&mut *page_table_ptr
}

///Initialize new `OffsetPageTable`.
///---
///
///# unsafe
///
/// This `fn` is `unsafe` because caller of this `fn` have to guarantees all physical memory is
/// offsetted by `physical_memory_offset` which is given.
///
/// # Attention
///
/// This `fn` must be called **only once**.  
/// Because calling this `fn` may cause `&mut` reference have several name.
///
/// >this is called *mutable aliasing* which is undefined.
pub unsafe fn init(physical_memory_offset: VirtAddr,) -> OffsetPageTable<'static,> {
	let level_4_table = active_level_4_table(physical_memory_offset,);
	OffsetPageTable::new(level_4_table, physical_memory_offset,)
}
