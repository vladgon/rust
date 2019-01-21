extern crate rand;
#[macro_use]
extern crate serde_derive;


use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use rand::Rng;

use db::model::model as MySQL_MODEL;
use db::mysql::my_sql_service as My_SQL_Service;
use file::inside_file::{*};

mod file;
mod db;
mod config;
mod util;


fn main() -> util::ResultOK {
    let res_dev = config::load(config::Env::DEV);
    println!("Dev {:?}", res_dev);
    let res_prod = config::load(config::Env::PROD);
    println!("Prod {:?}", res_prod);
    let cust = MySQL_MODEL::Customer {
        title: "Title".to_string(),
        last_name: "Last_Name++".to_string(),
        ..Default::default()
    };
    println!("Customer with Default {:#?}", cust);
    println!("Customer Serde JSON {}", serde_json::to_string(&cust)?);
//        {
//        Ok(&res) => res as MySQL_MODEL::Customer,
//        Err(e) => {
//            println!("error parsing header: {:?}", e);
//        }
//};

//    println!("From Config {:#?}", config);

    let _db_customer = My_SQL_Service::connect("vladg1", "123", 3306);

    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Secret Number {}", secret_number);

    let guess = "27";

    let guess = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => -1
    };

    let v1 = vec![1, 2, 3];

    let vector_arc = Arc::new(Mutex::new(v1));
    for i in 0..15 {
        let val = vector_arc.clone();
        thread::spawn(move || {
            let mut mutex_val = val.lock().unwrap();
            println!("Thread {}, {:?}", i, *mutex_val);
            mutex_val.push(i);
        });
    };

    print_number(guess);
    let x = 5;

    print_number(x);

    file::open_file();
    xx();
    file::inside::inside_file1::yy();

//    println!("Guess {}", guess);

    let v1 = vec![1, 2, 3];
    let res;

    {
        let v = vec![1, 2, 3];
        res = take(&v, &v1);
    };
    println!("{:?}", res);

    let y = vec![1, 2, 3];
    let f;

    {
        f = Foo { x: &y };
    };
    println!("{:?}", f.x());

    thread::sleep(Duration::from_secs(1));
    Ok(())
}

fn print_number(x: i32) {
    println!("x is: {}", x);
}


#[allow(unused_variables)]
fn take<'a>(x: &Vec<i32>, y: &'a Vec<i32>) -> &'a Vec<i32> {
//    let mut x1 = x;

//    x.push(1);
    &y
}


struct Foo<'a> {
    x: &'a Vec<i32>,
}

impl<'a> Foo<'a> {
    fn x(&self) -> &'a Vec<i32> { self.x }
}