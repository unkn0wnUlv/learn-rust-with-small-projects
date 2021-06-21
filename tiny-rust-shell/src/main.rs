use std::io::{self,Write};
use std::str::FromStr;

static DEBUG: bool = false;

#[cfg(test)]
mod unittest_tokenize_command {
    use super::*;
    #[test]
    #[ignore]
    fn empty_command() {
        assert_eq!("", tokenize_command("".to_string()).keyword);
    }

    #[test]
    fn test_keyword() {
      assert_eq!("test", tokenize_command("test".to_string()).keyword)
    }

    #[test]
    fn no_arg() {
      assert_eq!(0, tokenize_command("test".to_string()).args.len())
    }

    #[test]
    fn one_arg() {
      assert_eq!(1, tokenize_command("test one".to_string()).args.len())
    }

    #[test]
    fn multi_args() {
      assert_eq!(3, tokenize_command("test one two three".to_string()).args.len())
    }

    #[test]
    #[ignore]
    fn quotes() {
      assert_eq!(2, tokenize_command("test \"one two\" three".to_string()).args.len())
    }
}


struct Command {
    keyword: String,
    args: Vec<String>,
}

enum Builtin {
    Echo,
}

impl FromStr for Builtin {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "echo" => Ok(Builtin::Echo),
            _ => Err(()),
        }
    }
}

// Implementation of echo
fn builtin_echo(args : &Vec<String>) -> i32 {
    println!("{}", args.join(" "));
    0
}

// Process the command, checking if it is an available built in
fn process_command(c : Command) -> i32 {
    match Builtin::from_str(&c.keyword) {
        Ok(Builtin::Echo) => builtin_echo(&c.args),
        _ => {
            println!("{}: command not yet implemented", &c.keyword);
            1
        },
    }
}

fn print_prompt() {
    let prompt_char = "-~>";
    print!("{0} ", prompt_char);
    io::stdout().flush().unwrap();
}

fn read_command() -> String {
    let mut command = String::new();
    io::stdin().read_line(&mut command)
        .expect("Failed to read command");

    if DEBUG {
        println!("DEBUG: {:?}", command);
    }

    command
}

fn tokenize_command(c: String) -> Command {
    let mut command_split : Vec<String> = c.split_whitespace().map(|s| s.to_string()).collect();

    if DEBUG {
        println!("DEBUG: Split input {:?}", command_split);
    }

    let command = Command {
        keyword : command_split.remove(0),
        args : command_split,
    };
    command
}

fn main() {
    println!("~~~~~ WELCOME TO THE TINY RUST SHELL ~~~~~");
    loop {
        print_prompt();

        let command = read_command();

        let split_command: Command = tokenize_command(command);

        process_command(split_command);
    }
}
