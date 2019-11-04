pub mod mode1;

// mode Practice

#[allow(dead_code)]
mod student;

// Child, Parent and Sibling Realtionship

pub mod parent;

//Struc -> Public And Private

mod front_of_house{
	// we nee to explicit pub in individual field in struct
	#[derive(Debug)]
	pub struct Breakfast{
		pub toast:String,
		sessional_fruit:String,
	}	
	
	impl Breakfast{
		pub fn new(toast:String) -> Breakfast{
			Breakfast{
				toast,
				sessional_fruit:String::from("apple"),
			}
		}
	}
}

fn at_resturant(){
	let meal = front_of_house::Breakfast::new(String::from("wheat"));
	println!("{:#?}",meal);
}

// enum -> Public and Private

mod fh{
	#[derive(Debug)]
	pub enum things{
		one,
		two
	}
}

fn access_things(){
	let x = fh::things::one;
	println!("{:?}",x);
}

// Use Keywod

mod one{
	pub mod two{
		pub fn three(){
			println!("Print Use Function Example");
		}
	}
}

use crate::one::two;
fn example(){
	two::three();
	two::three();
}

// full path Declate for Sturct,enum and HashMap

pub use std::collections::HashMap;
fn check(){
	let mut x = HashMap::new();
	x.insert("1","Abbas");
	
}