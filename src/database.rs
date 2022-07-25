

// a modules for generation database credentials and running commands via processes

pub mod credentials
{

  use rand::{thread_rng, Rng};
  use std::collections::HashMap;
  use std::io::{self, stdin, Write};
  use rand::distributions::Alphanumeric;
  
  pub fn database_credentials(username: String, password_length: usize)
  {
      let rand_string: String = thread_rng().sample_iter(&Alphanumeric).take(password_length).map(char::from).collect();
      let mut credentials_map = HashMap::new();
      let password = &String::from(rand_string);
      credentials_map.insert("username", username);
      credentials_map.insert("password", password.to_string());
      let pasd =  credentials_map.get("password");
      let user =  credentials_map.get("username");
      
      println!("{:?}", credentials_map);
      
      println!("==================================");
      for (key, value) in &credentials_map
      {
        println!("{}: {}", key, value);
      }
      println!("==================================");

  }
  
  pub fn process_builder_command(cwd_one: String, cwd_two: String, cmd_one: String, cmd_two: String, args_one: &mut [String], args_two: &mut [String])
  {
    // command one
    let mut list_dir = Command::new(cmd_one);
    list_dir.args(args_one);
    list_dir.current_dir(cwd_one);
    list_dir.status().expect("process failed to execute");
    println!();
    
    //command two
    let mut list_dir_and_size = Command::new(cmd_two);
    list_dir_and_size.args(args_two);
    list_dir_and_size.current_dir(cwd_two);
    list_dir_and_size.status().expect("process failed to execute");
    println!();
  }
}
