// Lily's Homework problem

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::collections::HashMap;
/*
 * Complete the 'lilysHomework' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn lilysHomework(arr: &[i32]) -> i32 {
    fn count_swaps(arr: &[i32], target: &[i32]) -> i32 {
        let mut swaps = 0;
        let mut arr = arr.to_vec();
        let mut index_map: HashMap<i32, usize> = arr
            .iter()
            .enumerate()
            .map(|(i, &val)| (val, i))
            .collect();

        for i in 0..arr.len() {
            if arr[i] != target[i] {
                swaps += 1;

                let correct_value = target[i];
                let current_index = index_map[&arr[i]];
                let target_index = index_map[&correct_value];

                arr.swap(current_index, target_index);

                index_map.insert(arr[current_index], current_index);
                index_map.insert(arr[target_index], target_index);
            }
        }
        swaps
    }
    
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();

    let ascending_swaps = count_swaps(arr, &sorted_arr);

    sorted_arr.reverse();
    let descending_swaps = count_swaps(arr, &sorted_arr);

    ascending_swaps.min(descending_swaps)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = lilysHomework(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}
