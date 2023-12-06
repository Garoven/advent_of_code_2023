fn calc_range(time: f64, distance: f64) -> f64 {
    let delta = time.powf(2.0) - 4.0 * -distance * -1.0;
    let x1 = (-time + delta.sqrt()) / -2.0;
    let x2 = (-time - delta.sqrt()) / -2.0;

    x2.ceil() - x1.floor() - 1.0
}

pub fn part_1(input: impl AsRef<str>) -> u32 {
    let mut it = input.as_ref().lines().filter_map(|l| {
        l.split(':')
            .nth(1)
            .map(|l| l.split_whitespace().filter_map(|s| s.parse::<f64>().ok()))
    });
    let times = it.next().expect("Invalid");
    let distances = it.next().expect("Invalid input");

    times
        .zip(distances)
        .fold(1.0, |acc, (t, d)| acc * calc_range(t, d)) as u32
}

pub fn part_2(input: impl AsRef<str>) -> u32 {
    let mut it = input.as_ref().lines().filter_map(|l| {
        l.split(':').nth(1).map(|l| {
            l.split_whitespace()
                .fold(String::new(), |mut acc, s| {
                    acc += s;
                    acc
                })
                .parse::<f64>()
                .expect("Invalid input")
        })
    });
    let time = it.next().expect("Invalid");
    let distance = it.next().expect("Invalid input");

    calc_range(time, distance) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("./test.txt");

    #[test]
    fn part_1_ok() {
        let result = part_1(INPUT);
        let expected = 288;

        assert_eq!(expected, result);
    }

    #[test]
    fn part_2_ok() {
        let result = part_2(INPUT);
        let expected = 71503;

        assert_eq!(expected, result);
    }
}
