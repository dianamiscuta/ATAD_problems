/*angry professor problem*/

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'angryProfessor' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts following parameters:
 *  1. INTEGER k
 *  2. INTEGER_ARRAY a
 */

fn angryProfessor(k: i32, a: &[i32]) -> String {
    let on_time_students = a.iter().filter(|&&time| time <= 0).count() as i32;
    
    // If the number of on-time students is less than the threshold, return "YES", else return "NO"
    if on_time_students < k {
        "YES".to_string() // Return "YES" as a String
    } else {
        "NO".to_string()  // Return "NO" as a String
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

        let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

        let a: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();

        let result = angryProfessor(k, &a);

        writeln!(&mut fptr, "{}", result).ok();
    }
}
