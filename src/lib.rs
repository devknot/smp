
pub mod error;

pub mod block;

use crate::block::Shield;

use rand::{rngs::StdRng, SeedableRng};

const BLOCK: usize = 256;


#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_memory() {
		let block: Shield<BLOCK, u8, StdRng> = Shield::new(8, StdRng::from_entropy()).unwrap();
		
		{
			let r = block.as_ref_mut();
			
			*r += 2;
		}
		
		assert_eq!(*block.as_ref(), 10);
	}    
}




