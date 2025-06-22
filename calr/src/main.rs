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

fn center_text(text: &str, line_length: usize) -> String {
    let padding = (line_length - text.len()) / 2;
    let padding_left = " ".repeat(padding - 1);
    let padding_right = " ".repeat(line_length - padding - text.len() + 1);
    format!("{}{}{}", padding_left, text, padding_right)
}

fn days_in_month(year: i32, month: u32) -> i64 {
    let start_date = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1).expect("Invalid date")
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1).expect("Invalid date")
    };
    let end_date = NaiveDate::from_ymd_opt(year, month, 1).expect("Invalid date");
    -1 * end_date.signed_duration_since(start_date).num_days()
}

fn get_month(possible_month: &Option<String>, current_month: u32) -> u32 {
    match possible_month {
        Some(month) => {
            let month_as_number = month.parse::<u32>();
            match month_as_number {
                Ok(month) => month,
                Err(_) => match month.to_lowercase().as_str() {
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
                },
            }
        }
        None => current_month,
    }
}

fn day_headers(end_line: bool) {
    if end_line {
        println!("Su Mo Tu We Th Fr Sa  ");
    } else {
        print!("Su Mo Tu We Th Fr Sa  ");
    }
}

fn add_day_to_vec(vec: &mut Vec<String>, weekday: WkDay) {
    if vec.is_empty() {
        for i in 0..weekday.day_in_week {
            vec.push("  ".to_string());
        }
        vec.push(format!("{:2}", weekday.day));
    } else {
        vec.push(format!("{:2}", weekday.day));
    }
}

fn complete_vec(vec: &mut Vec<String>) {
    let missing_days = 7 - vec.len();
    for _ in 0..missing_days {
        vec.push("  ".to_string());
    }
}

fn month_header(year: i32, month: u32, include_year: bool, endl: bool) {
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

    let header = if include_year {
        center_text(&format!("{} {}", month_as_string, year), LINE_LENGTH)
    } else {
        center_text(&format!("{}", month_as_string), LINE_LENGTH)
    };
    if endl {
        println!("{}", header);
    } else {
        print!("{}", header);
    }
}

fn print_vec(vec: &Vec<String>, endl: bool) {
    if endl {
        println!("{}  ", vec.join(" "));
    } else {
        print!("{}  ", vec.join(" "));
    }
}

fn print_padding(endl: bool) {
    let padding = " ".repeat(LINE_LENGTH);
    if endl {
        println!("{}", padding);
    } else {
        print!("{}", padding);
    }
}

fn get_weekday(year: i32, month: u32, day: i64) -> WkDay {
    let date = NaiveDate::from_ymd_opt(year, month, day as u32).unwrap();

    let weekday = match date.weekday() {
        Weekday::Sun => WkDay {
            day_in_week: 0,
            day: day,
        },
        Weekday::Mon => WkDay {
            day_in_week: 1,
            day: day,
        },
        Weekday::Tue => WkDay {
            day_in_week: 2,
            day: day,
        },
        Weekday::Wed => WkDay {
            day_in_week: 3,
            day: day,
        },
        Weekday::Thu => WkDay {
            day_in_week: 4,
            day: day,
        },
        Weekday::Fri => WkDay {
            day_in_week: 5,
            day: day,
        },
        Weekday::Sat => WkDay {
            day_in_week: 6,
            day: day,
        },
    };

    weekday
}

struct WkDay {
    day_in_week: i64,
    day: i64,
}

fn run(args: Args) -> Result<()> {
    let current_year = Local::now().year();
    let current_month = Local::now().month();
    let month = get_month(&args.month, current_month);

    if !(0 < month && month <= 12) {
        return Err(anyhow::anyhow!(
            "month \"{}\" not in the range 1 through 12",
            month
        ));
    }

    if !(0 < args.year && args.year <= 9999) && !args.show_year {
        return Err(anyhow::anyhow!(
            "error: invalid value \'{}\' for '[YEAR]': {} is not in 1..=9999",
            args.year,
            args.year
        ));
    }

    let year = if args.year == 0 {
        current_year
    } else {
        args.year
    };

    if args.month.is_some() {
        let days_in_month = days_in_month(year, month);

        month_header(year, month, true, true);
        day_headers(true);
        let mut vec = vec![];
        for day in 1..=days_in_month {
            let weekday = get_weekday(year, month, day);
            add_day_to_vec(&mut vec, weekday);
            if vec.len() == 7 {
                print_vec(&vec, true);
                vec.clear();
            }
        }

        if !vec.is_empty() {
            complete_vec(&mut vec);
            print_vec(&vec, true);
        }

        print_padding(true);
    } else if args.show_year || args.year != 0 {
        let header = center_text(&format!("{}  ", year), (3 * LINE_LENGTH) - 2);
        println!("{}  ", header);

        let calendar_layout = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
            vec![10, 11, 12],
        ];

        for (idx, month_vec) in calendar_layout.iter().enumerate() {
            for month in month_vec {
                let end_line = month % 3 == 0;
                month_header(year, *month, false, end_line);
            }
            for month in month_vec {
                let end_line = month % 3 == 0;
                day_headers(end_line);
            }

            let mut vec = vec![];

            let days_in_months = month_vec
                .iter()
                .map(|month| days_in_month(year, *month))
                .collect::<Vec<_>>();

            let dm1 = days_in_months[0];
            let dm2 = days_in_months[1];
            let dm3 = days_in_months[2];
            let mut cdm1 = 1;
            let mut cdm2 = 1;
            let mut cdm3 = 1;
            let mut weeks_printed = 0;
            while cdm1 <= dm1 || cdm2 <= dm2 || cdm3 <= dm3 {
                weeks_printed += 1;
                if cdm1 <= dm1 {
                    for _ in cdm1..=dm1 {
                        let weekday = get_weekday(year, month_vec[0], cdm1);
                        cdm1 += 1;
                        add_day_to_vec(&mut vec, weekday);
                        if vec.len() == 7 {
                            print_vec(&vec, false);
                            vec.clear();
                            break;
                        }
                    }

                    if !vec.is_empty() {
                        complete_vec(&mut vec);
                        print_vec(&vec, false);
                        vec.clear();
                    }
                } else {
                    print_padding(false);
                }

                if cdm2 <= dm2 {
                    for _ in cdm2..=dm2 {
                        let weekday = get_weekday(year, month_vec[1], cdm2);
                        cdm2 += 1;
                        add_day_to_vec(&mut vec, weekday);
                        if vec.len() == 7 {
                            print_vec(&vec, false);
                            vec.clear();
                            break;
                        }
                    }

                    if !vec.is_empty() {
                        complete_vec(&mut vec);
                        print_vec(&vec, false);
                        vec.clear();
                    }
                } else {
                    print_padding(false);
                }

                if cdm3 <= dm3 {
                    for _ in cdm3..=dm3 {
                        let weekday = get_weekday(year, month_vec[2], cdm3);
                        cdm3 += 1;
                        add_day_to_vec(&mut vec, weekday);
                        if vec.len() == 7 {
                            print_vec(&vec, true);
                            vec.clear();
                            break;
                        }
                    }

                    if !vec.is_empty() {
                        complete_vec(&mut vec);
                        print_vec(&vec, true);
                        vec.clear();
                    }
                } else {
                    print_padding(true);
                }
            }
            if weeks_printed < 6 {
                print_padding(false);
                print_padding(false);
                print_padding(true);
            }
            if idx < 3 {
                println!();
            }
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
