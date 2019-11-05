use std::fs::File;
use std::io::ErrorKind;

fn main() {
    
	
	let f = File::open("a.txt");
	
	let f = match f{
		Ok(done) => done,
		Err(error) => match error.kind(){
						ErrorKind::NotFound => match File::create("a.txt"){
							Ok(o) => o,
							Err(e) => panic!("Check {:#?}",e)
						}
						_ => panic!("Error")
					}
	};
	
	//shorthand for result enum
	//let g = File::open("Hello.txt").unwrap();
	
	//Meaningfull Error By using Expect
	let g = File::open("Hello.txt").expect("File Not Found!");
}
