use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    let names = read_lines("./makemore/names.txt");

    // // step-by-step example
    // println!("First few names:");
    // println!("{:?}", names[0..10].to_vec());
    // println!("Length: {}", names.len());
    
    // let first_name = &names[1];
    // let first_name_extended = format!("<{}>", first_name);
    // println!("name {:?}", first_name_extended);
    // let name_chars: Vec<_> = first_name_extended.chars().collect();
    // println!("name chars: {:?}", name_chars);

    // let char_pairs = name_chars.iter()
    //     .zip(name_chars.iter().skip(1))
    //     .collect::<Vec<_>>();
    // println!("char pairs: {:?}", char_pairs);

    // let mut bigrams = HashMap::new();
    // for key in char_pairs.iter() {
    //     *bigrams.entry(key).or_insert(0) += 1
    // }
    // println!("bigrams: {:?}", bigrams);


    // complete example
    let mut bigram_map = HashMap::new();
    // let names_part = names[0..10].to_vec();
    // for name in names_part.iter() {
    for name in names.iter() {
        let name_extended = format!("<{}>", name);
        let name_chars: Vec<_> = name_extended.chars().collect();
        let char_pairs = name_chars.iter().zip(name_chars.iter().skip(1)).collect::<Vec<_>>();
        for &(&char_a, &char_b) in char_pairs.iter() {
            *bigram_map.entry((char_a, char_b)).or_insert(0) += 1;
        }
    }

    // order by count value and print
    let mut count_vec: Vec<_> = bigram_map.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));
    for bigram in count_vec.iter() {
        println!("{:?}", bigram)
    }

}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
