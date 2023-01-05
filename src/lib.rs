#![feature(allocator_api)]

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
/*
pub struct Shield<Data, Heap = std::alloc::Global>
where
	Data: std::marker::Sized,
	Heap: std::alloc::Allocator,
{
    ptr: *mut Data,
    alloc: Heap,
    addr: usize,
}

impl Shield {
	fn new() {
		std::mem::size_of::<Data>()
	}
}
*/

const BLOCK: usize = 256;

pub struct Shield<Data>
where
	Data: std::marker::Sized + std::marker::Copy + Default,
{
	block: [Data; BLOCK],
    addr: usize,
}

enum Error {
	Address,
}

impl <Data> Shield<Data>
where
	Data: std::marker::Sized + std::marker::Copy + Default,
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

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
