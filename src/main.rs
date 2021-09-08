use structopt::StructOpt;
mod period;

#[derive(StructOpt)]
#[structopt(name = "period")]
struct Period {
    #[structopt(name = "FROM")]
    from: String,
    #[structopt(name = "TO")]
    to: String,
}

fn main() {
    let period = Period::from_args();
    let _period = period::_Period {
        from: period::string_to_date(&period.from),
        to: period::string_to_date(&period.to),
    };
    let prime_days = _period.prime_days();

    for day in prime_days {
        println!("{}", day.format("%Y-%m-%d").to_string());
    }
}
