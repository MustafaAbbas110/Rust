fn main() {

	// Devalring String
    let x1 = String::new();
	
	// Initializing
	let x2 = String::from("Name");
	
	// Convert str to String
	let x3 = "Abbas";
	let mut x4 = x3.to_string();
	
	// Push into String
	let x4 = x4.push_str(" Khan");
	
	// By using push we just add one charactr into string
	let mut a = "Abbas".to_string();
	let mut a = a.push('k');
	
	// Concatenation
	let a1 = "Abbas".to_string();
	let a2 = " Mustafa".to_string();
	let r = a1.clone()+&a2; // If I Don't Use & owner transfer in to add function()
	println!("{}",&r);
	let y = format!("{}-{}",a1,a2);
	println!("{}",y);
	
	// indexing in str
	let name = "Mustafa";
	println!("{}",&name[0..1]);
	
	//looping through slicing
	for i in name.chars(){
		println!("{}",i);
	}
	
	//looping through bytes
	for i in name.bytes(){
		println!("{}",i);
	}
}
