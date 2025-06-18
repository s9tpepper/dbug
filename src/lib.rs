use core::fmt::{self, write};
use std::{
    cell::Cell,
    fmt::Arguments,
    hash::{DefaultHasher, Hash, Hasher},
    time::Instant,
};

const COLORS: [&str; 76] = [
    "#0000CC", "#0000FF", "#0033CC", "#0033FF", "#0066CC", "#0066FF", "#0099CC", "#0099FF",
    "#00CC00", "#00CC33", "#00CC66", "#00CC99", "#00CCCC", "#00CCFF", "#3300CC", "#3300FF",
    "#3333CC", "#3333FF", "#3366CC", "#3366FF", "#3399CC", "#3399FF", "#33CC00", "#33CC33",
    "#33CC66", "#33CC99", "#33CCCC", "#33CCFF", "#6600CC", "#6600FF", "#6633CC", "#6633FF",
    "#66CC00", "#66CC33", "#9900CC", "#9900FF", "#9933CC", "#9933FF", "#99CC00", "#99CC33",
    "#CC0000", "#CC0033", "#CC0066", "#CC0099", "#CC00CC", "#CC00FF", "#CC3300", "#CC3333",
    "#CC3366", "#CC3399", "#CC33CC", "#CC33FF", "#CC6600", "#CC6633", "#CC9900", "#CC9933",
    "#CCCC00", "#CCCC33", "#FF0000", "#FF0033", "#FF0066", "#FF0099", "#FF00CC", "#FF00FF",
    "#FF3300", "#FF3333", "#FF3366", "#FF3399", "#FF33CC", "#FF33FF", "#FF6600", "#FF6633",
    "#FF9900", "#FF9933", "#FFCC00", "#FFCC33",
];

fn xterm_color_index_for_string(input: &str) -> u8 {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    let hex_index = (hasher.finish() % COLORS.len() as u64) as usize;
    let hex = COLORS[hex_index];
    let ansi_color = ansi_256_from_hex(hex);

    ansi_color.unwrap_or(123)
}

fn colorize(color: u8, prefix: &str) -> String {
    format!("\x1b[1;38;5;{}m{}\x1b[0m", color, prefix)
}

fn ansi_256_from_hex(hex: &str) -> Result<u8, Box<dyn std::error::Error>> {
    let hex = hex.trim_start_matches('#');

    let r = u8::from_str_radix(&hex[0..2], 16)?;
    let g = u8::from_str_radix(&hex[2..4], 16)?;
    let b = u8::from_str_radix(&hex[4..6], 16)?;

    Ok(rgb_to_ansi256(r, g, b))
}

fn rgb_to_ansi256(r: u8, g: u8, b: u8) -> u8 {
    // Grayscale approximation
    if r == g && g == b {
        if r < 8 {
            return 16;
        }
        if r > 248 {
            return 231;
        }
        return 232 + ((r as u16 - 8) * 24 / 247) as u8;
    }

    // RGB 6x6x6 Cube (16–231)
    let r = scale_to_ansi(r);
    let g = scale_to_ansi(g);
    let b = scale_to_ansi(b);

    16 + 36 * r + 6 * g + b
}

fn scale_to_ansi(value: u8) -> u8 {
    match value {
        0..=47 => 0,
        48..=114 => 1,
        115..=154 => 2,
        155..=194 => 3,
        195..=234 => 4,
        _ => 5,
    }
}

fn parse_filter() -> Vec<String> {
    match std::env::var("DEBUG") {
        Ok(debug) if debug.contains(" ") => debug
            .split(" ")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect(),

        Ok(debug) if debug.contains(",") => debug
            .split(",")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect(),

        Ok(debug) => vec![debug],

        Err(_) => vec![],
    }
}

pub struct Logger {
    raw_label: String,
    label: String,
    filter: Vec<String>,
    color: u8,
    last_log: Cell<Option<Instant>>,
}

#[macro_export]
macro_rules! dbug {
    ($logger:expr, $($arg:tt)*) => {
        $logger.log_fmt(format_args!($($arg)*))
    }
}

impl Logger {
    pub fn new(label: &str) -> Self {
        let raw_label = label.to_string();
        let color = xterm_color_index_for_string(&raw_label);
        let label = colorize(color, &raw_label);
        let filter = parse_filter();

        Logger {
            color,
            raw_label,
            label,
            filter,
            last_log: None.into(),
        }
    }

    pub fn log_fmt(&self, args: Arguments) {
        let mut msg = String::new();
        let _ = write(&mut msg, args);

        self.log(&msg);
    }

    pub fn log(&self, message: &str) {
        if !self.should_log() {
            return;
        }

        let ms_diff = if let Some(last) = self.last_log.get() {
            let time = Instant::now();
            let elapsed = (time - last).as_millis();

            colorize(self.color, &format!("+{}", elapsed))
        } else {
            colorize(self.color, "+0")
        };

        println!("{} {} {}", self.label, message, ms_diff);

        self.last_log.set(Some(Instant::now()));
    }

    pub fn extend(&self, extension: &str) -> Logger {
        Logger::new(&format!("{}:{}", self.raw_label, extension))
    }

    pub fn to_closure(&self) -> impl Fn(&str) {
        |message: &str| {
            self.log(message);
        }
    }

    fn should_log(&self) -> bool {
        // handle negations, -somelabel and -somelabel*
        for filter in &self.filter {
            if filter.starts_with("-") && !filter.ends_with("*") && filter[1..] == self.raw_label {
                return false;
            }

            if filter.starts_with("-")
                && filter.ends_with("*")
                && self.raw_label.starts_with(&filter[1..filter.len() - 1])
            {
                return false;
            }
        }

        for filter in &self.filter {
            if self.raw_label == *filter
                || (filter.ends_with("*")
                    && self.raw_label.starts_with(&filter[0..filter.len() - 1]))
                || filter == "*"
            {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
