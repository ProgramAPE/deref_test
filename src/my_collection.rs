use core::any::Any;
use std::collections::HashMap;

pub fn my_collection(){
	let mut collection = HeteroCollection::default();
	collection.set("f32", 3.14f32);
	collection.set("f64", 2.4156f64);
	collection.set("another f32", 1.608f32);

	let f32_output = *collection.get::<f32>("f32").unwrap();
	assert_eq!(3.14, f32_output);

}


#[derive(Debug, Default)]
struct HeteroCollection {
	data: HashMap<&'static str, Box<dyn Any>>,
}

impl HeteroCollection {
	fn get<T: 'static>(&self, key: &'static str) -> Option<&T> {
		let unknown_output: &Box<dyn Any> = self.data.get(key)?;
		unknown_output.downcast_ref()
	}

	fn set<T: 'static>(&mut self, key:&'static str, value:T){
		self.data.insert(key, Box::new(value));
	}
}