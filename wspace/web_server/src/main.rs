use serde::Serialize;

mod log;
mod schema;
mod util;


fn main() {
	//    tests!("Test macro {}", "arg1");
	warn!("Test Warn1");
	warn!("Test Warn2");

	let val1 = &mut "val1".to_string();
	let val2 = &mut String::from("val2");
	let mut name = val1;
	let struct_str = Str { name };
	name = val2;
	let struct_str1 = Str { name };
	struct_str1.name.replace_range(.., "xxx");
	// println!("{:?}", val2.as_ptr());
	// println!("{:?}", name.as_ptr());
	// println!("{:?}", val1.as_ptr());
	println!("{:?}", struct_str.name.as_ptr());
	println!("{:?}", struct_str1.name.as_ptr());
	println!("Hello!!!, {:?}", get_ref(struct_str.name));
	println!("Hello!!!, {:?}", struct_str);
	println!("Hello!!!, {:?}", struct_str1);
	schema::schema();
}


#[derive(Debug)]
struct Str<'a> {
	name: &'a mut String,
}

#[allow(dead_code)]
fn get_ref(name: &mut String) -> Str {
// let mut new_str = st.name.to_string();
// new_str.push_str("5".to_string().as_str());
// st.name.push_str("44".to_string().as_mut_str());
// let s = st.name.to_string().add("34");
	Str { name }
}

// #[allow(dead_code)]
// fn get_ref1(_st: Str<'_>) -> Str<'_> { Str { name:  "asd".to_string().as_mut_str() } }

//fn substr(s: &str) -> &str {s}
//fn get_ref(st: &Str<'_>) -> &Str<'_> { Str { name: st.name } }
//fn new<'a>(buf: &'a mut [u8]) -> BufWriter<'a> {
//    BufReader::new(buf)
//}

