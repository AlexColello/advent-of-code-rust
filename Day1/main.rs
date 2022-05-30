fn main() {
    println!("Hello World!");

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

    println!("Answer: {}", count);
}