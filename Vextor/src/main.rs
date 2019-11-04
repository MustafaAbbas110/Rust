fn main() {
    // initilizing Variable
	let mut x = vec![1,2,3];
	println!("{:?}",x);
	
	// Vector Declaration
	let mut y:Vec<i32> = Vec::new();
	
	// Add value in Vector
	y.push(1);
	
	println!("{:?}",y);
	
	// Remove Value
	let z = x.pop();
	println!("{:?} , {:?}",x,z);
	
	//Vector and Scope
	{
		let f = vec![2,3];
	}
	// Out of Scope
	//println!("{:?}",f);
	
	//access Value from Vector
	let u = x.get(1);
	println!("{:?}",u); // Option enum Return
	
	// Second Technique
	let second = x[1];
	println!("{}",second);
	
	let mut avoid = vec![1,2,3];
	
	// race Condtion
	//let reference = &avoid[0];
	//avoid.push(4);
	//println!("{}",reference);

	// Looping thorugh in Vactor
	for i in &mut avoid{
		*i+=10;
		println!("{}",i);
	}
	
	let hetero = vec![dataType::Int(10),dataType::Float(10.1),dataType::Text(String::from("Abbas"))];
	#[derive(Debug)]
	enum dataType{
		Int(i32),
		Float(f64),
		Text(String)
	}
	
	for x in 0..3{
		println!("{:?}",hetero[x]);
	}
	
}

