use std::any::{Any, TypeId};
use std::collections::HashMap;

pub fn nut_test(){
	struct MethodA;
	struct MethodBWithArguments {
		text:String,
	}

	struct MyObject { counter:u32}
	impl MyObject {
		fn method_a(&mut self, _arg:MethodA){
			self.counter+=1;
			println!("invoke {:?} times without arguments", self.counter);
		}

		fn method_b(&mut self, arg:MethodBWithArguments){
			self.counter +=1;
			println!("invoke {:?} times whith arguments:{:?}", self.counter, arg.text);
		}
	}

	let obj = MyObject{counter:0};
	register_object(obj);
	register_method(MyObject::method_a);
	register_method(MyObject::method_b);

	invoke::<MyObject, _>(MethodA);
	invoke::<MyObject, _>(MethodBWithArguments{text:"hello world".to_owned()});

}




pub struct Nut {
	objects: HashMap<TypeId, Box<dyn Any>>,
	methods: HashMap<(TypeId, TypeId), Box<dyn FnMut(&mut Box<dyn Any>, Box<dyn Any>)>>
}

impl Nut {
	fn register_object<OBJECT>(&mut self, obj: OBJECT)
	where OBJECT:Any,
	{
		let key = TypeId::of::<OBJECT>();
		let boxed_obj = Box::new(obj);
		self.objects.insert(key, boxed_obj);
	}

	pub fn register_method<OBJECT, ARGUMENT, FUNCTION>(&mut self, mut method:FUNCTION)
	where 
		FUNCTION: FnMut(&mut OBJECT, ARGUMENT) + 'static,
		ARGUMENT:Any,
		OBJECT: Any,
	{
		let key = (TypeId::of::<OBJECT>(), TypeId::of::<ARGUMENT>());
		let wrapped_method = 
			Box::new(move |any_obj: &mut Box<dyn Any>, any_args: Box<dyn Any>|{
				let obj: &mut OBJECT = any_obj.downcast_mut().expect("Type conversion failed");
				let args: ARGUMENT = *any_args.downcast().expect("Type conversion failed");
				method(obj, args)
			});
		self.methods.insert(key, wrapped_method);
	}

	pub fn invoke<OBJECT, ARGUMENT>(&mut self, arg:ARGUMENT)
	where
		OBJECT:Any,
		ARGUMENT:Any,
	{
		let object_key = TypeId::of::<OBJECT>();
		let method_key = (TypeId::of::<OBJECT>(), TypeId::of::<ARGUMENT>());
		if let Some(obj) = self.objects.get_mut(&object_key) {
			if let Some(method)= self.methods.get_mut(&method_key) {
				method(obj,Box::new(arg));
			}
		}
	}
}

static mut NUT:Option<Nut> = None;
fn get_nut() -> &'static mut Nut {
	unsafe {
		NUT.get_or_insert_with(|| Nut {
			objects: HashMap::new(),
			methods: HashMap::new(),
		})
	}
}


pub fn register_object(obj: impl Any){
	get_nut().register_object(obj);
}

pub fn register_method<OBJECT, ARGUMENT, FUNCTION>(method: FUNCTION)
where
	FUNCTION: FnMut(&mut OBJECT, ARGUMENT) + 'static,
	ARGUMENT: Any,
	OBJECT: Any,
{
	get_nut().register_method(method);
}

pub fn invoke<OBJECT, ARGUMENT>(method_call: ARGUMENT)
where
	OBJECT: Any,
	ARGUMENT: Any,
{
	get_nut().invoke::<OBJECT, ARGUMENT>(method_call);
}
