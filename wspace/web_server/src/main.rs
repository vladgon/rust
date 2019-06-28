mod log;

fn main() {
//    test!("Test macro {}", "arg1");
    warn!("Test Warn1");
    warn!("Test Warn2");
//    println!("Hello!!!, {:?}", get_ref(&Str { name: "test" }));
}


#[derive(Debug)]
struct Str<'a> {
    name: &'a str,
}

fn get_ref<'a>(st: &'a Str) -> &'a Str<'a> { st }

fn get_ref1(_st: Str) -> Str<'_> { Str { name: "asd" } }

//fn substr(s: &str) -> &str {s}
//fn get_ref(st: &Str<'_>) -> &Str<'_> { Str { name: st.name } }
//fn new<'a>(buf: &'a mut [u8]) -> BufWriter<'a> {
//    BufReader::new(buf)
//}

