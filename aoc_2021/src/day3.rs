use std::fmt::Binary;

struct Bin {
    bin: [isize; 12],
}

pub fn part_a() -> isize {
    let file = include_str!("day3.input").trim().lines();
    let mut data: Vec<Bin>;
    for line in file.next() {
        data[line] = 0;
    }
    0
}
