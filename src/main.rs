use std::fs;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name="arguments")]
struct Arguments{
    #[structopt(short = "-n", long)]
    line_number: bool,
    #[structopt(name = "PATTERN")]
    pattern: String,
    #[structopt(name = "FILE")]
    path: Vec<String>,
}

fn grep(arguments: &Arguments, content: String, file_name: &str) {
    let mut line_number = 1;
    for line in content.lines() {
        if line.contains(arguments.pattern.as_str()) {
            let prefix_line_number = if arguments.line_number {
                format!("{}:", line_number)
            }else{
                format!("")
            };
            let prefix_file_name = if arguments.path.len() > 1{
                format!("{}:", file_name)
            }else{
                format!("")
            };
            println!("{}{}{}", prefix_file_name, prefix_line_number, line)
        }
        line_number += 1;
    }
}

fn run(arguments: Arguments){
    for file in arguments.path.iter() {
        match fs::read_to_string(file) {
            Ok(content) => grep(&arguments, content, file),
            Err(reason) => println!("{}", reason)
        }
    }
}

fn main() {
    let arguments = Arguments::from_args();
    run(arguments);
}