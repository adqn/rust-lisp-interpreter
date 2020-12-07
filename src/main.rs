use std::io;
use std::io::Write;

use regex::Regex;

// decompose input into Vec<&str> containing tokens and 
// lexical hierarchies
fn parse_input<'a>(input: &'a str) -> Vec<&'a str> {

    vec!["test"] 
}

// evaluate atomic expression from Vec<&str>
fn eval_expr(expr: Vec<&str>) {

}

fn main() {
    let get_expr = Regex::new(r"\(.+\)").unwrap();
    let mut expr: Vec<&str> = vec![];

    loop {
        print!(">> ");

        std::io::stdout().flush().ok().expect("Could not flush stdout");
        let mut input: String = String::new();
        let _ = std::io::stdin().read_line(&mut input);

        if get_expr.is_match(&input[..]) {
            println!("Valid input!");
            parse_input(&input[..]);
        } else {
            println!("Invalid input!");
        }
    }
}
