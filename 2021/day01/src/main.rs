use std::fs;

fn main() {
    let contents = fs::read_to_string("resources/input.txt").expect("Something went wrong reading the file");

    let depth_increases = count_increasing_depth(&contents);
    println!("There are {} increases in depth", depth_increases);

    let measurement_increases = sliding_window_depth_count(&contents);
    println!("There are {} measurement increases with a sliding window", measurement_increases);
}

fn count_increasing_depth(readings: &str) -> usize {
    let mut count = 0;
    let mut prev_reading: Option<i32> = None;
    for line in readings.lines() {
        let reading = line.trim().parse().unwrap();
        match prev_reading {
            Some(value) => if reading > value { count += 1 },
            _ => {}
        }
        prev_reading = Some(reading);
    }
    count
}

fn sliding_window_depth_count(readings: &str) -> usize {
    let mut count = 0;
    let mut window = Vec::new();
    let mut prev_measurement: Option<i32> = None;
    for line in readings.lines() {
        let reading: i32 = line.trim().parse().unwrap();
        window.push(reading);
        if window.len() < 3 {
            continue;
        }
        let measurement: i32 = window.iter().sum();
        match prev_measurement {
            Some(value) => if measurement > value { count += 1 },
            _ => {}
        }
        prev_measurement = Some(measurement);
        window.remove(0);
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_increases() {
        let readings = "\
199
200
208
210
200
207
240
269
260
263";
        assert_eq!(7, count_increasing_depth(readings));
    }

    #[test]
    fn count_sliding_window() {
        let readings = "\
199
200
208
210
200
207
240
269
260
263";
       assert_eq!(5, sliding_window_depth_count(readings));
    }
}