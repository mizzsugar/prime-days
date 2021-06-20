use chrono::naive::NaiveDate;
use chrono::Duration;
use primes::is_prime;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "period")]
struct Period {
    #[structopt(name = "FROM")]
    from: String,
    #[structopt(name = "TO")]
    to: String,
}

pub fn date_to_int(date: NaiveDate) -> u64 {
    let str_date = date.format("%Y%m%d").to_string();
    str_date.parse::<u64>().unwrap()
}

pub fn string_to_date(str_date: &str) -> NaiveDate {
    NaiveDate::parse_from_str(str_date, "%Y-%m-%d")
        .ok()
        .unwrap()
}

fn main() {
    let period = Period::from_args();
    let from_date = string_to_date(&period.from);
    let to_date = string_to_date(&period.to);
    let mut dt = from_date;

    while dt <= to_date {
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
        assert_eq!(date_to_int(NaiveDate::from_ymd(2020, 1, 1)), 20200101);
    }
}
