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

#[napi]
fn gamma_function(a: f64) -> f64
{
  // a function for calculating gamma function
  // Reference: Nemes, G. (2008). New asymptotic expansion for the Γ(x) function (an update).
  // In Stan's Library, Ed.S.Sykora, Vol.II. First released December 28, 2008.
  // Link: http://www.ebyte.it/library/docs/math08/GammaApproximationUpdate.html.
  // See Nemes' formula & Fig.1 on page 6 of full text: Nemes_6.
  // Application: Unconventional natural gas production decline analysis
  let  coefficient6 = ( 1.0 + 1.0/(12.0 * a * a) + 1.0/(1440.0 * a.powf(4.0)) + 239.0/(362880.0 * a.powf(6.0)) ).powf(a);   //Nemes_6 coefficient
  return (a/E).powf(a) * (2.0 * PI / a).sqrt() * coefficient6;
}


#[napi]
fn  gdf(a: f64, x: f64) -> f64
{
  // a function for calculating gamma distribution function
  // Reference: NIST/SEMATECH e-Handbook of statistical methods.
  // http://www.itl.nist.gov/div898/handbook/eda/section3/eda366b.htm. Retrieved January 5, 2016.
  // Application: Unconventional natural gas production decline analysis
  let gdf_value =  a.powf(x - 1.0) * (-a).exp() / gamma_function(x);
  println!("Gamma Distribution Function value is: {gdf_value}");
  return  1.2; //gdf_value;
}
