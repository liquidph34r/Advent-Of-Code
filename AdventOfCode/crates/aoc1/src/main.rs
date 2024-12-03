use std::{collections::HashMap, env, fs::read_to_string, io::Split, iter::zip, ops::Add, path::{absolute, PathBuf}};

fn main() {
    let mut distance = 0;
    let (mut vec1, mut vec2) = read_lines();

    vec1.sort();
    vec2.sort();

    //for (v1, v2) in vec1.into_iter().zip(vec2) {
    //    distance += (v2 - v1).abs()
    //}
    //println!("distance: {}", distance);

    println!("SimScore: {}", find_sim_score(&vec1, &vec2))
}

fn read_lines() -> (Vec<i32>, Vec<i32>) {
    let input = "./resources/aoc1input.txt";

    let mut line1 = Vec::new();
    let mut line2 = Vec::new();

     read_to_string(input)
    .unwrap()
    .lines()
    .for_each(|line| {
        let mut numbers = line.split_whitespace();
        if let (Some(left), Some(right)) = (numbers.next(), numbers.next()) {
            line1.push(left.parse::<i32>().unwrap());
            line2.push(right.parse::<i32>().unwrap());
        }
    });

     return (line1, line2);
}

fn find_sim_score(vec1: &[i32], vec2: &[i32]) -> i32 {

    let mut right_map = HashMap::new();
    for  val in vec2.into_iter()  {
        *right_map.entry(val).or_insert(0) += 1;
    }

    let mut simscore = 0;
    
    for val in vec1.into_iter() {
        if let Some(count) = right_map.get(&val) {
            simscore += count*val;
        }
    }
    return simscore;
    /* `i32` value */
    /* `i32` value */

}