use clap::{Parser, ValueEnum};
use wake_up_neo::{Color, MatrixEasterEgg};

#[derive(Parser)]
#[command(about, author, version)]
struct Cli {
    /// name used in message. (Default "Neo")
    #[arg(short, long)]
    name: Option<String>,

    /// message to print to console
    #[arg(short, long, num_args = 1..)]
    message: Option<Vec<String>>,

    /// character color
    #[arg(short, long)]
    color: Option<TerminalColor>,

    /// background color
    #[arg(short, long)]
    bg_color: Option<TerminalColor>,

    /// Ignore Ctrl-C (SIGINT)
    #[arg(short, long)]
    ignore_ctrlc: bool,

    /// display delay in milliseconds
    #[arg(
        short, long,
        num_args = 2,
        value_names = ["MIN", "MAX"])]
    delay: Option<Vec<u32>>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum TerminalColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    // Color256(u8),
}

impl From<TerminalColor> for Color {
    fn from(val: TerminalColor) -> Self {
        match val {
            TerminalColor::Black => Color::Black,
            TerminalColor::Red => Color::Red,
            TerminalColor::Green => Color::Green,
            TerminalColor::Yellow => Color::Yellow,
            TerminalColor::Blue => Color::Blue,
            TerminalColor::Magenta => Color::Magenta,
            TerminalColor::Cyan => Color::Cyan,
            TerminalColor::White => Color::White,
            // TerminalColor::Color256(a) => Color::Color256(a),
        }
    }
}

fn main() {
    let args = Cli::parse();
    if args.ignore_ctrlc {
        ctrlc::set_handler(|| {}).expect("Error setting Ctrl-C handler");
    }
    let mut neo = MatrixEasterEgg::default();
    if let Some(ref name) = args.name {
        neo.set_name(name.clone());
    }
    if let Some(ref message) = args.message {
        neo.set_sentences(message.clone());
    }
    if let Some(color) = args.color {
        neo.foreground(color.into());
    }
    if let Some(color) = args.bg_color {
        neo.background(color.into());
    }
    if let Some(ref delay) = args.delay {
        let (mut min, mut max) = (delay[0], delay[1]);
        if max < min {
            std::mem::swap(&mut min, &mut max);
        }
        neo.set_delay((min, max));
    }
    neo.draw().unwrap();
}
