# smp

proteção simples de memória ou smp é uma bilbioteca que usa da entropia para esconder a posição da memória de possíveis escaneadores que poderia modificar o valor

usa da biblioteca random para gerar um número entre o 0 até `BLOCK` -1 do `Shield`, o número será usado como um endereço para o valor. Em caso de erro retornará um Result da crate error

uso:

```rust
use smp::block::Shield;

use rand::{rngs::StdRng, SeedableRng};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	
	let mut block = Shield::new(8, StdRng::from_entropy());
	
	block.map(|x| {
		println!("{x}");
	});
	
	Ok(())   
}
```



