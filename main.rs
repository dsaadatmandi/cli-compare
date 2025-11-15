use std::{collections::HashSet, env, fs, path::PathBuf, process::exit};
use rayon::prelude::*;

fn help() {
    println!("usage: cli-compare full_dataset_path to_check_dataset_path not_found_path [default]")
}

fn main() {
    let time = std::time::Instant::now();
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        help();
        exit(0)
    }

    let path_full_set = PathBuf::from(&args[1]);
    let path_to_check = PathBuf::from(&args[2]);
    let path_output = PathBuf::from(&args[3]);

    match args[4].as_str() {
        "default" => standard_compare(path_full_set, path_to_check, path_output),
        // "fast" => fast_compare(),
        _ => {
            println!("Mode could not be determined");
            unreachable!()
        }
    }

    println!("Time elapsed: {:?} ms", time.elapsed().as_millis())

}

fn standard_compare(fp: PathBuf, p2c: PathBuf, out: PathBuf) {
    let full_set = fs::read_to_string(fp).expect("Could not load full set file");
    println!("Loaded full dataset from disk");
    let full_set_hs: HashSet<&str> = full_set
    .par_lines()
    .map(|s| s.trim())
    .collect();
    println!("Created HashSet from dataset");

    let check_set = fs::read_to_string(p2c).expect("Could not load check set file");
    println!("Loaded set to check to memory");
    
    let not_found_set: Vec<&str> = check_set
    .par_lines()
    .map(|e| e.trim())
    .filter(|e| !full_set_hs.contains(e))
    .collect();

    println!("Completed comparison");

    println!("Found {} missing entries", not_found_set.len());

    fs::write(out, not_found_set.join("\n")).expect("Could not write output file");

}

