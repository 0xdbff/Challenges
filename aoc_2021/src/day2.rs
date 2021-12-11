#[inline]
pub fn part_a() -> isize {
    let mut x = 0;
    let mut y = 0;

    for line in include_str!("day2.input").trim().lines() {
        let mut calls = line.split(' ');
        let command = calls.next().unwrap();
        let value: isize = calls.next().unwrap().parse().unwrap();
        match command {
            "forward" => x += value,
            "up" => y -= value,
            "down" => y += value,
            _ => panic!(),
        }
    }
    x * y
}

#[inline]
pub fn part_b() -> isize {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    for line in include_str!("day2.input").trim().lines() {
        let mut calls = line.split(' ');
        let command = calls.next().unwrap();
        let value: isize = calls.next().unwrap().parse().unwrap();
        match command {
            "forward" => {
                x += value;
                y += aim * value;
            }
            "up" => aim -= value,
            "down" => aim += value,
            _ => panic!(),
        }
    }
    x * y
}
