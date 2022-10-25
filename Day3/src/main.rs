const PART: u8 = 1;
const BIT_LENGTH: usize = 12;


fn convert_vec_to_num(vec: Vec<u8>) -> u32{
    let mut retval: u32 = 0;
    for (i, bit) in vec.iter().enumerate() {
        let shift_amount: u32 = (BIT_LENGTH - 1 - i) as u32;
        retval |= (*bit as u32) << shift_amount;
    }
    return retval;
}

fn part1() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let mut contents = std::fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    contents = contents.replace("\r", "");
    let lines = contents.split("\n");
    
    let mut num_one_bit = vec![0; BIT_LENGTH];
    let mut total_length: u32 = 0;
    for line in lines {
        for (i, val) in line.bytes().enumerate() {
            match val {
                b'0' => (),
                b'1' => num_one_bit[i] += 1,
                char => println!("Unexpected character {} found", char),
            }
        }
        total_length += 1;
    }

    let mut gamma_vec = vec![0u8; BIT_LENGTH];
    for (i, val) in num_one_bit.iter().enumerate() {
        if val * 2 > total_length {
            gamma_vec[i] = 1;
        }
    }
    let gamma = convert_vec_to_num(gamma_vec);
    let epsilon = gamma ^ (1u32 << BIT_LENGTH) - 1;

    let result = gamma * epsilon;
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