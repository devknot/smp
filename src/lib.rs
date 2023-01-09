
pub mod error;

pub mod block;

pub use crate::block::Shield;
pub use crate::error::Error;

use rand::{rngs::StdRng, SeedableRng};

const BLOCK: usize = 256;

/// lib
/// sdfjvjsd
/// ''' dec '''
/// ``` fdfg ```

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




