use std::error::Error;
use std::{path, env, fs};

use test_postgresql::create_client;

pub fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    let args: Vec<String> = env::args().collect();
    let file_name = args.get(1).expect("A sql filename should be specified");

    let absoute_path= path::absolute(format!("sql/{file_name}"))?;

    let file_content = fs::read_to_string(absoute_path)?;

    let mut client = create_client()?;

    let result = client.simple_query(&file_content)?;

    dbg!(result);

    Ok(())
}