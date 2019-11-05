use std::collections::HashMap;

fn main() {
	//HashMap Basic
    let mut x = HashMap::new();
	x.insert(1,"Mustafa Abbas".to_string());
	x.insert(2,"Popeye".to_string());
	
	println!("{:?}",x);
	
	//HashMap Using Vector
	let keys = vec![1,2,3];
	let values = vec![10,20,30];
	
	let map: HashMap<_,_> = keys.iter().zip(values.iter()).collect();
	println!("{:?}",map);
	
	// OwnerShip
	let k = "1".to_string();
	let v = "10".to_string();
	let mut m = HashMap::new();
	m.insert(k,v);
	//println!("{},{}",k,v); Value already borrowed
	
	//Access Value Return from is Option
	let va = m.get(&String::from("1"));
	println!("{:?}",va);
	
	// Looping Thorugh HashMap
	for (Key,Value) in &x{
		println!("{}-{}",Key,Value);
	}
	
	//Stroing Value on basis of previuos values
	x.entry(1).or_insert("Khan".to_string());
	x.entry(3).or_insert("Khan".to_string());
	println!("{:?}",x);
	
	// Updating value on the case of previous value
	let mut count_word = HashMap::new();
	let word = "Mustafa Abbas Mustafa Abbas";
	for i in word.split_whitespace(){
		let count = count_word.entry(i).or_insert(0); //it's return value with a pointer type
		println!("{}",count);
		*count+=1
	}
	println!("{:?}",count_word);

}
