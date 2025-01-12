mod locale;

use ansi_term::{
    Color::{Black, Cyan, Green, Red, Yellow},
    Style,
};
use chrono::Datelike;

const REFORM_YEAR: u32 = 1099;
const MONTHS: usize = 12;
const WEEKDAYS: u32 = 7;

const COLUMN: usize = 3;
const ROWS: usize = 4;
const ROW_SIZE: usize = 7;

static TOKEN: &str = "\n";

fn is_leap_year(year: u32) -> bool {
    if year <= REFORM_YEAR {
        return year % 4 == 0;
    }
    (year % 4 == 0) ^ (year % 100 == 0) ^ (year % 400 == 0)
}

fn days_by_year(mut year: u32) -> u32 {
    let mut count: u32 = 0;

    while year > 1 {
        year -= 1;
        if is_leap_year(year) {
            count += 366
        } else {
            count += 365
        }
    }
    count
}

fn days_by_month(year: u32) -> Vec<u32> {
    let mut feb_day: u32 = 28;

    if is_leap_year(year) {
        feb_day = 29;
    }
    vec![0, 31, feb_day, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
}

fn days_by_date(
    day: u32,
    month: usize,
    year: u32,
    months_memoized: Vec<u32>,
    year_memoized: u32,
) -> u32 {
    let mut count = 0;

    count += day;
    if month > 1 {
        count += months_memoized[month - 1]
    }
    if year > 1 {
        count += year_memoized
    }
    count
}

fn get_days_accumulated_by_month(year: u32) -> (Vec<u32>, Vec<u32>) {
    let mut count = 0;
    let mut accum = Vec::new();
    let days: Vec<u32> = days_by_month(year);

    (0..MONTHS + 1).for_each(|i| {
        count += days[i];
        accum.push(count);
    });
    (accum, days)
}

fn first_day_printable(day_year: u32, starting_day: u32) -> String {
    let mut spaces: String = "".to_string();
    let mut printable = format!("");

    if (day_year - starting_day) % WEEKDAYS == 0 {
        printable.push_str("                  ");
    }
    for i in 2..WEEKDAYS {
        spaces += &"   ".to_string();
        if (day_year - starting_day) % WEEKDAYS == i {
            printable.push_str(spaces.as_str());
            break;
        }
    }
    printable
}

fn remain_day_printable(day: u32, day_year: u32, starting_day: u32) -> String {
    let base = if ((day_year - starting_day) % WEEKDAYS) == 0 {
        format!("{:3}{}", day, TOKEN)
    } else {
        String::default()
    };

    let complement = (1..WEEKDAYS)
        .find_map(|i| ((day_year - starting_day) % WEEKDAYS == i).then(|| format!("{:3}", day)))
        .unwrap_or_default();

    format!("{}{}", base, complement)
}

fn body_printable(
    year: u32,
    month: usize,
    days: u32,
    months_memoized: Vec<u32>,
    year_memoized: u32,
    starting_day: u32,
) -> Vec<String> {
    let mut result = Vec::<String>::new();
    let mut result_days = format!("");

    // display month formatted
    (1..days + 1).for_each(|day| {
        if day == 1 {
            let first_day = days_by_date(1, month, year, months_memoized.clone(), year_memoized);
            result_days.push_str(&first_day_printable(first_day, starting_day))
        }
        let day_year = days_by_date(day, month, year, months_memoized.clone(), year_memoized);
        result_days.push_str(&remain_day_printable(day, day_year, starting_day))
    });

    // lines splitted by '\n' TOKEN
    result_days
        .split(TOKEN)
        .collect::<Vec<&str>>()
        .into_iter()
        .for_each(|i| result.push(i.to_string()));

    // all body should have at least 6 lines
    // plz refactor me
    let len = result.len();
    if len <= 6 {
        let spaces = 21 - result[len - 1].len();
        if result[len - 1].len() < 20 {
            for _i in 0..spaces {
                result[len - 1] += " "
            }
        }
        result.push("                     ".to_string())
    }
    result
}

fn month_printable(
    year: u32,
    month: usize,
    days: u32,
    months_memoized: Vec<u32>,
    year_memoized: u32,
    starting_day: u32,
    month_names: Vec<String>,
    week_names: Vec<String>,
) -> Vec<String> {
    let mut result = Vec::<String>::new();
    let body = body_printable(
        year,
        month,
        days,
        months_memoized,
        year_memoized,
        starting_day,
    );
    let month_name = &month_names[month - 1];
    result.push(format!(" {:^20}", month_name));
    let header = circular_week_name(week_names, starting_day as usize);
    result.push(header);

    body.into_iter().for_each(|item| {
        result.push(item);
    });
    result
}

fn circular_week_name(week_name: Vec<String>, idx: usize) -> String {
    let mut s = " ".to_string();
    let mut i = idx;

    while i < ROW_SIZE + idx {
        if i == (ROW_SIZE - 1) + idx {
            s.push_str(week_name[i % ROW_SIZE].as_str());
        } else {
            s.push_str(&format!("{} ", week_name[i % ROW_SIZE]));
        }
        i += 1
    }
    s.to_string()
}

pub fn calendar(year: u32, locale_str: &str, starting_day: u32) -> Vec<Vec<Vec<String>>> {
    let mut rows: Vec<Vec<Vec<String>>> = vec![vec![vec![String::from("")]; COLUMN]; ROWS];
    let mut row_counter = 0;
    let mut column_counter = 0;
    let (months_memoized, months) = get_days_accumulated_by_month(year);
    let year_memoized = days_by_year(year);
    let locale_info = locale::LocaleInfo::new(locale_str);

    (1..MONTHS + 1).for_each(|month| {
        rows[row_counter][column_counter] = month_printable(
            year,
            month,
            months[month],
            months_memoized.clone(),
            year_memoized,
            starting_day,
            locale_info.month_names(),
            locale_info.week_day_names(),
        );
        column_counter = month % COLUMN;
        if column_counter == 0 {
            row_counter += 1;
        }
    });
    rows
}

fn print_colored_row(row: &str, starting_day: u32, today_included: bool, pos_today: u32) {
    let pos_saturday = (((6 - starting_day as i32) % 7) + 7) % 7;
    let pos_sunday = (((7 - starting_day as i32) % 7) + 7) % 7;

    let char_saturday = (1 + 3 * pos_saturday) as usize;
    let char_sunday = (1 + 3 * pos_sunday) as usize;
    let char_today = (1 + 3 * pos_today) as usize;

    let row = row
        .split("")
        .filter(|s| !s.is_empty())
        .enumerate()
        .map(|(i, s)| {
            if today_included && (i == char_today || i == char_today + 1) {
                Black.on(Green).paint(s)
            } else if i == char_saturday || i == char_saturday + 1 {
                Yellow.bold().paint(s)
            } else if i == char_sunday || i == char_sunday + 1 {
                Red.bold().paint(s)
            } else {
                ansi_term::Style::default().paint(s)
            }
        })
        .collect::<Vec<ansi_term::ANSIString>>();

    print!("{} ", ansi_term::ANSIStrings(&row));
}

/// calculates the positions of the given day within the overall grid
///
/// Returns a tuple
///
/// (month row, month column, day x position, line of month)
fn get_today_position(year: u32, month: u32, day: u32, starting_day: u32) -> (u32, u32, u32, u32) {
    let (months_memoized, _) = get_days_accumulated_by_month(year);

    let first_of_month = days_by_date(1, month as usize, year, months_memoized, days_by_year(year));
    let first_offset = (first_of_month - starting_day - 1) % WEEKDAYS;

    let row_index = (month - 1) / 3;
    let col_index = (month - 1) % 3;

    let absolute_pos = first_offset + day - 1;
    let x = absolute_pos % 7;
    let y = absolute_pos / 7;

    (row_index, col_index, x, y)
}

pub fn display_colored(year: u32, locale_str: &str, starting_day: u32) {
    let rows = calendar(year, locale_str, starting_day);

    let today = {
        let now = chrono::Local::now();
        (now.year() as u32, now.month(), now.day())
    };

    let t_pos = if today.0 == year {
        Some(get_today_position(today.0, today.1, today.2, starting_day))
    } else {
        None
    };

    // print the year
    println!("{}", Style::new().bold().paint(format!(" {:^63}", year)));

    for (r, row) in rows.iter().enumerate() {
        for line in 0..8 {
            for col in 0..3 {
                if line == 0 {
                    print!("{} ", Cyan.bold().paint(&row[col][line]));
                } else {
                    // check if today is part of this line
                    let (today_included, x) = {
                        if let Some(p) = t_pos {
                            // check for the correct row, col and line
                            if p.0 == r as u32 && p.1 == col as u32 && p.3 + 2 == line as u32 {
                                (true, p.2)
                            } else {
                                (false, 0)
                            }
                        } else {
                            (false, 0)
                        }
                    };
                    // print the colred line
                    print_colored_row(&row[col][line], starting_day, today_included, x);
                }
            }
            println!();
        }
    }
}

pub fn display(year: u32, locale_str: &str, starting_day: u32) {
    let rows = calendar(year, locale_str, starting_day);
    println!(" {:^63}", year);
    for row in rows {
        for i in 0..8 {
            for j in 0..3 {
                print!("{} ", &row[j][i]);
            }
            println!();
        }
    }
}

#[test]
fn test_circular_week_name() {
    let locale_str = "en_US";
    let locale_info = locale::LocaleInfo::new(locale_str);
    let week_name = locale_info.week_day_names();
    assert_eq!(circular_week_name(week_name, 0), " Su Mo Tu We Th Fr Sa");
}

#[test]
fn test_circular_week_name_pt_br() {
    let locale_str = "pt_BR";
    let locale_info = locale::LocaleInfo::new(locale_str);
    let week_name = locale_info.week_day_names();
    assert_eq!(circular_week_name(week_name, 0), " Do Se Te Qu Qu Se Sá");
}
