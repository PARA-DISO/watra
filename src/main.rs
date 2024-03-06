mod converter;
use clap::Parser;
use converter::*;
use regex::Regex;
use std::io::{stdin, stdout, Write};
/// Japanese Calender - Western Calender Translate Tool
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    // Japanese Era Output Format
    #[arg(short, long, default_value_t=EraFormat::Kanji)]
    format: EraFormat,
    // Umm... the... what say...
    input: Option<String>,
}
enum Command {
    Convert(String),
    ModeChange(EraFormat),
    Quit,
    None,
}
/**
 * stdin
*/
fn read(mode: EraFormat) -> String {
    let mut input = String::new();
    print!(":{}>", mode.to_string());
    stdout().flush().unwrap();
    stdin().read_line(&mut input).ok();
    input.trim().parse().ok().unwrap()
}
/**
 * 入力の解析
*/
fn parse_input(input: impl AsRef<str>) -> Command {
    let input = input.as_ref();
    match input {
        "h" => {
            help_display();
            Command::None
        }
        "q" => Command::Quit,
        "i" => Command::ModeChange(EraFormat::Initial),
        "j" => Command::ModeChange(EraFormat::Kanji),
        _ => Command::Convert(input.to_string()),
    }
}
/**
 * 開始時の表示
*/
fn start_display() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let s = format!("Welcome to WaTra! (ver. {VERSION})");
    println!("{}", s);
    help_display();
}
/**
 * ヘルプの表示
*/
fn help_display() {
    println!(
        r"  +-----+--------------+
  | CMD |  DESCRIPTION |
  +-----+--------------+
  |  h  | Show Help    |
  |  i  | Initial Mode |
  |  j  | Kanji Mode   |
  |  q  | Quit         |
  +-----+--------------+"
    );
}
fn translate_core(s: impl AsRef<str>, mode: EraFormat) -> Result<String, String> {
    let s = s.as_ref().trim();
    let first_char = s.as_bytes()[0];
    let reg_western_calender = Regex::new(r"(\d+)(.*)").unwrap();
    if (48..=57).contains(&first_char) {
        // 西暦を和暦へ変換
        if let Some(caps) = reg_western_calender.captures(s) {
            if caps[2].is_empty() || &caps[2] == "年" {
                if let Ok(y) = &caps[1].parse::<u32>() {
                    let res = western_to_japanese(*y);
                    match res {
                        Ok((name, year)) => Ok(cvt_era_string(name, year, mode)),
                        Err(e) => Err(e),
                    }
                } else {
                    Err(format!("{} can not convert year.", s))
                }
            } else {
                Err(format!("{} is invalid format.", s))
            }
        } else {
            Err(format!("{} is invalid format.", s))
        }
        // western_to_japanese(year)
    } else {
        let res = japanese_to_western(s);
        match res {
            Ok(r) => Ok(r.to_string()),
            Err(e) => Err(e),
        }
    }
}
fn main() {
    let args = Args::parse();
    let mut mode = args.format;
    if let Some(s) = args.input {
        match translate_core(s, mode) {
            Ok(s) => println!("{}", s),
            Err(e) => println!("{}", e),
        }
    } else {
        start_display();
        loop {
            match parse_input(read(mode)) {
                Command::Quit => {
                    println!("Leaving WaTra.");
                    return;
                }
                Command::ModeChange(m) => mode = m,
                Command::Convert(s) => match translate_core(s, mode) {
                    Ok(s) => println!("{}", s),
                    Err(e) => println!("{}", e),
                },
                Command::None => {}
            }
        }
    }
}
