#[inline]
pub fn input() -> Vec<u64> {
    let data: Vec<u64> = include_str!("input.txt")
        .trim()
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect();
    data
}

//all values should be positive
#[inline]
pub fn part_a() -> u64 {
    let data = input();
    let mut answer = 0;
    for i in data.windows(2) {
        if i[1] > i[0] {
            answer += 1;
        }
    }
    answer
}

#[inline]
pub fn part_b() -> u64 {
    let data = input();
    let mut answer = 0;
    for i in data.windows(4) {
        if i[3] > i[0] {
            answer += 1;
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a() {
        assert_eq!(super::part_a(), 1301);
    }
    #[test]
    fn part_b() {
        assert_eq!(super::part_b(), 1346);
    }
}
