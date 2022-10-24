const PART: i32 = 1;

fn part1() {
    let result = 0;

    println!("Answer for part 1: {}", result);
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