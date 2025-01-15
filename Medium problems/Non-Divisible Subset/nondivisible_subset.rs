// Non-divisible subset

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'nonDivisibleSubset' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER k
 *  2. INTEGER_ARRAY s
 */

fn nonDivisibleSubset(k: i32, s: &[i32]) -> i32 {
    let mut count = vec![0; k as usize];

    // Count frequencies of remainders
    for &num in s {
        count[(num % k) as usize] += 1;
    }

    // Initialize subset size
    let mut subset_size = 0;

    // Handle numbers with remainder 0
    if count[0] > 0 {
        subset_size += 1;
    }

    // Handle other remainders
    for r in 1..=((k / 2) as usize) {
        if r == k as usize - r {
            // Special case when k is even (e.g., r = k / 2)
            subset_size += 1;
        } else {
            subset_size += count[r].max(count[k as usize - r]);
        }
    }

    subset_size
}


fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let s: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = nonDivisibleSubset(k, &s);

    writeln!(&mut fptr, "{}", result).ok();
}
