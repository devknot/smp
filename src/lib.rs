//const BLOCK: usize = 256;

use std::{
	alloc::{Layout, alloc_zeroed, dealloc},
	marker::Sized,
	mem::align_of,
	ops::Drop,
};

pub struct Shield<const BLOCK: usize, Data>
where
	Data: Sized,
{
	block: *mut Data,
    addr: usize,
    layout: Layout,
}

impl <const BLOCK: usize, Data> Shield<BLOCK, Data>
where
	Data: Sized,
{
	pub fn new() -> Self {
		let layout = Layout::from_size_align(
			BLOCK,
			align_of::<Data>(),
		).unwrap();
		
		let ptr = unsafe{
			alloc_zeroed(layout)
		};
		
		let block = ptr as *mut Data;
		let addr = 0;
		
		Self{
			block,
			addr,
			layout,
		}
		
	}
}

impl <const BLOCK: usize, Data> Drop for Shield<BLOCK, Data>
where
	Data: Sized,
{
	fn drop(&mut self) {
		unsafe{
			dealloc(self.block as *mut u8, self.layout);
		}
	}
}

/*
#[derive(Debug, PartialEq)]
enum Error {
	Address,
}

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



