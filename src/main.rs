use clap::{Parser, ValueEnum};
use wake_up_neo::{MatrixEasterEgg, Color};

#[derive(Parser)]
#[command(about, author, version)]
struct Cli {
    /// name used in message. (Default "Neo")
    #[arg(short, long)]
    name: Option<String>,

    /// message to print to console
    #[arg(short, long)]
    message: Option<Vec<String>>,

    /// character color
    #[arg(short, long)]
    color: Option<TerminalColor>,

    /// background color
    #[arg(short, long)]
    bg_color: Option<TerminalColor>,
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
    let mut neo = MatrixEasterEgg::default();
    if let Some(ref name) = args.name {
        neo.set_name(name.clone());
    }
    if let Some(ref message) = args.message {
        neo.set_sentences(message.clone());
    }
    neo.draw().unwrap();
}
