pub fn parent_fucntion(){
		println!("Parent Function");
		//child1::child1_fucntion();
		//child1 Must be Public Because it's Child property
	}
	
	mod child1;
	
	pub mod child2{
		fn child2_fucntion(){
			println!("child2 Function");
			super::child1::child1_fucntion();
			//Dont't need child1 pub (Siblings)
		}
		
		pub fn child22(){
			println!("child22 Function");
			child2_fucntion();
		}
	}