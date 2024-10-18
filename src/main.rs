pub mod utils;
pub mod steg;

use std::process::exit;

use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug, Clone, PartialEq)]
enum Action {
    /// Hide something in an existing image provided in --input-image flag
    Hide,
    /// Reveal something in an existing image provided in --input-image flag
    Reveal
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of the image you want to hide/reveal something
    #[arg(short, long)]
    input_image: String,

    /// Path of the resulting image after hiding/revealing something
    #[arg(short, long)]
    output_image: String,

    /// If you want to hide a text or an image
    #[arg(short = 't', long, default_value_t = true)]
    is_text: bool,

    /// The text you want to hide
    #[arg(short = 's', long, default_value_t = String::new())]
    text: String,

    /// Number of characters you want to reveal
    #[arg(short = 'n', long, default_value_t = 16)]
    n_characters: u32,
    
    /// Action you want to take (hide/reveal)
    #[command(subcommand)]
    action: Action
}

fn main() {
    let args = Args::parse();

    if args.is_text && args.text.len() == 0 && args.action == Action::Hide {
        println!("It's not possible to hide an empty string. Consider using --text <TEXT> properly.");
        exit(1);
    }

    let input_image = utils::fs::load_image(&args.input_image);
    let img: image::DynamicImage;
    match input_image {
        Ok(i) => img = i,
        Err(e) => {
            eprintln!("{e}");
            exit(1);
        }
    }

    match args.action {
        Action::Hide => {
            let bytes = utils::bits::bytes_to_bits(args.text.as_bytes());
            let result = steg::hide(&img, &bytes);

            match result {
                Ok(res_img) => {
                    utils::fs::save_image(res_img, &args.output_image);
                },
                Err(e) => {
                    eprintln!("{e}");
                    exit(1);
                }           
            }

        },
        Action::Reveal => {
            let result = steg::reveal(&img, args.n_characters);

            match result {
                Ok(res) => {
                    println!("{res}");
                },
                Err(e) => {
                    eprintln!("{e}");
                    exit(1);
                }           
            }
        }
    }
}