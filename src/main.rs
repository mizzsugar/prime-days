use chrono::naive::NaiveDate;
use chrono::Duration;
use primes::is_prime;
use rstest::rstest;
use std::mem;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "period")]
struct Period {
    #[structopt(name = "FROM")]
    from: String,
    #[structopt(name = "TO")]
    to: String,
}

struct _Period {
    from: NaiveDate,
    to: NaiveDate,
}

impl Iterator for _Period {
    type Item = NaiveDate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.from <= self.to {
            let next = self.from + Duration::days(1);
            Some(mem::replace(&mut self.from, next))
        } else {
            None
        }
    }
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

pub fn list_prime_numbers(number: u64) -> Vec<u64> {
    // 参考
    // https://qiita.com/fantm21/items/5e270dce9f4f1d963c1e
    if number < 2 {
        panic!("number should more than 2")
    }
    if number == 2 {
        return vec![2];
    }
    let mut primes: Vec<u64> = vec![2];
    let limit = (number as f64).sqrt().round() + 1f64;
    let mut data: Vec<u64> = (3u64..number + 1).filter(|i| i % 2 != 0).collect();
    loop {
        let p: u64 = data[0];
        let casted = p as f64;
        if limit <= casted {
            primes.append(&mut data);
            return primes;
        }
        primes.push(p);
        data = data.into_iter().filter(|e| e % p != 0).collect();
    }
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

    #[rstest(number, expected,
        case(10, vec![2, 3, 5, 7]),
        case(9, vec![2, 3, 5, 7]),
        case(3, vec![2, 3]),
        case(5, vec![2, 3, 5]),
        case(18, vec![2, 3, 5, 7, 11, 13, 17]),
        case(2, vec![2]),
        #[should_panic]
        case(1, vec![]),
    )]
    fn test_list_prime_numbers(number: u64, expected: Vec<u64>) {
        // 並列化
        // https://caddi.tech/archives/1849
        assert_eq!(list_prime_numbers(number), expected);
    }

    #[rstest(from, to, expected,
        case(
            NaiveDate::from_ymd(2021, 8, 1),
            NaiveDate::from_ymd(2021, 8, 1),
            vec![NaiveDate::from_ymd(2021, 8, 1)],
        ),
        case(
            NaiveDate::from_ymd(2021, 8, 1),
            NaiveDate::from_ymd(2021, 8, 2),
            vec![NaiveDate::from_ymd(2021, 8, 1), NaiveDate::from_ymd(2021, 8, 2)],
        ),
        case(
            NaiveDate::from_ymd(2021, 8, 1),
            NaiveDate::from_ymd(2021, 8, 3),
            vec![NaiveDate::from_ymd(2021, 8, 1), NaiveDate::from_ymd(2021, 8, 2), NaiveDate::from_ymd(2021, 8, 3)],
        ),
    )]
    fn range_period(from: NaiveDate, to: NaiveDate, expected: Vec<NaiveDate>) {
        let period = _Period { from: from, to: to };
        let actual: Vec<NaiveDate> = period.collect();
        assert_eq!(actual, expected);
    }
}
