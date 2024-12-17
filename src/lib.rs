use postgres::{Client, NoTls, Error};
use std::env;

pub fn create_client() -> Result<Client, Error> {
    let user = env::var("DATABASE_USER").expect("env: DATABASE_USER hast do be set");
    let password = env::var("DATABASE_PASSWORD").expect("env: DATABASE_PASSWORD hast do be set");
    let dbname   = env::var("DATABASE_NAME").expect("env: DATABASE_NAME hast do be set");

    Client::connect(&format!("host=localhost user={user} password={password} dbname={dbname}"), NoTls)
}