extern crate clap;
use clap::{App, Arg};
use std::str::FromStr;

fn main() {
    // Parse all the arguments
    let matches = App::new("Radix")
        .version("0.0.1")
        .author("Micah Tseng")
        .about("Converts numbers to differeing radixes")
        .arg(Arg::with_name("input").required(true).help("Input number"))
        .arg(
            Arg::with_name("input radix")
                .short("i")
                .long("input")
                .help("define the input radix (base)")
                .possible_values(&["decimal", "binary", "hex", "octal", "10", "2", "16", "8"])
                .default_value("decimal"),
        )
        .arg(
            Arg::with_name("output radix")
                .short("o")
                .long("output")
                .help("define the output radix (base)")
                .possible_values(&["decimal", "binary", "hex", "10", "2", "16"])
                .default_value("binary"),
        )
        .get_matches();

    // match the input number argument and get it's value
    let input_number = matches.value_of("input").unwrap();
    // println!("Value of input number is: {}\n", input_number);

    // Pull the value of the input radix
    let input_radix = matches.value_of("input radix").unwrap();
    // println!("{:X?}", input_radix);

    // Pull hte value of the output radix
    let output_radix = matches.value_of("output radix").unwrap();
    // println!("{:X?}", output_radix);

    // Instantiate a variable to be used inbetween the input and output funcitons
    let mut intermediary = 0;

    // match and execute the appropriate input function to the correct argument
    match input_radix {
        "decimal" | "10" => intermediary = input_decimal(&input_number),
        "binary" | "2" => intermediary = input_binary(&input_number),
        "hex" | "16" => intermediary = input_hex(&input_number),
        "octal" | "8" => intermediary = input_octal(&input_number),
        _ => println!("Not valid type"),
    }

    // match and execute the appropriate output function to the correct argument
    match output_radix {
        "decimal" | "10" => output_decimal(intermediary),
        "binary" | "2" => output_binary(intermediary),
        "hex" | "16" => output_hex(intermediary),
        _ => println!("Not valid type"),
    }
}

fn input_decimal(input: &str) -> usize {
    println!("Decimal input of: \n\n\t{}\n", input);
    usize::from_str(input).unwrap()
}

fn input_binary(input: &str) -> usize {
    println!("Binary input of: \n\n\t{}\n", input);
    usize::from_str_radix(input, 2).ok().unwrap()
}

fn input_hex(input: &str) -> usize {
    println!("Hex input of: \n\n\t{}\n", input);
    usize::from_str_radix(input, 16).ok().unwrap()
}

fn input_octal(input: &str) -> usize {
    println!("Octal input of: \n\n\t{}\n", input);
    usize::from_str_radix(input, 8).ok().unwrap()
}

fn output_decimal(input: usize) {
    println!("Decimal output of: \n\n\t{}\n", input);
}

fn output_binary(input: usize) {
    // Take the string input and format it in binary
    let str_input = String::from(format!("{:b}", input));
    // initialize a new string to hold the ending value
    let mut formatted_str = String::new();
    // create a vector of chars
    let mut v: Vec<char> = str_input.chars().collect();

    // reverse the vector
    v.reverse();

    // Iterate over the whole vector (adding into the length the number of spaces to be inserted)
    // And insert a space every fourth digit (or mod 5 index)
    for i in 0..(v.len() + (v.len()/4)) {
        if (i+1)%5 == 0 {
            v.insert(i  , ' ');        
        }
    }

    // reverse the vector to put it the correct direction
    v.reverse();

    // spit the vector into a string
    for c in v {
        formatted_str.push(c);
    }
    
    // Print out the answer
    println!("Binary output of:\n\n\t{}\n", formatted_str.trim());
}

fn output_hex(input: usize) {
    println!("Hex output of: \n\n\t{:X}\n", input);
}
