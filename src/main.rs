use std::fs::read_to_string;

fn main() {
    let result = read_lines("./makemore/names.txt");
    let result_part = result[0..10].to_vec();

    println!("First few names:");
    println!("{:?}", result_part);
    println!("Length: {}", result.len());
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
