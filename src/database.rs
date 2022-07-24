
pub mod credentials
{

  use rand::{thread_rng, Rng};
  use std::collections::HashMap;
  use std::io::{self, stdin, Write};
  use rand::distributions::Alphanumeric;
  
  pub fn postgres_db_credentials(password_length: usize)
  {
      let rand_string: String = thread_rng().sample_iter(&Alphanumeric).take(password_length).map(char::from).collect();
      let mut credentials_map = HashMap::new();
      let password = &String::from(rand_string);
      credentials_map.insert("username", "postgres");
      credentials_map.insert("password", password);
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
}
