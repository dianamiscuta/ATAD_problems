// Time in Words problem

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'timeInWords' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts following parameters:
 *  1. INTEGER h
 *  2. INTEGER m
 */

fn timeInWords(h: i32, m: i32) -> String {
    let numbers = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
        "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen",
        "nineteen", "twenty", "twenty one", "twenty two", "twenty three", "twenty four", "twenty five",
        "twenty six", "twenty seven", "twenty eight", "twenty nine",
    ];

    if m == 0 {
        return format!("{} o' clock", numbers[h as usize]);
    } else if m <= 30 {
        if m == 15 {
            format!("quarter past {}", numbers[h as usize])
        } else if m == 30 {
            format!("half past {}", numbers[h as usize])
        } else if m == 1 {
            format!("one minute past {}", numbers[h as usize])
        } else {
            format!("{} minutes past {}", numbers[m as usize], numbers[h as usize])
        }
    } else {
        let next_hour = if h == 12 { 1 } else { h + 1 };
        let remaining_minutes = 60 - m;
        if remaining_minutes == 15 {
            format!("quarter to {}", numbers[next_hour as usize])
        } else if remaining_minutes == 1 {
            format!("one minute to {}", numbers[next_hour as usize])
        } else {
            format!("{} minutes to {}", numbers[remaining_minutes as usize], numbers[next_hour as usize])
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let h = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let m = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = timeInWords(h, m);

    writeln!(&mut fptr, "{}", result).ok();
}
