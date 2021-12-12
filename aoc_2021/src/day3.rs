#[inline]
pub fn part_a() -> usize {
    let file: Vec<_> = include_str!("day3.input").trim().lines().collect();
    let mut char_cnt = 0;
    let mut gamma = String::new();
    let mut epsilon = String::new();
    'a: loop {
        let mut column_total = 0;
        for line in 1..file.len() {
            match file[line - 1].chars().nth(char_cnt) {
                Some(x) => column_total += x as usize - 48,
                None => break 'a,
            };
        }
        char_cnt += 1;
        if (column_total as f64 / file.len() as f64) > 0.5 {
            gamma.push('1');
            epsilon.push('0');
        } else if (column_total as f64 / file.len() as f64) < 0.5 {
            gamma.push('0');
            epsilon.push('1');
        } else {
            panic!()
        }
    }
    usize::from_str_radix(&gamma, 2).unwrap() * usize::from_str_radix(&epsilon, 2).unwrap()
}
