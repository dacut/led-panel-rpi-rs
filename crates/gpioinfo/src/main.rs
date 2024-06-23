use {
    clap::{Parser, Subcommand},
    gpio_linux_char::Gpio,
    std::{error::Error, path::Path, process::ExitCode},
};

#[derive(Parser)]
#[command(version, about, long_about = "A tool to get information about GPIO chips and lines.")]
struct Cli {
    #[clap(subcommand)]
    subcommand: Option<GpioInfoSubcommand>,
}

#[derive(Subcommand)]
enum GpioInfoSubcommand {
    /// Lists the available lines for a chip (or all chips).
    Lines {
        /// The chip to get lines from. This can be a full path to the `/dev/gpiochipN` device, a relative path
        /// (with `/dev/` assumed), or a chip number.
        #[arg(short, long)]
        chip: Option<String>,
    },

    /// Lists the available chips.
    Chips,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let subcommand = cli.subcommand.unwrap_or(GpioInfoSubcommand::Chips);
    let result = match subcommand {
        GpioInfoSubcommand::Chips => handle_chips(),
        GpioInfoSubcommand::Lines {
            chip,
        } => handle_lines(chip),
    };

    match result {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::FAILURE
        }
    }
}

fn handle_chips() -> Result<(), Box<dyn Error>> {
    let chips = Gpio::list_chips()?;
    println!("Chip                 Name             Label                    Lines");
    for chip in chips {
        let gpio = Gpio::open(&chip)?;
        let info = gpio.get_chip_info()?;
        println!("{:<20} {:<16} {:<24} {:>5}", chip.to_string_lossy(), info.name, info.label, info.lines);
    }

    Ok(())
}

fn handle_lines(chip: Option<String>) -> Result<(), Box<dyn Error>> {
    if let Some(chip) = chip {
        let desc = Gpio::parse_chip_descriptor(&chip)?;
        handle_lines_for_chip(&desc)?;
    } else {
        let chips = Gpio::list_chips()?;
        for desc in chips {
            handle_lines_for_chip(&desc)?;
        }
    }

    Ok(())
}

fn handle_lines_for_chip(chip: &Path) -> Result<(), Box<dyn Error>> {
    let gpio = Gpio::open(chip)?;
    let chip_info = gpio.get_chip_info()?;

    println!("Chip: {}", chip.to_string_lossy());
    println!("    Line   Offset Name                 Consumer             Flags");
    for line in 0..chip_info.lines {
        let info = match gpio.get_line_info(line) {
            Ok(info) => info,
            Err(e) => {
                eprintln!("Error getting line info for line {}: {}", line, e);
                return Err(e.into());
            }
        };

        println!("    {:>6} {:>6} {:<20} {:<20} {}", line, info.offset, info.name, info.consumer, info.flags);
    }

    Ok(())
}
