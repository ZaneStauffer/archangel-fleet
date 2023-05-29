pub mod api;
use rhai::{Engine, EvalAltResult, AST};

const HEADER_PATH: &str = "src/scripts/header.rhai";

// Instantiates the scripting engine RHAI
// -> engine?
pub fn init_engine() -> Result<Engine, Box<EvalAltResult>>{
    let mut engine = Engine::new(); 
    Ok(engine)
}

// Appends header script to a script by combining them as ASTs
// -> ()
fn append_header(header: AST, script: AST) -> Result<AST, Box<EvalAltResult>>{
    // combine header and script ASTs into one AST and return it
    // script is appended to header
    Ok(header.merge(&script))
}

// Reads a script file and returns an AST of the script
fn read_script(engine: &Engine, path: String) -> Result<AST, Box<EvalAltResult>>{
    let ast = engine.compile_file(path.into())?; // compiles the script into an AST
    Ok(ast)
}

// Executes a script AST
// -> Result<(), Box<EvalAltResult>>
fn execute(engine: &Engine, script: AST) -> Result<(), Box<EvalAltResult>>{
    let appended_script = append_header(read_script(engine, HEADER_PATH.to_string())?, script)?;
    // execute, return result
    unimplemented!()
}