/* migratory birds problem */

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::collections::HashMap;
/*
 * Complete the 'migratoryBirds' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn migratoryBirds(arr: &[i32]) -> i32 {
    // Create a HashMap to count the occurrences of each bird type
    let mut bird_count: HashMap<i32, i32> = HashMap::new();

    // Count the sightings of each bird type
    for &bird in arr {
        *bird_count.entry(bird).or_insert(0) += 1;
    }

    // Find the bird type with the maximum count, preferring the smaller id in case of a tie
    let mut max_count = 0;
    let mut result = i32::MAX;

    for (&bird_type, &count) in &bird_count {
        if count > max_count || (count == max_count && bird_type < result) {
            max_count = count;
            result = bird_type;
        }
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _arr_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = migratoryBirds(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}
