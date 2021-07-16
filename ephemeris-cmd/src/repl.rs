
use rustyline::error::ReadlineError;
//use rustyline::Editor;
use ephemeris::state::State;

pub fn repl(_state: &mut Box<State>) {

    println!("Ephemeris Interactive Shell.");
    println!("THIS IS CURRENTLY NOT IMPLEMENTED!");
    let mut rl = rustyline::Editor::<()>::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                println!("Line: {:?}", line);
                if line == "exit" {
                    break;
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("Ctrl-C caught, exiting.");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("Ctrl-D caught, exiting.");
                break
            },
            Err(err) => println!("Error: {:?}", err),
        }
    }
} 
