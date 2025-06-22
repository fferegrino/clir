use anyhow::Result;
use chrono::{Datelike, Local, NaiveDate, Weekday};
use clap::Parser;

const LINE_LENGTH: usize = 22;

#[derive(Parser)]
struct Args {
    #[arg(value_name = "YEAR", default_value = "0")]
    year: i32,
    #[arg(short('y'), long("year"), conflicts_with = "year")]
    show_year: bool,
    #[arg(short, long, conflicts_with = "show_year")]
    month: Option<String>,
}

fn center_text(text: &str) -> String {
    let line_length = 22;
    let padding = (line_length - text.len()) / 2;
    let padding_left = " ".repeat(padding - 1);
    let padding_right = " ".repeat(line_length - padding - text.len() + 1);
    format!("{}{}{}", padding_left, text, padding_right)
}

fn days_in_month(year: i32, month: u32) -> Result<i64> {
    let start_date = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
    }
    .unwrap();
    let end_date = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    Ok(-1 * end_date.signed_duration_since(start_date).num_days())
}

fn get_month(possible_month: Option<String>, current_month: u32) -> u32 {
    match possible_month {
        Some(month) => {
            let month_as_number = month.parse::<u32>();
            match month_as_number {
                Ok(month) => month,
                Err(_) => {
                    match month.to_lowercase().as_str() {
                        "january" => 1,
                        "february" => 2,
                        "march" => 3,
                        "april" => 4,
                        "may" => 5,
                        "june" => 6,
                        "july" => 7,
                        "august" => 8,
                        "september" => 9,
                        "october" => 10,
                        "november" => 11,
                        "december" => 12,
                        _ => current_month,
                    }
                }
            }
        }
        None => current_month,
    }
}

struct WkDay {
    day_in_week: i64,
    day: i64,
}

fn run(args: Args) -> Result<()> {
    let current_year = Local::now().year();
    let current_month = Local::now().month();
    let current_day = Local::now().day();

    let year = if args.year == 0 {
        current_year
    } else {
        args.year
    };
    let month = get_month(args.month, current_month);

    let month_as_string = match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => unreachable!(),
    };

    let header = center_text(&format!("{} {}", month_as_string, year));

    println!("{}", header);
    println!("Su Mo Tu We Th Fr Sa  ");

    let days_in_month = days_in_month(year, month)?;
    let mut vec = vec![];
    for day in 1..=days_in_month {
        let date = NaiveDate::from_ymd_opt(year, month, day as u32).unwrap();
        let weekday = match date.weekday() {
            Weekday::Sun => WkDay{day_in_week: 0, day: day},
            Weekday::Mon => WkDay{day_in_week: 1, day: day},
            Weekday::Tue => WkDay{day_in_week: 2, day: day},
            Weekday::Wed => WkDay{day_in_week: 3, day: day},
            Weekday::Thu => WkDay{day_in_week: 4, day: day},
            Weekday::Fri => WkDay{day_in_week: 5, day: day},
            Weekday::Sat => WkDay{day_in_week: 6, day: day},
        };

        if vec.is_empty() {
            for i in 0..weekday.day_in_week {
                vec.push("  ".to_string());
            }
            vec.push(format!("{:2}", weekday.day));
        } else {
            vec.push(format!("{:2}", weekday.day).as_str().to_string());
        }

        if vec.len() == 7 {
            println!("{}  ", vec.join(" "));
            vec.clear();
        }
    }

    if !vec.is_empty() {
        let missing_days = 7 - vec.len();
        for _ in 0..missing_days {
            vec.push("  ".to_string());
        }
        println!("{}  ", vec.join(" "));
    }
    let padding_bottom = " ".repeat(LINE_LENGTH);
    println!("{}", padding_bottom);
    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
