use std::{io::Write, thread, time::Duration};

pub use console::Color;
use console::{Style, Term};
use rand::{
    distributions::uniform::{SampleRange, SampleUniform},
    rngs::ThreadRng,
    Rng,
};

#[derive(Debug)]
pub struct MatrixEasterEgg {
    sentences: Vec<String>,
    name: String,
    style: Style,
    /// delay range (min, max) [ms]
    delay: (u32, u32),
}

impl Default for MatrixEasterEgg {
    fn default() -> Self {
        const DEFAULT: [&str; 4] = [
            "Wake up, {name}...",
            "The Matrix has you...",
            "Follow the white rabbit.",
            "Knock, knock, {name}.",
        ];

        Self {
            sentences: DEFAULT.into_iter().map(|a| a.to_owned()).collect(),
            name: "Neo".to_owned(),
            style: Style::new().green(),
            delay: (200, 325),
        }
    }
}

// #[derive(Copy, Clone, Debug, PartialEq, Eq, Arg)]
// pub enum TerminalColor {
//     Black,
//     Red,
//     Green,
//     Yellow,
//     Blue,
//     Magenta,
//     Cyan,
//     White,
//     Color256(u8),
// }

// impl From<TerminalColor> for Color {
//     fn from(val: TerminalColor) -> Self {
//         match val {
//             TerminalColor::Black => Color::Black,
//             TerminalColor::Red => Color::Red,
//             TerminalColor::Green => Color::Green,
//             TerminalColor::Yellow => Color::Yellow,
//             TerminalColor::Blue => Color::Blue,
//             TerminalColor::Magenta => Color::Magenta,
//             TerminalColor::Cyan => Color::Cyan,
//             TerminalColor::White => Color::White,
//             TerminalColor::Color256(a) => Color::Color256(a),
//         }
//     }
// }

impl MatrixEasterEgg {
    /// Empty setting.
    /// If you want display "The Matrix" version, use `Default::default()`
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_params(
        sentences: Vec<String>,
        name: String,
        style: Style,
        delay: (u32, u32),
    ) -> Self {
        Self {
            sentences,
            name,
            style,
            delay,
        }
    }

    pub fn set_sentences(&mut self, sentences: Vec<String>) {
        self.sentences = sentences;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn foreground(&mut self, color: Color) {
        self.style = self.style.clone().fg(color);
    }

    pub fn background(&mut self, color: Color) {
        self.style = self.style.clone().bg(color);
    }

    pub fn draw(&self) -> std::io::Result<()> {
        let mut rng = rand::thread_rng();
        let a = self.delay.0..self.delay.1;
        let mut term = Term::stdout();
        term.clear_screen()?;
        Self::sleep_range_millis(&mut rng, a.clone(), 1);
        for sen in &self.sentences {
            let sen = sen.replace("{name}", &self.name);
            for c in sen.chars() {
                Self::sleep_range_millis(&mut rng, a.clone(), 1);
                // {
                //     let dur = rng.gen_range(a.clone());
                //     let dur = Duration::from_millis(dur as u64);
                //     thread::sleep(dur);
                //     let styled = self.style.map_or(c, |a| a.apply_to(c));
                // }
                write!(&mut term, "{}", self.style.apply_to(c))?;
            }
            Self::sleep_range_millis(&mut rng, a.clone(), 6);
            term.clear_line()?;
        }
        Ok(())
    }

    #[inline(always)]
    fn sleep_range_millis<R, T>(rng: &mut ThreadRng, range: R, scale: u32)
    where
        R: SampleRange<T>,
        T: SampleUniform + Into<u64>,
    {
        let mut dur = rng.gen_range(range).into();
        dur *= scale as u64;
        let dur = Duration::from_millis(dur);
        thread::sleep(dur);
    }
}
