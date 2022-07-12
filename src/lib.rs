// lib.rs

use napi_derive::napi;

#[napi]
fn api(specific_gravity: f64) -> u8
{
   let api_value = (141.5/specific_gravity) - 131.5;
   let oil_api_gravity = api_value as u8;
   println!("Oil API Gravity is: {oil_api_gravity}");
   return  oil_api_gravity;
}
