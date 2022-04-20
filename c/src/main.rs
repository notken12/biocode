use std::io;
use std::collections::HashMap;

fn main() -> io::Result<()>{
    let mut length = String::new();
    io::stdin().read_line(&mut length)?;

    let mut map = HashMap::<char, i32>::new();

    let mut seq = String::new();
    io::stdin().read_line(&mut seq)?;
    let seq = seq.trim().chars();

    for c in seq {
        match map.get(&c) {
            Some(&count) => map.insert(c, count + 1),
            None => map.insert(c, 1)
        };
    }

    let mut max = 0;
    let mut most_common = ' ';
    for (c, f) in map {
        if f > max {
            max = f;
            most_common = c;
        }
    }

    println!("{}", most_common);
    Ok(())
}
