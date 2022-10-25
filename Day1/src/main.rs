use ringbuf::StaticRb;

const PART: u8 = 2;

fn part1() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let mut contents = std::fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    contents = contents.replace("\r", "");
    let lines = contents.split("\n");
    
    let mut count = 0;
    let mut previous: i32 = -1;
    for line in lines {
        let current: i32 = line.parse().unwrap();
        if previous >= 0 {
            if current > previous {
                count += 1;
            }
        }
        previous = current;
    }

    println!("Answer for part 1: {}", count);
}

fn part2() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let mut contents = std::fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    contents = contents.replace("\r", "");
    let lines = contents.split("\n");
    
    let mut count = 0;
    let mut rb = StaticRb::<i32, 3>::default();
    let (mut prod, mut cons) = rb.split_ref();
    for line in lines {
        let current: i32 = line.parse().unwrap();
        if cons.is_full() {
            let previous = cons.pop();
            if previous.is_some() && current > previous.unwrap() {
                count += 1;
            }
        }
        match prod.push(current) {
            Ok(_) => (),
            Err(n) => println!("Error: failed to add {}", n),
        }
    }

    println!("Answer for part 2: {}", count);
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