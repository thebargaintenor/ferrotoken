extern crate clap;

use clap::{App, Arg};
use image::open;

use ferrotoken::{color, token};

fn main() {
    let cli_args = App::new("Ferrotoken")
        .version("0.1.0")
        .author("Ted Monchamp (github.com/thebargaintenor)")
        .about("Overlays images into tokens")
        .arg(Arg::with_name("template")
            .short("t")
            .long("template")
            .help("Template image location")
            .required(true)
            .takes_value(true)
        )
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .help("Output image file location")
            .required(true)
            .takes_value(true)
        )
        .arg(Arg::with_name("mask")
            .short("m")
            .long("mask")
            .help("Masking channel color (default FF00FF)")
            .takes_value(true)
        )
        .arg(Arg::with_name("INPUT")
            .help("Input image file location")
            .index(1)
            .required(true)
        )
        .get_matches();

    let template_path = cli_args.value_of("template").unwrap();
    let output_path = cli_args.value_of("output").unwrap();
    let content_path = cli_args.value_of("INPUT").unwrap();
    
    let mask_hex_color = cli_args.value_of("mask").unwrap_or("#FF00FF");
    let mask = color::try_parse_rgba(mask_hex_color)
            .expect("Invalid mask color value provided.");

    let template = open(template_path)
        .expect("Invalid template path.")
        .into_rgba8();

    let mut content = open(content_path)
        .expect("Invalid content path.")
        .into_rgba8();

    match token::create(mask, template, &mut content) {
        Some(created_token) => {
            created_token.save(output_path).expect("Error while saving token.");
            println!("Token created successfully!");
        },
        None => println!("No mask channel found in template."),
    }
}
