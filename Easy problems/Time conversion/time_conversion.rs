/* time conversion problem */

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'timeConversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn timeConversion(s: &str) -> String {
// Extract hours, minutes, seconds, and period (AM/PM) from the input string
    let hours: i32 = s[0..2].parse().unwrap();
    let minutes = &s[3..5];
    let seconds = &s[6..8];
    let period = &s[8..10];

    // Convert to 24-hour format based on the period
    let new_hours = match period {
        "AM" => {
            if hours == 12 {
                0 // Convert 12 AM to 00
            } else {
                hours
            }
        }
        "PM" => {
            if hours == 12 {
                12 // Leave 12 PM as 12
            } else {
                hours + 12 // Convert PM hours to 24-hour format
            }
        }
        _ => panic!("Invalid time format"),
    };

    // Format the time into 24-hour format
    format!("{:02}:{:02}:{:02}", new_hours, minutes, seconds)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
