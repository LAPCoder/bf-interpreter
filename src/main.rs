mod interpreter;

use clap::Parser;
use interpreter::{str_to_symbol, tunnels, execution};

#[derive(Parser, Debug)]
#[command(name = "Brainfuck Interpreter")]
#[command(about = "A simple Brainfuck interpreter", long_about = None)]
struct Args {
	/// Enable verbose output
	#[arg(short, long)]
	verbose: bool,

	/// Input file containing the Brainfuck program (use stdin if not provided)
	#[arg(short, long)]
	file: Option<String>,

	/// Custom EOF symbol for stdin input (default: EOF)
	#[arg(short, long, default_value = "EOF")]
	eof_symbol: String,
}

fn main() -> Result<(),Box<dyn std::error::Error>>
{
	let args = Args::parse();

	if args.verbose {
		println!("Brainfuck interpreter");
	}

	let mut input: String = " ".to_string();
	let mut input_str = " ";
	let mut instructions = Vec::new();

	// Read from file or stdin
	if let Some(file_path) = &args.file {
		// Read from file
		let file_content = std::fs::read_to_string(file_path)?;
		for line in file_content.lines() {
			if let Some(v) = str_to_symbol(line) {
				instructions.extend(v);
			}
		}
	} else {
		// Read from stdin
		let stdin = std::io::stdin();
		while input_str != args.eof_symbol {
			input.clear();
			stdin.read_line(&mut input)?;
			input_str = input.trim();
			if args.verbose {
				println!("Got '{}'", input_str);
			}
			if let Some(v) = str_to_symbol(input_str) {
				instructions.extend(v);
			}
		}
	}

	if args.verbose {
		println!("Program done, executing...");
	}

	execution(
		&instructions,
		&tunnels(&instructions).ok_or("Mismatch between number of [ and ]")?,
		args.verbose)?;

	Ok(())
}
