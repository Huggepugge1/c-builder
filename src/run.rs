use crate::build::build;
use crate::cli::Run;
use crate::command;

pub fn run(args: &Run) -> Result<Option<String>, String> {
    match build(args) {
        Ok(Some(v)) => println!("{}", v),
        Ok(None) => (),
        Err(e) => return Err(e),
    }
    let command = String::from("./") + &args.output;
    println!("Running {}", command);
    let mut process = match command::spawn(&command) {
        Ok(process) => process,
        Err(error) => {
            return Err(format!("Failed to run command: {}", error));
        }
    };

    match process.wait() {
        Ok(_) => Ok(None),
        Err(e) => Err(format!("Failed to wait for command: {}", e)),
    }
}

pub fn memory_run(args: &Run) -> Result<Option<String>, String> {
    match build(args) {
        Ok(Some(v)) => println!("{}", v),
        Ok(None) => (),
        Err(e) => return Err(e),
    }
    let command =
        String::from("valgrind --leak-check=full --show-leak-kinds=all --track-origins=yes ./")
            + &args.output;
    println!("Running {}", command);
    let mut process = match command::spawn(&command) {
        Ok(process) => process,
        Err(error) => {
            return Err(format!("Failed to run command: {}", error));
        }
    };

    match process.wait() {
        Ok(_) => Ok(None),
        Err(e) => Err(format!("Failed to wait for command: {}", e)),
    }
}
