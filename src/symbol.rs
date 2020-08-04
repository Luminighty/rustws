#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Symbol {
	S, T, L
}

impl Symbol {
	pub fn from_char(c: char) -> Option<Symbol> {
		match c {
			' '  => Some(Symbol::S),
			'\t' => Some(Symbol::T),
			'\n' => Some(Symbol::L),
			_ => None,
		}
	}
}


mod tests {
	
	#[test]
	fn space() {
		use crate::Symbol;
		assert_eq!(Symbol::from_char(' '), Some(Symbol::S));
	}

	#[test]
	fn tab() {
		use crate::Symbol;
		assert_eq!(Symbol::from_char('\t'), Some(Symbol::T));
	}

	#[test]
	fn ln() {
		use crate::Symbol;
		assert_eq!(Symbol::from_char('\n'), Some(Symbol::L));
	}

	
	#[test]
	fn misc() {
		use crate::Symbol;
		assert_eq!(Symbol::from_char('a'), None);
		assert_eq!(Symbol::from_char('b'), None);
		assert_eq!(Symbol::from_char('d'), None);
		assert_eq!(Symbol::from_char('t'), None);
		assert_eq!(Symbol::from_char('j'), None);
		assert_eq!(Symbol::from_char(','), None);
		assert_eq!(Symbol::from_char('\r'), None);
	}
}