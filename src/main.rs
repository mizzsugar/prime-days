use chrono::prelude::*;
use chrono::Date;

pub fn date_to_int(date: Date<Utc>) -> u32 {
    let str_date = date.format("%Y%m%d").to_string();
    str_date.parse::<u32>().unwrap()
}

fn main() {
    let dt = Utc.ymd(2020, 1, 1);
    println!("{}", date_to_int(dt));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_date_to_int() {
        assert_eq!(date_to_int(Utc.ymd(2020, 1, 1)), 20200101);
    }
}
