// main.rs 

mod utilities;
pub use crate::utilities::utils;

fn main()
{
    // create postgres database credential
    let postgres_username: String  = "postgres".to_string();
    let postgres_password_length: usize  = 10;
    utils::database_credentials(postgres_username, postgres_password_length);
    
    // run commands via processes
    let cwd_one: String = "/home/ubuntu".to_string();
    let cwd_two: String = "/home/ubuntu".to_string();
    let cmd_one = "ls".to_string();
    let cmd_two = "ls".to_string();
    let mut args_one:  [String; 2] = [ "-l".to_string(), "-a".to_string() ];
    let mut args_two:  [String; 1] = [ "-trl".to_string() ];
    utils::process_builder_command(cwd_one, cwd_two, cmd_one, cmd_two, &mut args_one, &mut args_two);
    
    // confirm successfull test
    println!("Testing NAPI-Rust Addons");
}
