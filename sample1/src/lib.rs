#![feature(proc_macro_hygiene, decl_macro)]
extern crate rand;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

pub mod db;
pub mod config;
pub mod util;
pub mod web;