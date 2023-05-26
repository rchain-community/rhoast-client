mod eval;

use super::types::{Select, Type};
use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "Rhoast tools", about = "Rhoast tools")]
pub struct Args {
    //select app type
    #[structopt(short = "s", long = "select")]
    pub select: Select,

    //rhoast tool utility to call
    #[structopt(short = "u", long = "utility", required_if("select", "cli"))]
    pub utility: Option<Type>,

    //simple deploy payload
    #[structopt(long = "simpledeploy")]
    pub simpledeploy: Option<String>,
}
