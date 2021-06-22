use std::process;
use utils;

// Implementation of echo
pub fn builtin_echo(args : &Vec<String>) -> i32 {
    println!("{}", args.join(" "));
    0
}

pub fn builtin_exit() -> i32 {
    println!("Goodbye good friend :)");
    process::exit(0)
}

pub fn builtin_cat(args : &Vec<String>) -> i32 {
    //TODO : read all the files in the args
    debug(format!("reading the file(s) {}", args[0]));

    //TODO : implement reading files
    
    0
}
