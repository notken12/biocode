use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;

    let dna = buffer.trim().chars();

    let mut nucleotides = String::new();

    let mut count = 0;
    for d in dna {
        if count % 3 == 0 {
            nucleotides.push(d);
        }
        count += 1;
    }

    println!("{}", nucleotides);

    Ok(())
}
