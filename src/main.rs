use luaca::config::Vmconfig;
use luaca::decompile::decompile;
use luaca::interpreter::eval_program;

use std::env;
use std::process;

// One function that parse args and give back flags with the right values

fn main() {
    
    let args: Vec<String> = env::args().collect();

    let vmconfig = Vmconfig::build(args).unwrap_or_else(
        |err|{
            println!("Problem parsing command line: {err}");
            process::exit(1);
        });

    let main = decompile::decompile(vmconfig).unwrap_or_else(
        |err|{
            println!("Problem parsing file: {err}");
            process::exit(1);
        });

    eval_program(main).unwrap_or_else(
        |err|{
            println!("Problem interpreting bytecode: {err}");
            process::exit(1);
        }
    );

}
