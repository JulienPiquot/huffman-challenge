mod huffman_encoder;
mod huffman_tree;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path).unwrap();
    let counter = create_counter(file);
    print_char_count(&counter);
}

fn create_counter<R: Read>(reader: R) -> HashMap<char, i32> {
    let reader = BufReader::new(reader);
    let mut counter = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        for c in line.chars() {
            *counter.entry(c).or_insert(0) += 1;
        }
    }
    counter
}

fn print_char_count(counter: &HashMap<char, i32>) {
    let mut sorted_keys: Vec<_> = counter.keys().collect();
    sorted_keys.sort();
    println!("Character Frequency:");
    for ch in sorted_keys {
        println!("'{}': {}", ch, counter[ch]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::io::Cursor;

    #[test]
    fn test_create_counter() {
        let input_data = "hello world";
        let fake_file = Cursor::new(input_data.as_bytes().to_vec());

        let counter = create_counter(fake_file);

        let mut expected = HashMap::new();
        expected.insert('h', 1);
        expected.insert('e', 1);
        expected.insert('l', 3);
        expected.insert('o', 2);
        expected.insert(' ', 1);
        expected.insert('w', 1);
        expected.insert('r', 1);
        expected.insert('d', 1);

        assert_eq!(counter, expected);
    }
}
