const PART: u8 = 1;

fn part1() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let mut contents = std::fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    contents = contents.replace("\r", "");
    let lines = contents.split("\n");
    
    let mut depth = 0;
    let mut distance = 0;
    for line in lines {
        let splits = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let move_distance: i32 = splits[1].parse().unwrap();
        match splits[0] {
            "forward" => distance += move_distance,
            "up" => depth -= move_distance,
            "down" => depth += move_distance,
            move_type => println!("Failed to parse move {}", move_type),
        }
    }

    println!("Answer for part 1: {}", depth * distance);
}

fn part2() {
    let result = 0;

    println!("Answer for part 2: {}", result);
}

fn main() {
    if PART == 1 {
        part1();
    } else if PART == 2 {
        part2();
    } else {
        println!("A part number of {} is invalid.", PART);
    }
}