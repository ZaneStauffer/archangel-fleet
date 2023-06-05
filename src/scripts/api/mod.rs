use rhai::{Engine, EvalAltResult, Dynamic, ImmutableString};
use crate::statics;
use crate::entities::{agent::*, ship::*};

/*
This module binds scripting functions to our API
*/
// For now don't bind agent registration

pub fn bind_functions(engine: &mut Engine) -> Result<(), Box<EvalAltResult>>{
    // list all functions to bind here
    engine.register_fn("get_agent_data", get_data);
    // end list
    Ok(())
}
// TODO:
fn register_agent() -> Result<(), Box<EvalAltResult>>{
    todo!("register_agent function not implemented")
}

// how should the function look in the scripting language?
// use whatever the API needs- for a ship, the ship symbol, for example

/*
so heres our conundrum with the scripting API:
for exammple we want to call the get_agent_data function from the scripting language
but we need to pass in the config and limiter to the function in the script api module
so we need to pass in the config and limiter to the function in the script api module but they cannot be parameters of the function

*/
// we can possibly generalize this with dynamic parameters from RHAI
// native call contexts can prevent malicious scripts from executing
// native call contexts can also be used to access functions from the scripting API module
// can also do callbacks with native call contexts

// if we take a dynamic as a parameter, we can pass in our own types like Agent, and we can call their functions if registered
// TODO: register methods
// How do we use this to get Config and Limiter?
// fn get_agent_data(data: Dynamic) -> Result<(), Box<EvalAltResult>>{
//     // how do we pass the config and limiter without using parameters?
//     // agent.get_data(config, limiter)
//     // Agents::get_agent()?
    
// }
// data is a dynamic, so we can pass in our own types like Agent or Ship


fn get_data(data: Dynamic) -> Result<(), Box<EvalAltResult>>{
    let config = &statics::Config;

    Ok(())
}

fn test() -> i64{
    20
}

fn test_2() -> Dynamic{
    "hehe uwu".into()
}