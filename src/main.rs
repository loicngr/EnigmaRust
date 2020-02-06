extern crate rand;

use std::io;
use rand::Rng;
use std::env;

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
			Err(error) => {
				println!(" > [ERREUR] {}", error);
				break;
			}
		}

		count += 1;
	}
	_game_output
}

fn main() {
	let args: Vec<String> = env::args().collect();

	let mut min: i128 = 0;
	let mut max: i128 = 100;

	let len = args.len();
	if len >= 3 {
		match args[1].trim().parse::<i128>() {
			Ok(i) => { min = i; }
			Err(error) => { println!(" > [ERREUR] {}", error); }
		}
		match args[2].trim().parse::<i128>() {
			Ok(i) => { max = i; }
			Err(error) => { println!(" > [ERREUR] {}", error); }
		}
	}

	let generated_nbr = rand::thread_rng().gen_range(min, max);

	println!("Entre un nombre entre {} et {} :", min, max);

	let result = try_again(generated_nbr, 10u8);
	println!(" > {}", result);
	println!(" > Le nombre gagnant était : {}.", generated_nbr);
}
