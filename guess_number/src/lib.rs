

pub mod generate_number{
	use rand::Rng;
	pub fn secret_number() -> i32 {
		let number = rand::thread_rng().gen_range(1, 11);
		return number;
	}
}

pub mod user_input{
	use std::io;
	pub fn input() -> i32{
		loop{
			let mut user_number = String::new();
				io::stdin().read_line(&mut user_number).expect("Please Enter a Number");
			return match user_number.trim().parse(){
				Ok(num) => num,
				Err(_error) => {
					println!("Please Enter Number");
					continue;
				}
			}	
		}
	}
}

pub mod value_compare{
	pub fn compare(secret_number:i32 , user_number:i32) -> bool{
		if secret_number == user_number {
			println!("You Won!");
			return true;
		}else{
			println!("Try Again");
			return false;
		}
	}
}

pub mod main_logic{
	pub fn start(){
		let secret_number = super::generate_number::secret_number();
		loop{
			let user_number =  super::user_input::input();
			if super::value_compare::compare(secret_number,user_number){
				break;
			}
		}
	}
}