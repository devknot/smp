//const BLOCK: usize = 256;

use std::{
	alloc::{Layout, LayoutError, alloc_zeroed, dealloc},
	marker::Sized,
	mem::align_of,
	ops::Drop,
	slice,
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
		
		let mut part = unsafe{
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
enum Error {
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



