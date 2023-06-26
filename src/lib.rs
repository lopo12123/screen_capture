#[macro_use]
extern crate napi_derive;


#[napi]
pub fn helloworld() -> String {
    "Just a classic hello-world.".to_string()
}