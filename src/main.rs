// main.rs 

mod database;
pub use crate::database::credentials;

fn main()
{
    // create postgres database credential
    let postgres_username: String  = "postgres".to_string();
    let postgres_password_length: usize  = 10;
    credentials::database_credentials(postgres_username, postgres_password_length);
    
    // confirm successfull test
    println!("Testing NAPI-Rust Addons");
}
