use std::path::PathBuf;
use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {

    /// Activate debug mode
    #[structopt(short = "L", long)]
    log: u8,

    /// private key
    #[structopt(long)]
    pk: String,

    /// private key file
    #[structopt(long)]
    pkfile: parse(from_os_str),

    /// read/write (validator) node
    #[structopt(short, long, parse(from_http_str))]
    validator: String,

    /// read-only (observer) node
    #[structopt(short, long, parse(from_http_str))]
    observer: String,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:#?}", opt);
}
