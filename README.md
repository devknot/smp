# smp

proteção simples de memória ou smp é uma bilbioteca que usa da entropia para esconder a posição da memória de possíveis escaneadores que poderia modificar o valor

usa da biblioteca random para gerar um número entre o 0 até `BLOCK` -1 do `Shield`, o número será usado como um endereço para o valor. Em caso de erro retornará um Result da crate error

uso:

```
use smp::block::Shield;

use rand::{rngs::StdRng, SeedableRng};

const BLOCK: usize = 256;

fn main() -> Result<(), Box<dyn std::error::Error>> {

	let block: Shield<BLOCK, u8, StdRng> = 
	Shield::new(8, StdRng::from_entropy()).unwrap();
	
	println!("{}", block.as_ref());
	
	Ok(())   
}
```



