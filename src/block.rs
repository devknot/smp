use std::{
	alloc::{Layout, LayoutError, alloc_zeroed, dealloc},
	marker::Sized,
	mem::align_of,
	ops::Drop,
	ptr,
	slice,
	mem,
};

use crate::error::{Result, Error};

pub struct Shield<const BLOCK: usize, Data, Random>
where
	Data: Sized,
	Random: rand::RngCore,
{
	block: *mut Data,
    addr: usize,
    layout: Layout,
    generator: Random,
}

impl <const BLOCK: usize, Data, Random> Shield<BLOCK, Data, Random>
where
	Data: Sized,
	Random: rand::RngCore,
{
	pub fn new(value: Data, mut generator: Random) -> Result<Self> {
		let layout = match Layout::from_size_align(
			BLOCK,
			align_of::<Data>(),
		){
			Ok(value) => value,
			Err(error) => return Err(Error::Layout(error)),
		};
		
		let ptr = unsafe{
			alloc_zeroed(layout)
		};
		
		if ptr.is_null() {
			return Err(Error::Alloc);
		}
		
		let block = ptr as *mut Data;
		
		let addr = generator.next_u64() as usize % BLOCK;
		
		let part = unsafe{
			slice::from_raw_parts_mut(block, BLOCK)
		};
		
		part[addr] = value;
		
		Ok(Self{
			block,
			addr,
			layout,
			generator,
		})
		
	}
	
	fn swap(&mut self) -> Result<usize> {
		
		let new_addr = self.generator.next_u64() as usize % BLOCK;
		
		let part = unsafe{
			slice::from_raw_parts_mut(self.block, BLOCK)
		};
		
		unsafe{
			ptr::copy(
				&part[self.addr] as *const Data,
				&mut part[new_addr] as *mut Data,
				1,
			)
		}
		
		let tmp = self.addr;
		
		self.addr = new_addr;
		
		Ok(tmp)
		
	}
	
	fn clear(&mut self, addr: usize) -> Result<()> {
		if addr >= BLOCK {
			return Err(Error::Address);
		}
		
		let part = unsafe{
			slice::from_raw_parts_mut(self.block, BLOCK)
		};
		
		unsafe {
			std::ptr::write_bytes(
				&mut part[addr] as *mut Data,
				0x0,
				mem::size_of::<Data>(),
			)
		}
		
		Ok(())
	}
	
	pub fn as_ref<'shield>(&self) -> &'shield Data {
		let part = unsafe{
			slice::from_raw_parts_mut(self.block, BLOCK)
		};
		
		&part[self.addr]
		
	}
	
	pub fn as_ref_mut<'shield>(&self) -> &'shield mut Data {
		let part = unsafe{
			slice::from_raw_parts_mut(self.block, BLOCK)
		};
		
		&mut part[self.addr]
	}
	
}

impl <const BLOCK: usize, Data, Random> Drop for Shield<BLOCK, Data, Random>
where
	Data: Sized,
	Random: rand::RngCore,
{
	fn drop(&mut self) {
		unsafe{
			dealloc(self.block as *mut u8, self.layout);
		}
	}
}

