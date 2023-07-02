#[cfg(windows)]
use kernel32;
#[cfg(windows)]
use winapi;
#[cfg(not(windows))]
use libc;

use chrono::{DateTime, Local, TimeZone};
use clap::{Parser, Subcommand, ValueEnum};
use std::mem::zeroed;

#[derive(Parser)]
#[command(name = "clock")]
#[command(about = "Gets and (aspirationally) sets the time.")]
#[command(version = "0.1")]
struct Cli {
    #[command(subcommand)]
    action: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Get {
    #[arg(short = 's')]
    #[arg(long = "use-standard")]
    #[arg(value_enum)]
    #[arg(default_value_t = OutputStandard::Rfc3339)]
    std: OutputStandard,
    },
    Set {
    #[arg(short = 's')]
    #[arg(long = "use-standard")]
    #[arg(value_enum)]
    #[arg(default_value_t = OutputStandard::Rfc3339)]
    std: OutputStandard,
    #[arg(help = "When <action> is 'set', apply <datetime>. Otherwise, ignore.")]
    datetime: Option<String>
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum OutputStandard {
    Timestamp,
    Rfc2822,
    Rfc3339,
}

impl std::fmt::Display for OutputStandard {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "{self:?}")
   }
}

struct Clock;
impl Clock {
    fn get() -> DateTime<Local> {
        Local::now()
    }

    #[cfg(windows)]
    fn set<Tz: TimeZone>(t: DateTime<Tz>) {
        use chrono::Weekday;
        use kernel32::SetSystemTime;
        use winapi::{SYSTEMTIME, WORD};

        let t = t.with_timezone(&Local);
        let mut systime: SYSTEMTIME = unsafe { zeroed() };
        let dow = match t.weekday() {
            Weekday::Mon => 1,
            Weekday::Tue => 2,
            Weekday::Wed => 3,
            Weekday::Thu => 4,
            Weekday::Fri => 5,
            Weekday::Sat => 6,
            Weekday::Sun => 0,
        };
        let mut ns = t.nanosecond();
        let is_leap_second = ns > 1_000_000_000;

        if is_leap_second {
            ns -= 1_000_000_000;
        }

        systime.wYear = t.year() as WORD;
        systime.wMonth = t.month() as WORD;
        systime.wDayOfWeek = dow as WORD;
        systime.wDay = t.day() as WORD;
        systime.wHour = t.hour() as WORD;
        systime.wMinute = t.minute() as WORD;
        systime.wSecond = t.second() as WORD;
        systime.wMillisecond = (ns / 1_000_000) as WORD;

        let systime_ptr = &systime as *const SYSTEMTIME;

        unsafe {
            SetSystemTime(systime_ptr);
        }
    }

    fn set<Tz: TimeZone>(t: DateTime<Tz>) {
        use libc::{timeval, time_t, suseconds_t};
        use libc::{settimeofday, timezone};

        let t = t.with_timezone(&Local);
        let mut u: timeval = unsafe { zeroed() };

        u.tv_sec = t.timestamp() as time_t;
        u.tv_usec = t.timestamp_subsec_micros() as suseconds_t;

        unsafe {
            let mock_tz: *const timezone = std::ptr::null();
            settimeofday(&u as *const timeval, mock_tz);
        }
    }
}

pub fn main() {
    let cli = Cli::parse();

    match cli.action.unwrap_or(Commands::Get{std: OutputStandard::Rfc3339}) {
        Commands::Get{std} => {
            let now = Clock::get();
            match std {
                OutputStandard::Timestamp => println!("{}", now.timestamp()),
                OutputStandard::Rfc2822 => println!("{}", now.to_rfc2822()),
                OutputStandard::Rfc3339 => println!("{}", now.to_rfc3339()),
            }
        }
        Commands::Set{std, datetime} => {
            let datetime = datetime.unwrap();
            let parser = match std {
                OutputStandard::Rfc2822 => DateTime::parse_from_rfc2822,
                OutputStandard::Rfc3339 => DateTime::parse_from_rfc2822,
                _ => unimplemented!(),
            };

            let err_msg = format!("Unable to parse {datetime} according to {std}");
            let t = parser(&datetime).expect(&err_msg);

            Clock::set(t)
        }
    }
}
