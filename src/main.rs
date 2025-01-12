use argh::FromArgs;
use chrono::prelude::*;
use locale_config::Locale;

use rusti_cal::{display, display_colored};

#[derive(FromArgs, PartialEq, Debug)]
/// A command with positional arguments.
struct WithPositional {
    #[argh(positional, default = "default_year()")]
    year: u32,

    /// an optional starting_day which is "0" by default
    #[argh(option, default = "0")]
    starting_day: u32,

    /// an optional flag for enabling colored output
    #[argh(switch, short = 'c')]
    color: bool,
}

fn default_year() -> u32 {
    let now = Local::now();
    let (_, year) = now.year_ce();
    year
}

fn locale() -> String {
    let locale = Locale::user_default();
    locale
        .tags()
        .next()
        .map(|(_, x)| x.to_string().replace("-", "_"))
        .unwrap_or_default()
}

fn main() {
    let arg = argh::from_env::<WithPositional>();
    if arg.color {
        display_colored(arg.year, &locale(), arg.starting_day);
    } else {
        display(arg.year, &locale(), arg.starting_day);
    }
}
