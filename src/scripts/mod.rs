pub mod api;
use rhai::{Engine, EvalAltResult};

// Instantiates the scripting engine RHAI
// -> engine?
fn init_engine(){
    unimplemented!()
}

// Appends header script to a script by combining them as ASTs
// -> ()
fn append_header(){
    unimplemented!()
}

// Executes a script string
// -> Result<(), Box<EvalAltResult>>
fn execute(script: String){
    unimplemented!()
}