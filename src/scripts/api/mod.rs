use rhai::{Engine, EvalAltResult};

/*
This module binds scripting functions to our API
*/
// For now don't bind agent registration

pub fn bind_functions(engine: &mut Engine) -> Result<(), Box<EvalAltResult>>{
    // list all functions to bind here
    engine.register_fn("test", test_function);
    
    Ok(())
}

fn test_function(){
    unimplemented!()
}