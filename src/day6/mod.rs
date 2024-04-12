use std::io::stdin;

mod part1;
mod part2;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();

    println!("Part 1 or Part 2? (1/2)");

    stdin().read_line(&mut input)?;

    input = input.trim().to_string();

    if input.eq("1") {
        let _ = part1::part1()?;
    } else if input.eq("2") {
        let _ = part2::part2()?;
    } else {
        println!("Input not valid, Try again.");
    }

    Ok(())
}
