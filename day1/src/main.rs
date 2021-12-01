use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let lines = read_lines("../input/day1.txt")?;
    let lines: Vec<i64> = lines.map(|l| l.unwrap().parse::<i64>().unwrap()).collect();

    let count = lines.iter().zip(lines[1..].iter()).filter(|(a, b)| a < b).count();

    println!("{}", count);

    Ok(())
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
