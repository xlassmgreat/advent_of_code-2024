mod day1;

fn main() {
    if let Some(day) = std::env::args().nth(1) {
        match day.parse::<u32>() {
            Ok(day) => {
                if day > 25 {
                    eprintln!("Not a real day")
                } else {
                    let out = match day {
                        1 => day1::day1(),
                        _ => panic!("Something weird"),
                    };

                    match out {
                        Ok((p1, p2)) => println!("Part 1: {p1} and Part 2: {p2}"),

                        Err(e) => eprintln!("There's an IO error: {e}"),
                    }
                }
            }
            Err(e) => eprintln!("Argument could not be parsed: {e}"),
        }
    } else {
        eprintln!("No argument passed.");
    }
}
