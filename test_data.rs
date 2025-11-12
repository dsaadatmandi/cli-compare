use std::fs::File;
use std::io::{BufWriter, Write};
use rand::Rng;
use rand::distributions::Alphanumeric;

fn random_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

fn main() -> std::io::Result<()> {
    let count = 15_000_000; // Scale to 15M for real test
    
    // Generate set1
    let mut set1 = Vec::with_capacity(count);
    for _ in 0..count {
        set1.push(random_string(16));
    }
    
    // Generate set2 - copy 90% from set1, 10% missing
    let mut set2 = set1[..count * 9 / 10].to_vec();
    for _ in 0..count / 10 {
        set2.push(random_string(16));
    }
    
    // Write files
    let mut f1 = BufWriter::new(File::create("fullset.txt")?);
    for s in &set1 {
        writeln!(f1, "{}", s)?;
    }
    
    let mut f2 = BufWriter::new(File::create("checkset.txt")?);
    for s in &set2 {
        writeln!(f2, "{}", s)?;
    }
    
    println!("Generated {} entries, ~{}% missing from set2", 
             count, (count - count * 9 / 10) * 100 / count);
    Ok(())
}
