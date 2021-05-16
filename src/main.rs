use chrono::prelude::*;
use chrono::{Date, Duration};
use primes::is_prime;

pub fn date_to_int(date: Date<Utc>) -> u64 {
    let str_date = date.format("%Y%m%d").to_string();
    str_date.parse::<u64>().unwrap()
}

fn main() {
    let mut dt = Utc.ymd(2021, 1, 1);
    while dt <= Utc.ymd(2023, 12, 21) {
        if is_prime(date_to_int(dt)) {
            println!("{}", dt.format("%Y-%m-%d").to_string());
        }
        dt = dt + Duration::days(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_date_to_int() {
        assert_eq!(date_to_int(Utc.ymd(2020, 1, 1)), 20200101);
    }
}
