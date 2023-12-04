///associate greetings module with this crate

mod greetings;
mod how_you_hold_data_for_operations;

extern crate hello_world_lib;

//use greetings::default_greeting;
///Optionally load each member of greetings
/*use greetings::default_greeting;
use greetings::spanish;
use greetings::french;*/
///Alternatively, use * to load them all
//use greetings::*;
///Alternatively, load them in one line
//use greetings::*;
use greetings::{english, spanish, french};
use how_you_hold_data_for_operations::{primitives, derived};
use primitives::{scalar, compound};
use derived::user_defined;



fn main() {
    

    println!("Hello, world!");
    println!(
        "My first greeting is '{}' and the second is '{}'",
        english::default_greeting(),
        english::default_greeting2()
    );

    // println!("{}", english::default_greeting());
    // println!("{}", spanish::default_greeting());
    // println!("{}", french::default_greeting());
    // println!("{}", hello_world_lib::greeting_from_lib());
    // scalar::scalar2();
    // scalar::scalar1();
    
    //compound::compound1();
    compound::compound2();
    user_defined::run();
    

}
