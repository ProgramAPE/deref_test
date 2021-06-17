use core::any::*;
use std::collections::HashMap;

pub fn singleton_collection_test(){
	let mut a = SingletonCollection {
		data:HashMap::new()
	};

	let b = Name1 { age:99};
	let c = Name2 { name:"haha".into()};
	a.set(b);
	a.set(c);
	println!("{:?}", a.get::<Name2>());
}

struct SingletonCollection {
	data: HashMap<TypeId, Box<dyn Any>>
}

impl SingletonCollection {
	pub fn get<T:Any> (&self) -> &T{
		self.data[&TypeId::of::<T>()]
			.downcast_ref()
			.as_ref()
			.unwrap()
	}

	pub fn set<T:Any>(&mut self, value:T) {
		self.data.insert(TypeId::of::<T>(), Box::new(value));
	}
}

#[derive(Debug)]
struct Name1 {
	age: usize
}

#[derive(Debug)]
struct Name2 {
	name: String
}