fn main() {
    
	println!("********************");
	println!("Enum Technique 1");
	println!("********************");
	println!();
	let student1 = Student{
		Location:Location::Online,
		Name:String::from("Mustafa")
	};
	
	println!("{:#?}",student1);

	/////////////////////////////
	println!();
	println!("********************");
	println!("Enum Technique 2");
	println!("********************");
	println!();
	
	let ipadressV4 = IPAdresses::v4(String::from("192.0.0.1"));
	let ipadressV6 = IPAdresses::v6(192,168);
	
	println!("{:?}",ipadressV4);
	println!("{:?}",ipadressV6);

	////////////////////////////
	println!();
	println!("********************");
	println!("Enum Example");
	println!("********************");
	println!();
	
	let mesg1 = Message::Quit;
	let mesg2 = Message::Write(String::from("Hello, World!"));
	let mesg3 = Message::Move{x:10,y:12};
	let mesg4 = Message::ChangeColor(123,124,125);
	
	println!("{:?}",mesg1);
	println!("{:?}",mesg2);
	println!("{:#?}",mesg3);
	println!("{:?}",mesg4);
	
	/////////////////////////////////
	println!();
	println!("********************");
	println!("Call By Enum Mehtod");
	println!("********************");
	println!();
	
	mesg1.call();
	mesg2.call();
	mesg3.call();
	mesg4.call();
	
	/////////////////////////////////
	println!();
	println!("********************");
	println!("Enum Call By Function");
	println!("********************");
	println!();
	
	route(mesg1);
	route(mesg2);
	route(mesg3);
	route(mesg4);
	
	/////////////////////////////////
	println!();
	println!("********************");
	println!("Enum And Generic Data");
	println!("********************");
	println!();
	
	let aNumber = AnyThing::any(123);
	let aString = AnyThing::any(String::from("Hi!"));

	println!("{:?}",aNumber);
	println!("{:?}",aString);
	
	/////////////////////////////////
	println!();
	println!("********************");
	println!("Predefine Option enum");
	println!("********************");
	println!();
	
	let anyString = Some("This is Option Enum");
	let noneType: Option<i32> = None;
	
	println!("{:?}",anyString);
	println!("{:?}",noneType);
	
	/////////////////////////////////
	println!();
	println!("********************");
	println!("Difference between normal and Option Data Type");
	println!("********************");
	println!();
	
	let x:i8 = 1;
    let y: Option<i8> = Some(2);
	// let result = x+y  // Error Because Both have Different Data type
	
}

fn m(){

}

// First Technique
#[derive(Debug)]
enum Location{
	Online,
	Onsight
}

#[derive(Debug)]
struct Student{
	Location:Location,
	Name:String
}
////////////////
// Second Technique
#[derive(Debug)]
enum IPAdresses{
	v4(String),
	v6(u32,u32),
}
/////////////////
// Example
#[derive(Debug)]
enum Message{
	Quit,
	Write(String),
	Move{x:u32,y:u32},
	ChangeColor(u32,u32,u32)
}
///////////////////
//Defining Method for Enum
impl Message{
	fn call(&self){
		println!("{:?}",self);
    }
}

///////////////////
//Enum Pass Into the Function
fn route(x: Message){
		println!("{:?}",x);
}

///////////////////
//Enum and Generic Data Type
#[derive(Debug)]
enum AnyThing <T>{
	any(T)
}