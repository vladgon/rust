#![feature(proc_macro_hygiene, decl_macro)]
//#[macro_use]
extern crate rocket;
extern crate sample1;

use sample1::util::ResultOK;
use sample1::web::routs;

fn main() -> ResultOK {
    rocket::ignite()
        .mount("/", routs::get())
        .launch();
    Ok(())
}