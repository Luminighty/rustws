use rustws::{Interpreter, Status};

fn read_num() -> i32 {
	let mut buffer = String::new();
	std::io::stdin().read_line(&mut buffer).expect("Failed");
	buffer.trim().parse().unwrap()
}

fn read_char() -> i32 {
	use std::io::Read;
	let mut buffer = [0];
	std::io::stdin().read(&mut buffer).expect("Failed");
	buffer[0] as i32
}

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let length = args.len();
	if length < 2 {
		println!("No input file given.");
		println!("Usage: wscomp {{.ws file}}");
		return;
	}


	let mut inter = Interpreter::from_file(&args[1], args.len() > 2 && args[2] == String::from("debug")).unwrap();
	let mut exit = false;

	while !exit {
		match inter.step() {
			Ok(Status::Step) => {},
			Ok(Status::Print(n)) => print!("{}", n),
			Ok(Status::PrintChar(n)) => print!("{}", n),
			Ok(Status::ReadChar) => inter.write(read_char()),
			Ok(Status::ReadInt) => inter.write(read_num()),
			Ok(Status::Exit) => exit = true,
			Err(x) => {
				println!("Program threw an Exception {:?} at line {:}", x, inter.pc());
				break;
			},
		}
	}

}