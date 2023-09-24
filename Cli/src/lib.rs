use clap::Parser;
use clap::{self};
use image::Image;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(value_parser = clap::builder::ValueParser::path_buf(), action = clap::ArgAction::Set, help = "Input file")]
    input: PathBuf,
    #[arg(
        short('i'),
        long("inverse"),
        default_value = "false",
        help = "Inverse image color"
    )]
    inverse: bool,
    #[arg(short, long, default_value = "false", help = "Use some demo image.bmp")]
    demo: bool,
    #[arg(
        short('l'),
        long("large"),
        default_value = "1",
        action = clap::ArgAction::Count,
        help = "Show a image that is larger then default"
    )]
    large: u8,
    #[arg(short, long, value_parser = validate_size, default_value = "80x40", action = clap::ArgAction::Set, help = "Set a specific size of a image" )]
    size: (usize, usize),
    #[arg(short, long, value_parser = clap::builder::ValueParser::path_buf(), action = clap::ArgAction::Set, help = "Output file")]
    output: Option<PathBuf>,
}
fn validate_size(s: &str) -> Result<(usize, usize), String> {
    return match s.split_once('x') {
        Some((w, h)) => Ok((
            w.parse::<usize>()
                .map_err(|_| format!("Width is not a number"))?,
            h.parse::<usize>()
                .map_err(|_| format!("Height is not a number"))?,
        )),
        None => Err(format!("No value")),
    };
}
pub fn run() {
    let cli = Cli::parse();
    let color_mode = cli.inverse;
    let path = match (cli.demo, cli.input) {
        (true, _) => Image::demo_file(),
        (false, file) => file,
    };
    let (width, height) = (
        cli.size.0 * cli.large as usize,
        cli.size.1 * cli.large as usize,
    );
    let image = Image::new(path).resize(width, height);
    match cli.output {
        Some(file) => image.save(file, color_mode).expect("Could't write a file"),
        None => image.draw(color_mode),
    }
}
#[cfg(test)]
mod tests {
    use crate::validate_size;

    #[test]
    fn no_x() {
        assert!(validate_size("123s123").is_err())
    }
    #[test]
    fn invalid_number() {
        assert!(validate_size("12ax123").is_err());
        assert!(validate_size("12x12b").is_err());
        assert!(validate_size("123x-123").is_err());
        assert!(validate_size("-123x123").is_err());
        assert!(validate_size("123x123x5").is_err());
    }
    #[test]
    fn valid_size() {
        assert_eq!(validate_size("123x123").unwrap(), (123, 123));
        assert_eq!(validate_size("123x123").unwrap(), (123, 123))
    }
}
