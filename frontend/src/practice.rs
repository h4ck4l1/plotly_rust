#![allow(unused)]

use itertools::Itertools;
use serde::Serialize;
use serde_json::{to_string_pretty, to_value, Value};
use web_sys::js_sys::Math;

#[async_std::main]
async fn main() -> Result<(),anyhow::Error> {


    let fourth_struct = FourthStruct {sixth: "sixth".to_string(), seventh: "seventh".to_string()};
    let first_sturct = FirstStruct {first: "first".to_string(), second: "second".to_string(),fourth_struct};
    let second_struct = SecondStruct {fourth: "fourth".to_string(),fifth: "fifth".to_string()};
    let third_struct = ThirdStruct {base: first_sturct, second_struct};
    println!("{:#?}",third_struct);
    let json_val = to_value(&third_struct)?;
    println!("{}",to_string_pretty(&json_val)?);


    Ok(())
}


#[derive(Debug,Serialize)]
struct FirstStruct {
    first: String,
    second: String,
    fourth_struct: FourthStruct
}

#[derive(Debug,Serialize)]
struct SecondStruct {
    fourth: String,
    fifth: String
}

#[derive(Debug,Serialize)]
struct ThirdStruct {
    #[serde(flatten)]
    base: FirstStruct,
    second_struct: SecondStruct
}

#[derive(Debug,Serialize)]
struct FourthStruct {
    sixth: String,
    seventh: String
}