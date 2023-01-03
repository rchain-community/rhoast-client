mod config;
mod error;
mod example;
mod grpc;
mod http;
mod models;
mod proto;
mod utils;

use crate::example::run;
fn main() {
    println!("hello world!");
    run();
    stuff::<String>(add);
}

fn add(num: i32) -> String {
    format!("{num}")
}

fn stuff<T>(func: fn(i32) -> T) {
    func(32);
}
