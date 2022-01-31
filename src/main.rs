use std::env;
use std::fs;
use std::slice::Iter;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Game {
    difficulty: u64,
    board: Vec<u64>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let learning_file = &args[1];
    let target_file = &args[2];

    let learn_json = fs::read_to_string(learning_file)
        .expect("Failed to read the 'learning' json :(");

    let target_json = fs::read_to_string(target_file)
        .expect("Failed to read the 'target' json :(");

    let learning_data: Vec<Game> = serde_json::from_str(&learn_json)
        .expect("Failed to parse learning data from string");

    let target_data: Vec<Vec<u64>> = serde_json::from_str(&target_json)
        .expect("Failed to parse target data from string");

    print!("Processing learning data...");
    let stats: Vec<Vec<u64>> = process_data(learning_data);
    println!("DONE\n{:?}", stats);
    for stat in &stats {
        print!("{:?}\t", calculate_mean(stat));
    }


    print!("\nDetecting Given sudoku difficulty...");
    let results: Vec<(u64, u64)> = judge_difficulty(target_data, stats);
    println!("DONE");

    println!("Results: (difficulty, number count)");
    for r in results {
        println!("\t{:?}", r);
    }
}

fn process_data(data: Vec<Game>) -> Vec<Vec<u64>> {
    let mut results: Vec<Vec<u64>> = vec![vec![]; 4];
    for v in data  {
        let index: u64 = v.difficulty;
        let board: Vec<u64> = v.board;
        results[index as usize].push(find_number_count(board));
    }
    return results;
}

fn find_number_count(board: Vec<u64>) -> u64 {
    let mut count: u64 = 0;
    for v in board {
        if v != 0 {
            count += 1;
        }
    }
    return count;
}

fn judge_difficulty(data: Vec<Vec<u64>>, mut stats: Vec<Vec<u64>>) -> Vec<(u64, u64)> {
    let mut ret_val: Vec<(u64, u64)> = vec![];
    for game in data {
        let count = find_number_count(game);
        let mut found: (u64, f64) = (0, 81.0);

        for (i, stat) in stats.iter().enumerate() {
            let diff: f64 = (calculate_mean(&stat) - count as f64).abs();
            if diff < found.1 {
                found = (i as u64, diff)
            }
        }
        stats[found.0 as usize].push(count);
        //for stat in &stats {
            //print!("{:?}\t", calculate_mean(stat));
        //}
        //println!(" ");
        ret_val.push((found.0, count));
    };
    ret_val
}

fn calculate_mean(vector: &[u64]) -> f64 {
    let sum: u64 = Iter::sum(vector.iter());
    (sum as f64) / (vector.len() as f64)
}
