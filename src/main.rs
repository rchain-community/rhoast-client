use rust_client::example::run;
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
