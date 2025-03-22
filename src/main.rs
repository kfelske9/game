
use std::io::{self, BufRead};
use rand::Rng;

fn main() {

	let stdin 		= io::stdin();
	let mut handle 	= stdin.lock();
	let trial 		= 5;
	
    println!("Guess 3 numbers between 1 and 9!\n");
    
    println!("{} trials, strike = hit right place, ball = hit wrong place\n", trial);
    let mut secret_array : [i32; 3] = [0; 3];

    secret_array [0] = rand::thread_rng().gen_range(1..=9);
    
    loop {
		secret_array [1] = rand::thread_rng().gen_range(1..=9);
		if secret_array [1] != secret_array [0] {
			break;
		}
	}
	
	loop {
		secret_array [2] = rand::thread_rng().gen_range(1..=9);
		if secret_array [2] != secret_array [0] && secret_array [2] != secret_array [1] {
			break;
		}
	}
    
    let mut i = 0;
    let mut result;
    
    while i < trial {		
		
		let mut buffer = String::new();
		let _ = handle.read_line(&mut buffer);
			
		result = parse(buffer.clone(), secret_array.to_vec());
		
		if result / 10 == 3 {
			println!("Gratulation!\nYou won with {} atempts!", i + 1);
			break;
			
		} else if result < 0 {
			i += 1;
			println!("Only 3 numbers between 1 and 9!\n");
			
		} else {
			i += 1;
			println!("Strike:	{}", result / 10);
			println!("Ball:	{}\n", result % 10);
		}
	}
	if i == trial {
		println!("\nSorry, you lost!");
	}

	println!("\nThe number is: {}{}{}", secret_array [0], secret_array [1], secret_array [2]);
}

fn parse (letters: String, secret_array: Vec<i32>) -> i32 {
	
	if letters.len() > 4 			{ return -1 }
	
	let mut strike 	= 0;
	let mut ball 	= 0;
	let mut i 		= 0;
	
	for n in secret_array {
		
		let place = letters.find(&n.to_string());
				
		match place {
				Some(0) => { 	if i == 0 {
									strike = strike + 1; }
								else {
									ball = ball + 1; }
							}
				
				Some(1) => { 	if i == 1 {
									strike = strike + 1; }
								else {
									ball = ball + 1; }
							}
				
				Some(2) => { 	if i == 2 {
									strike = strike + 1; }
								else {
									ball = ball + 1; }
							} 
							      
				None    => { }
        
				_       => { return -1 }
		}
		
		i = i + 1;
	}
	
	strike * 10 + ball

}
