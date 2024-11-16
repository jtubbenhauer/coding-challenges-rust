use clap::Parser;
use clio::Input;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'c', long)]
    bytes: bool,

    #[arg(short = 'l', long)]
    lines: bool,

    #[arg(required = true)]
    filename: String,
}

fn main() -> clio::Result<()> {
    let args = Args::parse();
    let input = clio::Input::new(&args.filename)?;
    let mut output = String::from("");

    if args.bytes {
        handle_bytes(&input, &mut output)
    }

    if args.lines {
        handle_lines(&input, &mut output)
    }

    println!("{output} {}", input.path());
    Ok(())
}

fn handle_bytes(input: &Input, output: &mut String) {
    if let Some(size) = input.len() {
        if !output.is_empty() {
            output.push(' ');
        }
        output.push_str(&size.to_string());
    }
}

fn handle_lines(input: &Input, output: &mut String) {
    if let None = input.len() {
        return;
    }

    let lines = input.to_string().lines().count().to_string();
    if lines.is_empty() {
        return;
    }

    if !output.is_empty() {
        output.push(' ');
    }
    output.push_str(&lines);
}
