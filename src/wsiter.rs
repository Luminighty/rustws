use crate::symbol::Symbol;

pub struct WsIter<'a> {
	iter: std::str::Chars<'a>
}

impl WsIter<'_> {
	pub fn new(iter: std::str::Chars) -> WsIter {
		WsIter {iter}
	}
}


impl Iterator for WsIter<'_> {
	type Item = Symbol;

	fn next(&mut self) -> Option<Symbol> {
		while let Some(c) = self.iter.next() {
			let symbol = Symbol::from_char(c);
			if symbol.is_some() {
				return symbol;
			}
		}
		None
	}

}


mod tests {

	#[test]
	fn normal() {
		use crate::{WsIter, Symbol};

		let mut iter = WsIter::new("  \t \n\n \t".chars());
		assert_eq!(iter.next(), Some(Symbol::S));
		assert_eq!(iter.next(), Some(Symbol::S));
		assert_eq!(iter.next(), Some(Symbol::T));
		assert_eq!(iter.next(), Some(Symbol::S));
		assert_eq!(iter.next(), Some(Symbol::L));
		assert_eq!(iter.next(), Some(Symbol::L));
		assert_eq!(iter.next(), Some(Symbol::S));
		assert_eq!(iter.next(), Some(Symbol::T));
		assert_eq!(iter.next(), None);
	}

	#[test]
	fn comments() {
		use crate::{WsIter, Symbol};

		let mut iter = WsIter::new("Hello World \t".chars());
		assert_eq!(iter.next(), Some(Symbol::S));
		assert_eq!(iter.next(), Some(Symbol::S));
		assert_eq!(iter.next(), Some(Symbol::T));
		assert_eq!(iter.next(), None);
	}

	#[test]
	fn symbol_comments() {
		use crate::{WsIter, Symbol};

		let mut iter = WsIter::new("\t \n\r\r\n".chars());
		assert_eq!(iter.next(), Some(Symbol::T));
		assert_eq!(iter.next(), Some(Symbol::S));
		assert_eq!(iter.next(), Some(Symbol::L));
		assert_eq!(iter.next(), Some(Symbol::L));
		assert_eq!(iter.next(), None);
	}

}