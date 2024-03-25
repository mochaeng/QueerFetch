use std::process;

use clap::Parser;
use flags::FlagRepository;
use system::SysRepository;

mod colors;
mod flags;
mod system;

#[derive(Parser)]
#[command(about = "QueerFetch displays system information alongside your favorite LGBTQ+ flag", long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FLAG")]
    flag: Option<String>,
}

fn main() {
    let args = Cli::parse();
    let flag_name: &str = &args.flag.unwrap_or("transgender".to_string());

    let flag_name = match flag_name {
        "pride" => "lgbt",
        "trans" => "transgender",
        "ace" => "assexual",
        "nb" => "nonbinary",
        "bi" => "bisexual",
        "pan" => "pansexual",
        "fluid" => "genderfluid",
        "queer" => "genderqueer",
        _ => flag_name,
    };

    let flag_repository = FlagRepository::new();
    let mut sys_repository = SysRepository::new();

    let flag = match flag_repository.get_flag(flag_name) {
        Some(value) => value,
        None => {
            println!("No {} flag was found", flag_name);
            process::exit(1);
        }
    };

    let host_color = &flag.layers[0];
    let title_color = &flag.layers[1];
    let dash_color = &flag.layers[2];

    let reset = "\x1b[0m\x1b[39m";
    let reset_foreground = "\x1b[39m";
    let reset_background = "\x1b[49m";
    let width = 35;

    let lines = if flag.layers.len() == 3 { 3 } else { 2 };

    let mut idx = 0;
    flag.layers.iter().for_each(|rgb| {
        for _ in 0..lines {
            let mut line = String::new();

            line.push_str(&colors::get_color_line(width, rgb));
            line.push_str(reset_background);

            if let Some(func) = sys_repository.get_func() {
                let info = func();
                let title_line = match idx {
                    0 => format!("{} {} {}", colors::get_color_text(host_color), info, reset),
                    1 => format!("{} {} {}", colors::get_color_text(dash_color), info, reset),
                    _ => format!("{} {} {}", colors::get_color_text(title_color), info, reset),
                };
                line.push_str(&title_line);
            }

            line.push_str(reset_background);
            line.push_str("\n");

            print!("{}{}", line, reset_foreground);
            idx += 1;
        }
    });
}
