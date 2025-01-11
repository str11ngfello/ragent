use crate::tool::Tool;
use serde_json::{json, Value};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AdderError {
    #[error("Uknown error occurred")]
    Unknown,
    #[error("Math error occurred")]
    Math,
}

pub struct AdderArgs {
    pub x: f64,
    pub y: f64,
}

pub struct Adder;

impl Tool for Adder {
    const NAME: &'static str = "Adder";
    const DESCRIPTION: &'static str = "Add 2 floating point numbers";

    type Error = AdderError;
    type Args = AdderArgs;
    type Output = f64;

    fn schema(&self) -> Option<Value> {
        Some(json!({
            "type": "object",
            "properties": {
                "x": {
                    "type": "number",
                    "description": "The first number to add"
                },
                "y": {
                    "type": "number",
                    "description": "The second number to add"
                }
            }
        }))
    }

    fn run(&self, args: Self::Args) -> Result<Self::Output, Box<Self::Error>> {
        if args.x.is_infinite() || args.y.is_infinite() || args.x.is_nan() || args.y.is_nan() {
            return Err(Box::new(AdderError::Math));
        }
        Ok(args.x + args.y)
    }
}
