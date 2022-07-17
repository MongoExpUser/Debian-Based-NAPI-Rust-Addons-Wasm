/* - This file can be compiled to wasm (https://webassembly.org/) and used on the frond-end
   - To compile the file:
     (1) Install rust as: curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
     (2) Install  wasm32-unknown-unknown: rustup target add wasm32-unknown-unknown
     (3) The compile file in CWD, with optimization as:
         rustc -A dead_code --target wasm32-unknown-unknown --crate-type=cdylib sample.rs -o sample.wasm -C opt-level=s -C lto=yes           
*/


#[no_mangle]
pub extern "C" fn irr(cash_flow_array: &mut [f64], calculate_npv: bool) -> f64
{
  // a function for calculating internal rate of return (IRR) of variable-length input array
  let increment: f64  = 1E-4;
  let mut guess: f64 = 1E-1;
  let mut npv_out: f64 = 0.0;
  
  if calculate_npv == true
  {

    loop
    {
      guess += increment;
      let mut npv: f64 = 0.0;
  
      for item in  &mut *cash_flow_array
      {
        npv += *item / (1.0 + guess).powf(*item);
        npv_out = npv;
      }
      
      let condition = npv_out > 0.0;
      
      if !condition
      {
          break;
      }
    }
  }
  
