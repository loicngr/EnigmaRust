extern crate rand;

use std::io;
use rand::Rng;

fn try_again( generated_nbr: i128, max_try_count: u8 ) -> String {
	let mut count: u8 = 0;
	let mut _game_output = String::new();
	
	loop {
		if count >= max_try_count {
			_game_output = String::from("Nombre de tentatives possible dépassé.");
			break;
		}

		let mut input = String::new();
		io::stdin()
        	.read_line(&mut input)
			.expect(" > [ERREUR] Impossible de lire stdIn");
	
		let trimmed = input.trim();
		match trimmed.parse::<i128>() {
			Ok(i) => {
				if i > generated_nbr { println!(" > Trop haut!");  }
				else if i < generated_nbr { println!(" > Trop bas!");  }
				else if i == generated_nbr { 
					_game_output = String::from("Bravo !");
					break;
				}
			}
			Err(error) => println!(" > [ERREUR] {}", error),
		}

		count += 1;
	}
	_game_output
}

fn main() {
	let debug: bool = false;

	let mut rng = rand::thread_rng();

	let generated_nbr = rng.gen_range(0, 100);
	let max_try_count: u8 = 10;

	if debug == true {
		println!(" > [DEBUG] nombre généré : {}", generated_nbr);
	}
	println!("Entre un nombre entre {} et {} :", 0, 100);

	let result = try_again(generated_nbr, max_try_count);
	println!(" > {}", result);
	println!(" > Le nombre gagnant était : {}.", generated_nbr);
}
