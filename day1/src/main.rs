// advent of code 2003: day 1
// https://adventofcode.com/2023/day/1

use std::io::{self, BufRead};

fn main() {
    let mut calibration = 0;

    // read the input from stdin, looping through each line
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    // while let Some(line) = lines.next() {
    for line in lines {
        let input = line.unwrap();
        let mut buffer = String::from("");

        // loop through the string, index + 1; If the char is numeric, append
        // to the buffer. If not, see if the slice matches one of our numeric strings
        for n in 0..input.len() {
            // get slice of string, starting with our new index
            let s = input.get(n..).unwrap();
            let first = s.chars().next().unwrap();

            if first.is_numeric() {
                buffer += first.to_string().as_str();
            } else {
                // This solves the second half of the challenge. Comment it out
                // if running against the first half of the challenge.

                // Check if the string *begins with* the name of a number
                if s.starts_with("one") { buffer += "1"; }
                if s.starts_with("two") { buffer += "2"; }
                if s.starts_with("three") { buffer += "3"; }
                if s.starts_with("four") { buffer += "4"; }
                if s.starts_with("five") { buffer += "5"; }
                if s.starts_with("six") { buffer += "6"; }
                if s.starts_with("seven") { buffer += "7"; }
                if s.starts_with("eight") { buffer += "8"; }
                if s.starts_with("nine") { buffer += "9"; }
            }
        }

        // Could strip all non-numeric characters and use simple string indexing
        let numbers: String = buffer.chars().filter(|c| c.is_ascii_digit()).collect();

        // Combine the first and last numbers into a single two-digit number
        // and parse it into an i32 so we can add it to the calibration.
        let number = format!(
            "{}{}",
            numbers.chars().next().unwrap(),
            numbers.chars().nth(numbers.len() - 1).unwrap()
        ).parse::<i32>().unwrap();
        calibration += number;
    }
    println!("Calibration: {}", calibration);
}
