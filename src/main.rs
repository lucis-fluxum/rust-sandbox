use std::collections::HashMap;
use std::io;

fn main() {
    let mut roles = HashMap::new();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Nope");

    for entry in buffer.split(',') {
        let mut parts_iterator = entry.split(':')
            .map(str::trim)
            .map(str::to_string);

        roles.insert(parts_iterator.next().unwrap(),
                     parts_iterator.next().unwrap());
    }

    println!("{:#?}", roles);
}
