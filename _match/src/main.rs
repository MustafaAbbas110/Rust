fn main() {
	
	match_syntex(3);
	let x = match_syntex_Multiline(Coin::Quarter(UsState::Alabama));
	println!("{}",x);
	println!("{:?}",Coin::Quarter(UsState::Alabama));
	

	let val:Option<i32> = plus_one(plus_one(Some(1)));
	println!("{:?}",val);
}

fn match_syntex(x:u32){
	match x {
		1 => println!("Match1"),
		2 => println!("Match2"),
		3 => println!("Match3"),
		_ => println!("Not Match"),
	}
}

fn match_syntex_Multiline(x:Coin) -> i8{
	match x {
		Coin::Penny => {
			println!("Penny");
			1
		},
		Coin::Nickle => {
			println!("Nickle");
			5
		},
		Coin::Dime => {
			println!("Dime");
			10
		},
		Coin::Quarter(state) => {
			println!("Quarter");
			println!("State: {:?}",state);
			25
		}
	}
}

#[derive(Debug)]
enum Coin{
	Penny,
	Nickle,
	Dime,
	Quarter(UsState)
}
#[derive(Debug)]
enum UsState{
	Alabama,
	Alaska
}

fn plus_one(x:Option<i32>) -> Option<i32>{
	match x{
		None => None,
		Some(i) => Some(i+1),
	}
}