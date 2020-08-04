#![warn(missing_docs)]
#![crate_name = "rustws"]

//! 
//! This crate can be used to easily compile and interpret WhiteSpace programs
//! 


mod wsiter;
mod symbol;
mod instruction;
mod error;

/// The errors that can be thrown during runtime
pub use error::Error;

#[allow(unused_imports)]
use symbol::Symbol;
use instruction::{Instruction, Label};
use wsiter::WsIter;
use std::collections::HashMap;


struct Program {
	instructions: Vec<Instruction>,
	labels: std::collections::HashMap<Label, Label>
}

fn compile(code: String, print_instructions: bool) -> Program {
	let mut iter = WsIter::new(code.chars());
	let mut instructions = Vec::<Instruction>::new();
	let mut labels = std::collections::HashMap::new();
	let mut position : Label = 0;
	while let Some(instr) = Instruction::next(&mut iter) {
		if print_instructions { println!("{:?}", instr); }
		match instr {
			Instruction::LABEL(l) => {labels.insert(l, position);},
			i => {
				instructions.push(i); 
				position += 1;
			}
		}
	}

	Program { instructions, labels }
}

/// Current state of the program if no error is thrown
pub enum Status {
	/// No action is needed from outside, next instruction can be called
	Step, 
	/// Print the string 
	Print(String), 
	/// Convert integer to a character and print it
	PrintChar(i32), 
	/// Read an integer from i/o
	ReadInt, 
	/// Read a character fom i/o
	ReadChar, 
	/// The program should quit without any errors. Don't call the step function anymore
	Exit
}
/// The interpreter used for running whitespace
/// 
pub struct Interpreter {
	program: Program,
	pc: Label,
	stack: Vec<i32>,
	flowstack: Vec<Label>,
	heap: HashMap<instruction::Number, instruction::Number>,
}

type RuntimeStatus = Result<Status, Error>;

impl Interpreter {
	/// Create an Interpreter from a source code passed as the argument
	/// 
	/// The print_instructions parameter prints the instructions. This should be set to True when debugging the code
	/// 
	/// # Example
	/// ```
	/// # use rustws::Interpreter;
	/// let source = String::from("   \t\t\n");
	/// let interpreter = Interpreter::new(source, false);
	/// ```
	pub fn new(code: String, print_instructions: bool) -> Interpreter {
		Interpreter {
			program: compile(code, print_instructions),
			pc: 0,
			stack: Vec::new(),
			flowstack: Vec::new(),
			heap: HashMap::new()
		}
	}

	/// Initialize an interpreter from a file
	/// 
	/// The print_instructions parameter prints the instructions. This should be set to True when debugging the code
	/// 
	/// # Example
	/// ```
	/// # use rustws::Interpreter;
	/// let interpreter = Interpreter::from_file("main.ws", false);
	/// ```
	pub fn from_file(file: &str, print_instructions: bool) -> Option<Interpreter> {
		use std::fs;

		if let Ok(code) = fs::read_to_string(file) {
			Some(Interpreter::new(code, print_instructions))
		} else {
			None
		}
	}

	/// Write a value on top of the stack
	/// Should be used when the Status is set to ReadChar/ReadInt
	pub fn write(&mut self, num: i32) { 
		self.stack.push(num) 
	}

	/// Step to the next instructions
	/// 
	/// See Status and Error for the return value
	pub fn step(&mut self) -> RuntimeStatus {
		if let Some(&instr) = self.program.instructions.get(self.pc) {
			self.step_instr(instr)
		} else {
			Err(Error::PC_OUT_OF_BOUNDS)
		}
	}

	/// Current Program Counter
	pub fn pc(&self) -> usize {
		self.pc
	}
}

impl Interpreter {
	fn step_instr(&mut self, instr: Instruction) -> RuntimeStatus {
		match instr {
			Instruction::PUSH(num) => self.instr_push(num),
			Instruction::DUP       => self.instr_dup(),
			Instruction::SWAP      => self.instr_swap(),
			Instruction::DROP      => self.instr_drop(),

			Instruction::ADD       => self.instr_arithmetic(|l, r| Ok(l + r)),
			Instruction::SUB       => self.instr_arithmetic(|l, r| Ok(l - r)),
			Instruction::MUL       => self.instr_arithmetic(|l, r| Ok(l % r)),
			Instruction::DIV       => self.instr_arithmetic(|l, r| if r != 0 {Ok(l / r)} else {Err(Error::DIVISION_BY_ZERO)}),
			Instruction::MOD       => self.instr_arithmetic(|l, r| Ok(l % r)),

			Instruction::SHEAP     => self.instr_sheap(),
			Instruction::RHEAP     => self.instr_rheap(),

			Instruction::CALL(l)   => self.instr_call(l),
			Instruction::JUMP(l)   => self.instr_jump(l),
			Instruction::EJUMP(l)  => self.instr_ifjump(l, |v| v == 0),
			Instruction::NJUMP(l)  => self.instr_ifjump(l, |v| v < 0),
			Instruction::RET       => self.instr_ret(),
			Instruction::EXIT      => Ok(Status::Exit),

			Instruction::PRINTC    => self.instr_printc(),
			Instruction::PRINTN    => self.instr_print(),

			Instruction::READC     => {self.pc += 1; Ok(Status::ReadChar)},
			Instruction::READN     => {self.pc += 1; Ok(Status::ReadInt)},
			_ => Err(Error::UNIMPLEMENTED_INSTRUCTION)
		}
	}


	fn instr_push(&mut self, num: instruction::Number) -> RuntimeStatus {
		self.stack.push(num);
		self.pc += 1;
		Ok(Status::Step)
	}

	fn instr_dup(&mut self) -> RuntimeStatus {
		if let Some(&last) = self.stack.last() {
			self.stack.push(last);
			self.pc += 1;
			Ok(Status::Step)
		} else {
			Err(Error::EMPTY_STACK)
		}
	}

	fn instr_swap(&mut self) -> RuntimeStatus {
		let f = self.stack.pop();
		let s = self.stack.pop();

		match (f, s) {
			(Some(first), Some(second)) => {
				self.stack.push(first); self.stack.push(second);
				self.pc += 1;
				Ok(Status::Step)
			}
			_ => Err(Error::EMPTY_STACK)
		}
	}

	fn instr_drop(&mut self) -> RuntimeStatus {
		if let Some(_) = self.stack.pop() {
			self.pc += 1;
			Ok(Status::Step)
		} else {
			Err(Error::EMPTY_STACK)
		}
	}

	fn instr_arithmetic(&mut self, fun: fn(i32, i32) -> Result<i32, Error>) -> RuntimeStatus {
		let right = self.stack.pop();
		let left = self.stack.pop();
		match (left, right) {
			(Some(l), Some(r)) => {
				match fun(l, r) {
					Ok(res) => {
						self.stack.push(res); 
						self.pc += 1;
						Ok(Status::Step)},
					Err(err) => Err(err),
				}
			},
			_ => Err(Error::EMPTY_STACK)
		}
	}


	fn instr_sheap(&mut self) -> RuntimeStatus {
		let value = self.stack.pop();
		let addr = self.stack.pop();
		match (addr, value) {
			(Some(a), Some(v)) => {
				self.heap.insert(a, v);
				self.pc += 1;
				Ok(Status::Step)
			},
			_ => Err(Error::EMPTY_STACK)
		}
	}

	fn instr_rheap(&mut self) -> RuntimeStatus {
		if let Some(addr) = self.stack.pop() {
			if let Some(&val) = self.heap.get(&addr) {
				self.stack.push(val);
				self.pc += 1;
				Ok(Status::Step)
			} else {
				Err(Error::UNDEFINED_HEAP_ADDRESS)
			}
		} else {
			Err(Error::EMPTY_STACK)
		}
	}

	fn to_label(&mut self, label: Label) -> RuntimeStatus {
		if let Some(&l) = self.program.labels.get(&label) {
			self.pc = l;			
			Ok(Status::Step)
		} else {
			Err(Error::UNDEFINED_LABEL)
		}
	}

	fn instr_call(&mut self, label: Label) -> RuntimeStatus {
		self.flowstack.push(self.pc);
		self.to_label(label)
	}

	fn instr_jump(&mut self, label: Label) -> RuntimeStatus {
		self.to_label(label)
	}
	
	fn instr_ifjump(&mut self, label: Label, fun: fn(i32) -> bool) -> RuntimeStatus {
		if let Some(&last) = self.stack.last() {
			if let Some(&l) = self.program.labels.get(&label) {
				if fun(last) {
					self.pc = l;
				} else {
					self.pc += 1;
				}
				Ok(Status::Step)
			} else {
				Err(Error::UNDEFINED_LABEL)
			}
		} else {
			Err(Error::EMPTY_STACK)
		}
	}

	fn instr_ret(&mut self) -> RuntimeStatus {
		if let Some(&last) = self.flowstack.last() {
			self.pc = last;
			Ok(Status::Step)
		} else {
			Err(Error::EMPTY_FLOW_STACK)
		}
	}

	fn instr_printc(&mut self) -> RuntimeStatus {
		if let Some(last) = self.stack.pop() {
			self.pc += 1;
			Ok(Status::PrintChar(last))
		} else {
			Err(Error::EMPTY_STACK)
		}

	}

	fn instr_print(&mut self) -> RuntimeStatus {
		if let Some(last) = self.stack.pop() {
			self.pc += 1;
			Ok(Status::Print(last.to_string()))
		} else {
			Err(Error::EMPTY_STACK)
		}
	}

}
