// lib.rs

use napi_derive::napi;

#[napi]
fn oil_api_gravity(specific_gravity: f64) -> u8
{
   let api = (141.5/specific_gravity) - 131.5;
   let oil_api_gr = api as u8;
   println!("Oil API Gravity is: {oil_api_gr}");
   return oil_api_gr;
}
