use crate::symbol::Symbol;

pub type Number = i32;
pub type Label = usize;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
	PUSH(i32), DUP, SWAP, DROP,
	ADD, SUB, MUL, DIV, MOD,
	SHEAP, RHEAP,
	LABEL(Label), CALL(Label), JUMP(Label), EJUMP(Label), NJUMP(Label), RET, EXIT,
	PRINTC, PRINTN, READC, READN
}



impl Instruction {

	pub fn next<'a, I>(iter: &mut I) -> Option<Instruction>
	where I: Iterator<Item = Symbol> {
		let fst = iter.next();
		let sec = iter.next();
		match (fst, sec) {
			(Some(Symbol::S), Some(s)) => Instruction::next_stack(s, iter),
			(Some(Symbol::T), Some(Symbol::S)) => Instruction::next_arithmetic(iter),
			(Some(Symbol::T), Some(Symbol::T)) => Instruction::next_heap(iter),
			(Some(Symbol::L), Some(s)) => Instruction::next_flow(s, iter),
			(Some(Symbol::T), Some(Symbol::L)) => Instruction::next_io(iter),
			_ => None,
		}
	}


	fn next_stack<'a, I>(first: Symbol, iter: &mut I) -> Option<Instruction>
	where I: Iterator<Item = Symbol> {
		match first {
			Symbol::S => Some(Instruction::PUSH(Instruction::next_param::<Number>(iter))),
			Symbol::L => match iter.next() {
				Some(Symbol::S) => Some(Instruction::DUP),
				Some(Symbol::T) => Some(Instruction::SWAP),
				Some(Symbol::L) => Some(Instruction::DROP),
				None => None,
			},
			_	=> None
		}
	}
	
	fn next_arithmetic<'a, I>(iter: &mut I) -> Option<Instruction>
	where I: Iterator<Item = Symbol> {
		match (iter.next(), iter.next()) {
			(Some(Symbol::S), Some(Symbol::S)) => Some(Instruction::ADD),
			(Some(Symbol::S), Some(Symbol::T)) => Some(Instruction::SUB),
			(Some(Symbol::S), Some(Symbol::L)) => Some(Instruction::MUL),
			(Some(Symbol::T), Some(Symbol::S)) => Some(Instruction::DIV),
			(Some(Symbol::T), Some(Symbol::T)) => Some(Instruction::MOD),
			_ => None
		}
	}

	fn next_heap<'a, I>(iter: &mut I) -> Option<Instruction>
	where I: Iterator<Item = Symbol> {
		match iter.next() {
			Some(Symbol::S) => Some(Instruction::SHEAP),
			Some(Symbol::T) => Some(Instruction::RHEAP),
			_ => None,
		}
	}

	fn next_flow<'a, I>(first: Symbol, iter: &mut I) -> Option<Instruction>
	where I: Iterator<Item = Symbol> {
		match (first, iter.next()) {
			(Symbol::S, Some(Symbol::S)) => Some(Instruction::LABEL(Instruction::next_param::<Label>(iter))),
			(Symbol::S, Some(Symbol::T)) => Some(Instruction::CALL (Instruction::next_param::<Label>(iter))),
			(Symbol::S, Some(Symbol::L)) => Some(Instruction::JUMP (Instruction::next_param::<Label>(iter))),
			(Symbol::T, Some(Symbol::S)) => Some(Instruction::EJUMP(Instruction::next_param::<Label>(iter))),
			(Symbol::T, Some(Symbol::T)) => Some(Instruction::NJUMP(Instruction::next_param::<Label>(iter))),
			(Symbol::T, Some(Symbol::L)) => Some(Instruction::RET),
			(Symbol::L, Some(Symbol::L)) => Some(Instruction::EXIT),
			_ => None
		}
	}


	fn next_param<'a, T: num::PrimInt>(iter: &mut dyn Iterator<Item = Symbol>) -> T {
		let mut l : T = T::zero();
		let mut osymbol = iter.next();
		while let Some(sym) = osymbol {
			match sym {
				Symbol::S => { l = l.shl(1);},
				Symbol::T => { l = l.shl(1) + T::one();},
				Symbol::L => { break; }
			}
			osymbol = iter.next();
		}
		l
	}

	fn next_io<'a, I>(iter: &mut I) -> Option<Instruction>
	where I: Iterator<Item = Symbol> {
		match (iter.next(), iter.next()) {
			(Some(Symbol::S), Some(Symbol::S)) => Some(Instruction::PRINTC),
			(Some(Symbol::S), Some(Symbol::T)) => Some(Instruction::PRINTN),
			(Some(Symbol::T), Some(Symbol::S)) => Some(Instruction::READC),
			(Some(Symbol::T), Some(Symbol::T)) => Some(Instruction::READN),
			_ => None
		}

	}
}


mod tests {

	#[test]
	fn stack() {
		use crate::{Symbol, Instruction};

		let v = vec![Symbol::S, Symbol::S, Symbol::S];
		assert_eq!(Instruction::next(&mut v.iter().cloned()), Some(Instruction::PUSH(0)));
		
		let v = vec![Symbol::S, Symbol::L, Symbol::S];
		assert_eq!(Instruction::next(&mut v.iter().cloned()), Some(Instruction::DUP));
		
		let v = vec![Symbol::S, Symbol::L, Symbol::T];
		assert_eq!(Instruction::next(&mut v.iter().cloned()), Some(Instruction::SWAP));
		
		let v = vec![Symbol::S, Symbol::L, Symbol::L];
		assert_eq!(Instruction::next(&mut v.iter().cloned()), Some(Instruction::DROP));
	}


	#[test]
	fn aritmethic() {
		use crate::{Symbol, Instruction};

		let v = vec![Symbol::T, Symbol::S, Symbol::S, Symbol::S];
		assert_eq!(Instruction::next(&mut v.iter().cloned()), Some(Instruction::ADD));
		
		let v = vec![Symbol::T, Symbol::S, Symbol::S, Symbol::T];
		assert_eq!(Instruction::next(&mut v.iter().cloned()), Some(Instruction::SUB));
		
		let v = vec![Symbol::T, Symbol::S, Symbol::S, Symbol::L];
		assert_eq!(Instruction::next(&mut v.iter().cloned()), Some(Instruction::MUL));
		
		let v = vec![Symbol::T, Symbol::S, Symbol::T, Symbol::S];
		assert_eq!(Instruction::next(&mut v.iter().cloned()), Some(Instruction::DIV));
		
		let v = vec![Symbol::T, Symbol::S, Symbol::T, Symbol::T];
		assert_eq!(Instruction::next(&mut v.iter().cloned()), Some(Instruction::MOD));		
	}

	#[test]
	fn heap() {
		use crate::{Symbol, Instruction};
		
		let v = vec![Symbol::T, Symbol::T, Symbol::S];
		assert_eq!(Instruction::next(&mut v.iter().cloned()), Some(Instruction::SHEAP));
		
		let v = vec![Symbol::T, Symbol::T, Symbol::T];
		assert_eq!(Instruction::next(&mut v.iter().cloned()), Some(Instruction::RHEAP));
	}

	#[test]
	fn flow() {
		use crate::{Symbol, Instruction};

		let v = vec![Symbol::L, Symbol::S, Symbol::S, Symbol::S];
		assert_eq!(Instruction::next(&mut v.iter().cloned()), Some(Instruction::LABEL(0)));
		
		let v = vec![Symbol::L, Symbol::S, Symbol::T, Symbol::S];
		assert_eq!(Instruction::next(&mut v.iter().cloned()), Some(Instruction::CALL(0)));
		
		let v = vec![Symbol::L, Symbol::S, Symbol::L, Symbol::S];
		assert_eq!(Instruction::next(&mut v.iter().cloned()), Some(Instruction::JUMP(0)));
		
		let v = vec![Symbol::L, Symbol::T, Symbol::S, Symbol::S];
		assert_eq!(Instruction::next(&mut v.iter().cloned()), Some(Instruction::EJUMP(0)));
		
		let v = vec![Symbol::L, Symbol::T, Symbol::T, Symbol::S];
		assert_eq!(Instruction::next(&mut v.iter().cloned()), Some(Instruction::NJUMP(0)));
		
		let v = vec![Symbol::L, Symbol::T, Symbol::L];
		assert_eq!(Instruction::next(&mut v.iter().cloned()), Some(Instruction::RET));

		let v = vec![Symbol::L, Symbol::L, Symbol::L];
		assert_eq!(Instruction::next(&mut v.iter().cloned()), Some(Instruction::EXIT));
	}

	#[test]
	fn io() {
		use crate::{Symbol, Instruction};
		
		let v = vec![Symbol::T, Symbol::L, Symbol::S, Symbol::S];
		assert_eq!(Instruction::next(&mut v.iter().cloned()), Some(Instruction::PRINTC));
		
		let v = vec![Symbol::T, Symbol::L, Symbol::S, Symbol::T];
		assert_eq!(Instruction::next(&mut v.iter().cloned()), Some(Instruction::PRINTN));
		
		let v = vec![Symbol::T, Symbol::L, Symbol::T, Symbol::S];
		assert_eq!(Instruction::next(&mut v.iter().cloned()), Some(Instruction::READC));
		
		let v = vec![Symbol::T, Symbol::L, Symbol::T, Symbol::T];
		assert_eq!(Instruction::next(&mut v.iter().cloned()), Some(Instruction::READN));
	}

	#[test]
	fn misc() {
		use crate::{Symbol, Instruction};
		
		let v = vec![Symbol::T, Symbol::L, Symbol::L];
		assert_eq!(Instruction::next(&mut v.iter().cloned()), None);
		
		let v = vec![Symbol::T, Symbol::L, Symbol::T, Symbol::L];
		assert_eq!(Instruction::next(&mut v.iter().cloned()), None);

		let v = vec![Symbol::T, Symbol::L, Symbol::L];
		assert_eq!(Instruction::next(&mut v.iter().cloned()), None);

		let v = vec![];
		assert_eq!(Instruction::next(&mut v.iter().cloned()), None);
	}

}