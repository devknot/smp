use std::{
	marker::Copy,
	convert::{AsRef, AsMut},
	sync::{Arc, Mutex, LockResult, MutexGuard},
	ops::{Deref, DerefMut},
	cell::RefCell,
	error::Error,
};

use rand::Rng;

use crate::error;


const BLOCK: usize = 256;


pub struct Shield<Data, Random>
where
	Data: Default + Copy,
	Random: Rng,
{
	block: [Data; BLOCK],
	addr: usize,
	generator: Random,
}




impl <D, R> Shield<D, R>
where
	D: Default + Copy,
	R: Rng,
{
	pub fn new(value: D, mut generator: R) -> Self {
		let mut block = [D::default(); BLOCK];
		
		let addr = generator.gen_range(0..BLOCK);
		
		block[addr] = value;
		
		Self{
			block,
			addr,
			generator,
		}
		
	}
	
	fn take(&mut self){
		
		let new_addr = self.generator.gen_range(0..BLOCK);
		
		let tmp = self.block[self.addr];
		
		self.block[new_addr] = tmp;
		
		self.clear(self.addr);
		
		self.addr = new_addr;
	}
	
	fn clear(&mut self, addr: usize) {
		self.block[addr] = D::default();
	}
	
	
	pub fn map<F, U>(&mut self, function: F) -> U
	where
		F: FnOnce(&D) -> U,
	{
		self.take();
		
		function(&self.block[self.addr])
		
	}
	
	/*
	pub fn map_mut<F, U>(&mut self, function: F) -> U
	where
		F: FnOnce(&mut D) -> U,
	{
		self.take();
		
		function(&mut self.block[self.addr])
		
	}
	*/
	/*
	pub fn guard(&mut self) -> Guard<'_, D, R> {
		self.take();
		Guard::new(Arc::new(Mutex::new(self)))
	}
	
	*/
	
}

/*
pub struct Guard<'guard, D, R>
where
	D: Default + Copy,
	R: Rng,
{
	reference: Arc< Mutex <&'guard mut Shield<D, R> > >,
}


impl <'guard, D, R> Guard<'guard, D, R>
where
	D: Default + Copy,
	R: Rng,
{
	fn new(reference: Arc< Mutex <&'guard mut Shield<D, R> > >) -> Self {
		Self{
			reference,
		}
	}
	
	pub fn map<F, U, T>(
		&mut self,
		function: F,
	) -> Result<(), error::Error> //Response
	where
		F: Fn(&D) -> U,
	{
		//function(&self.reference)
		
		let mut guard = match self.reference.lock() {
			Ok(lock) => lock,
			Err(_) => return Err(error::Error::PoisonError),
		};
		
		guard.take();
		
		Ok(())
		//&self.reference.borrow_mut();
	}
	
}
*/

/*
impl <'guard, D, R> Guard<'guard, D, R>
where
	D: Default + Copy,
	R: Rng,
{
	pub fn action<Function, Response>(
		self,
		function: Function,
	) -> ()
	where
		Function: Fn(&D) -> Response,
	{
		//function(&self.reference)
		
		let a = RefCell::into_inner(self.reference);
		//&self.reference.borrow_mut();
	}
	
	fn new(reference: RefCell<&'guard Shield<D, R>>) -> Self {
		Self{
			reference,
		}
	}
	
}
*/
