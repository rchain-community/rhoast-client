use crate::rholang::parse_simple_deploy;
use structopt::StructOpt;

use super::Args;

impl Args {
    pub fn new() -> Self {
        Args::from_args()
    }

    pub fn parse_simple_deploy(self) {
        if self.simpledeploy.is_none() {
            panic!("please supply a simple delpoy payload")
        }
        match parse_simple_deploy(&self.simpledeploy.unwrap()) {
            Ok(text) => {
                println!("{text}")
            }
            Err(e) => {
                println!("{:?}", e)
            }
        }
    }
}
