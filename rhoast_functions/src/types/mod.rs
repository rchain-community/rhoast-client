use std::str::FromStr;

use serde::{Deserialize, Serialize};
type ParseError = &'static str;

//cli enum, for handling types

#[derive(Debug, Clone, Copy)]
pub enum Type {
    SimpleDeploy,
}

#[derive(Debug, Clone, Copy)]
pub enum Select {
    HTTP,
    CLI,
}

//http payload types
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimpleDeployPayload {
    pub text: String,
}

//string impl
impl FromStr for Type {
    type Err = ParseError;
    fn from_str(types: &str) -> Result<Self, Self::Err> {
        match types {
            "simple_deploy" => Ok(Type::SimpleDeploy),
            _ => Err("Could not parse utility type"),
        }
    }
}

impl FromStr for Select {
    type Err = ParseError;
    fn from_str(types: &str) -> Result<Self, Self::Err> {
        match types {
            "http" => Ok(Select::HTTP),
            "cli" => Ok(Select::CLI),
            _ => Err("Could not parse utility type"),
        }
    }
}
