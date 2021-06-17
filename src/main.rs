use std::collections::HashMap;

mod nut;
mod my_collection;
mod singletone_collection;

fn main() {
	// let mut a = Name1 {name:"haha".into()};
	// let b = Box::new(a);
	// //println!("{:?}", a);
 //    println!("Hello, world!");
 //    //assert_eq!(a, *b);
 //    println!("{:?}", *b);
 //    println!("{:?}", b);
 //    include!("./test.txt");
 //    println!("{:?}", a);
	my_collection::my_collection();	
	singletone_collection::singleton_collection_test();


	// let a = Name1 { name:"fff".into()};
	// let b = Name2 { age:9};

	// let mut map:HashMap<usize, Box<dyn Name>> = HashMap::new();

	// map.insert(1, Box::new(a));
	// map.insert(2, Box::new(b));
	// let d = map.get(&1).unwrap();
	
	// let c = *d.downcast::<Name1>();

	nut::nut_test();

}


#[derive(Debug, std::cmp::PartialEq)]
struct Name1 {
	name: String
}
impl Name for Name1 {
	// add code here
	fn print(&self){
		println!("name {:?}", self.name);
	}
}

#[derive(Debug)]
struct Name2 {
	age: usize
}
impl Name for Name2 {
	// add code here
	fn print(&self){
		println!("aget {:?}", self.age);
	}
}

trait Name {
	// add code here
	fn print(&self);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
	// add code here
	fn new(x:T) -> MyBox<T> {
		MyBox(x)
	}
}
