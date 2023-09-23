use std::error::Error;
use std::fmt::format;

use clap::error::ErrorKind;
use clap::CommandFactory;
use clap::Parser;
use clap::{self};
use image::Image;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(action = clap::ArgAction::Set, help = "Input file")]
    file: Option<String>,
    #[arg(
        short('i'),
        long("inverse"),
        default_value = "false",
        help = "Inverse image color"
    )]
    is_inverse: bool,
    #[arg(short, long, default_value = "false", help = "Use some demo image.bmp")]
    demo: bool,
    #[arg(
        short('l'),
        long("large"),
        default_value = "false",
        help = "Show a image that is larger then default"
    )]
    is_large: bool,
    #[arg(short, long, value_parser = validate_size, default_value = "80x40", action = clap::ArgAction::Set, help = "Set a specific size of a image" )]
    size: (usize, usize),
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

fn main() {
    let cli = Cli::parse();
    let color_mode = cli.is_inverse;
    let path = match (cli.demo, cli.file) {
        (false, file) => file.unwrap_or_else(|| {
            todo!();
            Cli::command()
                .error(
                    ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand,
                    "No file specified",
                )
                .exit()
        }),
        (true, _) => Image::demo_file(),
    };
    let (width, height) = match (cli.is_large, cli.size) {
        (true, _) => (150, 100),
        (false, size) => size,
    };
    let image = Image::new(path.as_str()).resize(width, height);
    image.draw(color_mode);
}
