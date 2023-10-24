#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run(args: Vec<u8>) -> Result<(), String> {
    println!("{:?}", args);
    Ok(())
}
