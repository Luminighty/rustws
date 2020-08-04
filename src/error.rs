#[allow(non_camel_case_types)]
#[derive(Debug)]
/// The kind of exceptions the program can throw
pub enum Error {
	/// Error thrown when current Program Conter reached a position outside the program
	/// This can also be thrown if the did not receive an Exit command before the last statement
	PC_OUT_OF_BOUNDS,
	/// Error thrown when an undefined statement was read
	/// Syntax Error for short
	UNIMPLEMENTED_INSTRUCTION, 
	/// Error thrown when trying to access data from an empty stack
	EMPTY_STACK,
	/// Error thrown when return called without calling a function beforehand
	EMPTY_FLOW_STACK,
	/// Error thrown when trying to divide by zero
	DIVISION_BY_ZERO,
	/// Error thrown when trying to access an undefined address from the heap
	UNDEFINED_HEAP_ADDRESS,
	/// Error thrown when trying to jump/call a label that was not defined
	UNDEFINED_LABEL,
}