// The Power Sum problem

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'powerSum' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER X
 *  2. INTEGER N
 */

fn powerSum(X: i32, N: i32) -> i32 {
  fn helper(target: i32, power: i32, current: i32) -> i32 {
        let value = current.pow(power as u32);

        if value > target {
            0 // Base case: Exceeded target
        } else if value == target {
            1 // Base case: Exact match
        } else {
            // Recursive case: Include or exclude the current number
            helper(target - value, power, current + 1) + helper(target, power, current + 1)
        }
    }

    helper(X, N, 1)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let X = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let N = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = powerSum(X, N);

    writeln!(&mut fptr, "{}", result).ok();
}
