use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    if let Ok(mut f) = File::open("words.txt") {
        let mut buf = String::new();
        f.read_to_string(&mut buf);
        let mut word_counts: HashMap<String, i32> = HashMap::new();
        for word in buf.split_whitespace() {
            let count = word_counts.entry(String::from(word)).or_insert(0);
            *count += 1;
        }
        println!("{:?}", word_counts);
    } else {
        println!("Couldn't open words.txt");
    }
}
