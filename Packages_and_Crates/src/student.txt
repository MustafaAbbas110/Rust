pub mod matric{
		pub fn information(){
			println!("Student Information");
		}
		
		pub fn result(){
			println!("Result Information");
			crate::student::check();
		}
	}
	fn check(){
		println!("Public Method Access");
		matric::information();
	}
	mod intermediate{
		fn information(){
			println!("Access Information")
		}
		
		fn result(){
		
		}
	}