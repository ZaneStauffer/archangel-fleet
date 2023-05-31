use rhai::{Engine, EvalAltResult, Dynamic, ImmutableString};
use crate::entities::{agent::*, ship::*};

/*
This module binds scripting functions to our API
*/
// For now don't bind agent registration

pub fn bind_functions(engine: &mut Engine) -> Result<(), Box<EvalAltResult>>{
    // list all functions to bind here
    engine.register_fn("test", test)
          .register_fn("test2", test_2)
          
    ;
    // end list
    Ok(())
}
// TODO:
fn register_agent() -> Result<(), Box<EvalAltResult>>{
    Ok(())
}

// how should the function look in the scripting language?
// use whatever the API needs- for a ship, the ship symbol, for example


fn get_data() -> Result<(), Box<EvalAltResult>>{
    Ok(())
}

fn test() -> i64{
    20
}

fn test_2() -> Dynamic{
    "hehe uwu".into()
}