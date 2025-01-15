// Bigger is Greater problem

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'biggerIsGreater' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING w as parameter.
 */

fn biggerIsGreater(w: &str) -> String {
    let mut chars: Vec<char> = w.chars().collect();
    let n = chars.len();

    // Find the pivot
    let mut i = n - 1;
    while i > 0 && chars[i - 1] >= chars[i] {
        i -= 1;
    }

    if i == 0 {
        return "no answer".to_string(); // No larger permutation exists
    }

    // Find the smallest character larger than the pivot
    let mut j = n - 1;
    while chars[j] <= chars[i - 1] {
        j -= 1;
    }

    // Swap the pivot with this character
    chars.swap(i - 1, j);

    // Reverse the suffix
    chars[i..].reverse();

    chars.into_iter().collect()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let T = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..T {
        let w = stdin_iterator.next().unwrap().unwrap();

        let result = biggerIsGreater(&w);

        writeln!(&mut fptr, "{}", result).ok();
    }
}
