extern crate rand;

use std::io;
use rand::Rng;
use std::env;

fn try_again( generated_nbr: u32, max_try_count: u32 ) -> String {
	let mut count: u32 = 0;
	let mut _game_output = String::new();
	
	while count < max_try_count {
		let mut input = String::new();
		io::stdin()
        	.read_line(&mut input)
			.expect(" > [ERREUR] Impossible de lire stdIn");
	
		let trimmed = input.trim();
		match trimmed.parse::<u32>() {
			Ok(i) => {
				if i > generated_nbr { print("Trop haut!".to_string(), 1);  }
				else if i < generated_nbr { print("Trop bas!".to_string(), 1);  }
				else if i == generated_nbr {
					_game_output = String::from("Bravo !");
					break;
				}
			}
			Err(error) => {
				print(error.to_string(), 2);
				break;
			}
		}

		count += 1;
		if count >= max_try_count { _game_output = String::from("Nombre de tentatives possible dépassé."); }
	}
	_game_output
}

fn print( str: String, type_str: u8 ) {
	if type_str == 1  { println!(" > {}", str); }
	else if type_str == 2 { println!(" > [ERREUR] {}", str); }
}

fn main() {
	let args: Vec<String> = env::args().collect();

	let mut min: u32 = 0;
	let mut max: u32 = 100;
	let mut max_try_count: u32 = 10;

	if args.len() >= 3 {
		match args[1].trim().parse::<u32>() {
			Ok(i) => { min = i; }
			Err(error) => { print(error.to_string(), 2); }
		}
		match args[2].trim().parse::<u32>() {
			Ok(i) => { max = i; }
			Err(error) => { print(error.to_string(), 2); }
		}
		match args[3].trim().parse::<u32>() {
			Ok(i) => { max_try_count = i; }
			Err(error) => { print(error.to_string(), 2); }
		}
	}

	let generated_nbr = rand::thread_rng().gen_range(min, max);

	println!("Entre un nombre entre {} et {} :", min, max);

	let result = try_again(generated_nbr, max_try_count);
	print(result, 1);
	println!(" > Le nombre gagnant était : {}.", generated_nbr);
}