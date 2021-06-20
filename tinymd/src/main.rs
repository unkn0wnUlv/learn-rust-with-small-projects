use std::path::Path;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        2 => parse_markdown_file(&args[1]),
        _ => {
            println!("ERROR -> Invalid invocation (you done goofed!)");
            usage();
        }
    }
}

fn parse_markdown_file(filename: &str) {
    println!("Starting parser...");
    println!("INFO -> Trying to parse {}...", filename);

    // Create a path variable from the filename
    let input_filename = Path::new(filename);

    // Try to open the file
    let file = File::open(&input_filename)
        .expect("ERROR -> Failed to open file!");

    let mut _ptag: bool = false;
    let mut _htag: bool = false;

    // Create a place to store all our tokens
    let mut tokens: Vec<String> = Vec::new();

    // Read the file line-by-line
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_contents = line.unwrap();
        let mut first_char: Vec<char> = line_contents.chars().take(1).collect();
        let mut output_line = String::new();
        match first_char.pop() {
            Some('#') => {
                if _ptag {
                    _ptag = false;
                    output_line.push_str("</p>\n");
                }

                if _htag {
                    _htag = false;
                    output_line.push_str("</h1>\n");
                }
                
                _htag = true;
                output_line.push_str("<h1>");
                output_line.push_str(&line_contents[2..]); // Get all but the first two chars
            },
            _ => {
                if !_ptag {
                    _ptag = true;
                    output_line.push_str("<p>");
                }

                output_line.push_str(&line_contents);
            }
        }
        
        if _ptag {
            _ptag = false;
            output_line.push_str("</p>\n");
        }

        if _htag {
            _htag = false;
            output_line.push_str("</h1>\n");
        }

        if output_line != "<p></p>\n" {
            tokens.push(output_line);
        }
    }

    // Create an output file based on the input file, minus ".md"
    let mut output_filename = String::from(&filename[..filename.len()-3]);
    output_filename.push_str(".html");

    let mut outfile = File::create(output_filename)
        .expect("ERROR -> Could not create output file!");

    for line in &tokens {
        outfile.write_all(line.as_bytes())
            .expect("ERROR -> Could not write to output file!");
    }

    println!("SUCCESS -> Parsing complete!");

}

fn usage() {
    println!("---usage---");
}
