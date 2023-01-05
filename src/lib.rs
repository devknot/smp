//const BLOCK: usize = 256;

use std::{
	alloc::{Layout, LayoutError, alloc_zeroed, dealloc},
	marker::Sized,
	mem::align_of,
	ops::Drop,
	ptr,
	slice,
	mem,
};

pub type Result<T> = std::result::Result<T, Error>;

pub struct Shield<const BLOCK: usize, Data, Random>
where
	Data: Sized,
	Random: rand::Rng,
{
	block: *mut Data,
    addr: usize,
    layout: Layout,
    generator: Random,
}

impl <const BLOCK: usize, Data, Random> Shield<BLOCK, Data, Random>
where
	Data: Sized,
	Random: rand::Rng,
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
		
		let addr = generator.gen::<usize>() % BLOCK;
		
		let part = unsafe{
			slice::from_raw_parts_mut(block, BLOCK)
		};
		
		part[addr] = value;
		
		
		//block[addr] = value;
		
		Ok(Self{
			block,
			addr,
			layout,
			generator,
		})
		
	}
	
	fn swap(&mut self) -> Result<usize> {
		
		let new_addr = self.generator.gen::<usize>() % BLOCK;
		
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
	
	fn as_ref<'shield>(&self) -> &'shield Data {
		let part = unsafe{
			slice::from_raw_parts_mut(self.block, BLOCK)
		};
		
		&part[self.addr]
		
	}
	
}

impl <const BLOCK: usize, Data, Random> Drop for Shield<BLOCK, Data, Random>
where
	Data: Sized,
	Random: rand::Rng,
{
	fn drop(&mut self) {
		unsafe{
			dealloc(self.block as *mut u8, self.layout);
		}
	}
}


#[derive(Debug, PartialEq)]
pub enum Error {
	Address,
	Alloc,
	Layout(LayoutError),
}
/*
//impl std::error::Error for Error {}

impl <Data> Shield<Data>
where
	Data: std::marker::Sized + Default + std::marker::Copy,
{
	fn new(addr: usize) -> Self {
		Self{
			block: [Data::default(); BLOCK],
			addr,
		}
	}
	
	fn swap(&mut self, addr: usize) -> Result<usize, Error> {
		
		if addr >= BLOCK {
			return Err(Error::Address);
		}
		
		let dst: *mut Data = &mut self.block[addr] as *mut Data;
		
		unsafe {
			std::ptr::write(dst, self.block[self.addr]);
		}
		
		let tmp = self.addr;
		
		self.addr = addr;
		
		Ok(tmp)
		
	}
	
	fn clear(&mut self, addr: usize) -> Result<(), Error> {
		if addr >= BLOCK {
			return Err(Error::Address);
		}
		
		let dst: *mut Data = &mut self.block[addr] as *mut Data;
		
		unsafe {
			std::ptr::write(dst, Data::default());
		}
		
		Ok(())
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    /*
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    */
    
    #[test]
    fn test_error_addrress() {
    	let mut d: Shield<u8> = Shield::new(0);
    	
    	assert_eq!(d.swap(BLOCK+1), Err(Error::Address));
    	
    }
    
}
*/



