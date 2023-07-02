use chrono::{Local, DateTime};
use clap::{Parser, Subcommand, ValueEnum};

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

    fn set() -> ! {
        unimplemented!()
    }
}

fn main() {
    let cli = Cli::parse();
    let now = Clock::get();

    match cli.action.unwrap_or(Commands::Get{std: OutputStandard::Rfc3339}) {
        Commands::Get{std} => 
            match std {
                OutputStandard::Timestamp => println!("{}", now.timestamp()),
                OutputStandard::Rfc2822 => println!("{}", now.to_rfc2822()),
                OutputStandard::Rfc3339 => println!("{}", now.to_rfc3339()),
            }
        Commands::Set{std: _, datetime: _} => unimplemented!(),
    }
}
