pub mod api;
use rhai::{Engine, EvalAltResult, ImmutableString, AST, Dynamic};
use crate::entities::{agent::{*, self}, ship::*};

const HEADER_PATH: &str = "src/scripts/header.rhai";

// Instantiates the scripting engine RHAI
// -> engine?
pub fn init_engine() -> Result<Engine, Box<EvalAltResult>>{
    let mut engine = Engine::new(); 
    api::bind_functions(&mut engine)?; // bind functions to the engine
    Ok(engine)
}

// FIXME:
// register our custom types with the engine (like agent, ship, etc)
fn handle_types(engine: &mut Engine){
    let agent_type = engine.register_type_with_name::<Agents>("agent::Agent");
    agent_type.register_fn("new_agent", Agents::new);
    agent_type.register_fn("get_agent", Agents::get_data);
}

// Appends header script to a script by combining them as ASTs
// -> ()
fn append_header(header: AST, script: AST) -> Result<AST, Box<EvalAltResult>>{
    // combine header and script ASTs into one AST and return it
    // script is appended to header
    Ok(header.merge(&script))
}

// Reads reads a script file as a string and compiles it into an AST
pub fn read_script(engine: &Engine, path: String) -> Result<AST, Box<EvalAltResult>>{
    let ast = engine.compile_file(path.into())?; // compiles the script into an AST
    Ok(ast)
}

// Executes a script AST
// -> Result<(), Box<EvalAltResult>>
pub fn execute(engine: &Engine, script: AST) -> Result<Dynamic, Box<EvalAltResult>>{
    // append the header script to the script to be executed
    let appended_script = append_header(read_script(engine, HEADER_PATH.to_string())?, script)?;
    // execute the script
    // TODO: async execution
    let result = engine.eval_ast(&appended_script)?;
    Ok(result)
}