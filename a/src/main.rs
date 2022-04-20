use std::io;

fn main() -> io::Result<()> { 
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let mut nums = buffer.trim().split(' ');
    let num1: i32 = nums.next().unwrap().parse().unwrap();
    let num2: i32 = nums.next().unwrap().parse().unwrap();
    println!("{}", (num1 + num2).to_string());
    Ok(())
}
