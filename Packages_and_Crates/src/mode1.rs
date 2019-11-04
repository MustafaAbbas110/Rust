pub fn detail(){
		println!("Hi! From lib.rs");
		//Absolute Path
		crate::student::matric::result();
		//Realtive Path
		super::student::matric::information();
		
		super::parent::child2::child22(); 
		// Sibling Must be Public
		
		super::at_resturant();
		
		super::access_things();
		
		super::example();
		
		super::check();
	}