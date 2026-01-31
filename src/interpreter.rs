use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum SYMBOLS { // All the symbols used by the BF
	Add(isize), // + and -
	Move(isize), // < and >
	Output,  // .
	Input, // ,
	LoopB,// [
	LoopE // ]
}

impl SYMBOLS {
    pub fn combine(self, other: SYMBOLS) -> Option<SYMBOLS> {
        match (self, other) {
            (SYMBOLS::Add(a),  SYMBOLS::Add(b))  => Some(SYMBOLS::Add(a + b)),
            (SYMBOLS::Move(a), SYMBOLS::Move(b)) => Some(SYMBOLS::Move(a + b)),
            _ => None,
        }
    }
}

const BAD_TUNNEL: &str =
	"Runtime error: start/end of loop incorrectly linked to the other end.";

// Step 1: convert text to instructions
pub fn str_to_symbol(string: &str) -> Option<Vec<SYMBOLS>> {
	let mut instructions_ret: Vec<SYMBOLS> = Vec::new();

	instructions_ret.reserve(string.len());

	for c in string.bytes() {
		if let Some(instr) = match c {
			b'+' => Some(SYMBOLS::Add(1)),
			b'-' => Some(SYMBOLS::Add(-1)),
			b'>' => Some(SYMBOLS::Move(1)),
			b'<' => Some(SYMBOLS::Move(-1)),
			b'.' => Some(SYMBOLS::Output),
			b',' => Some(SYMBOLS::Input),
			b'[' => Some(SYMBOLS::LoopB),
			b']' => Some(SYMBOLS::LoopE),
			 _  => None,
		} {
			if !instructions_ret.is_empty() {
				if (matches!(instructions_ret.last()?, SYMBOLS::Move(_))
				 && matches!(instr,                    SYMBOLS::Move(_)))
				|| (matches!(instructions_ret.last()?, SYMBOLS::Add(_))
				 && matches!(instr,                    SYMBOLS::Add(_))) {
					let t = instructions_ret.pop()?;
					instructions_ret.push(t.combine(instr)?);
				}
				else {
					instructions_ret.push(instr);
				}
			}
			else {
				instructions_ret.push(instr);
			}
		}
	}

	Some(instructions_ret)
}

// Step 2: make tunnels (links between the correspondings [ and ]
// so it can jump from one th the other)
// Returns a list of arrays: [the start of a loop, it's end]
pub fn tunnels(instruction: &Vec<SYMBOLS>) -> Option<HashMap<usize, usize>>
{
	let mut loop_stack: Vec<usize> = Vec::new();
	let mut return_list: HashMap<usize, usize> = HashMap::new();

	for (i, instr) in instruction.into_iter().enumerate() {
		match instr {
			SYMBOLS::LoopB => { loop_stack.push(i); },
			SYMBOLS::LoopE => {
				return_list.insert(i, *loop_stack.last()?); // The 2 ways so the
				return_list.insert(loop_stack.pop()?, i);  // reading is in O(1)
			},
			_ => { },
		}
	}
	
	if loop_stack.is_empty() { Some(return_list) } else { None }
}

// Step 3: execute the instructions
pub fn execution(
	instructions: &Vec<SYMBOLS>,
	tunnels_list: &HashMap<usize, usize>,
	verbose: bool)
 -> Result<(),Box<dyn std::error::Error>>
{
	let mut ptr: usize = 0;
	let mut tape: Vec<u8> = vec![0u8;std::cmp::min(1_000_000_000,usize::MAX-1)];

	let mut i = 0;

	while i < instructions.len() {
		let instr: &SYMBOLS = &instructions[i];
		match *instr {
			SYMBOLS::Add(n) => { tape[ptr] = tape[ptr].wrapping_add(n as u8); },
			SYMBOLS::Move(n) => { ptr = ptr.wrapping_add(n as usize); },
			SYMBOLS::Input => { 
				use std::io::{self, Read, Write};
				io::stdout().flush().unwrap();
				let mut buffer = [0u8; 1];
				let read_bytes = io::stdin().read(&mut buffer)?;
				tape[ptr] = if read_bytes == 0 { 0 } else { buffer[0] };
			},
			SYMBOLS::Output=> { print!("{}", tape[ptr] as char); },
			// No need for flush: it does it when there is a \new line, 
			// when there is an input (see above) of when the stdout internal
			// buffer is full (a few kBytes, so a few thousand of chars)

			SYMBOLS::LoopB => 
				// Logic: save current loop cntr and jump to it if tape==0 and
				// make a link <=> so it can jump in both directions
				{ if tape[ptr] == 0 {
				                i = *tunnels_list.get(&i).ok_or(BAD_TUNNEL)?; }},
			SYMBOLS::LoopE => { i = *tunnels_list.get(&i).ok_or(BAD_TUNNEL)?-1;},
		}
		i += 1;
	}
	
	use std::io::{self, Write};

	io::stdout().flush().unwrap();
	if verbose {
		println!("Done");
	}

	Ok(())
}
