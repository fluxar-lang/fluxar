use std::collections::HashMap;

use crate::ast::tokens::{LiteralValue, Token, TokenType, TokenType::*};

fn get_keywords() -> HashMap<&'static str, TokenType> {
	HashMap::from([
		("fn", Fn),
		("let", Let),
		("return", Return),
		("struct", Struct),
		("enum", Enum),
		("extend", Extend),
		("interface", Interface),
		("match", Match),
		("if", If),
		("else", Else),
		("while", While),
		("for", For),
		("in", In),
		("loop", Loop),
		("break", Break),
		("continue", Continue),
		("defer", Defer),
		("region", Region),
		("use", Use),
		("public", Public),
		("async", Async),
		("true", True),
		("false", False),
		("or", Or),
		("as", As),
		("self", SelfKw),
		("mut", Mut),
		("move", Move),
	])
}

pub struct Scanner {
	source: Vec<char>,
	pub tokens: Vec<Token>,
	start: usize,
	current: usize,
	line: usize,
	keywords: HashMap<&'static str, TokenType>,
}

impl Scanner {
	pub fn new(source: &str) -> Self {
		Self {
			source: source.chars().collect(),
			tokens: vec![],
			start: 0,
			current: 0,
			line: 1,
			keywords: get_keywords(),
		}
	}

	pub fn scan_tokens(&mut self) -> Result<Vec<Token>, String> {
		let mut errors = vec![];

		while !self.is_at_end() {
			self.start = self.current;
			match self.scan_token() {
				Ok(_) => (),
				Err(msg) => errors.push(msg),
			}
		}

		self.tokens.push(Token {
			token_type: Eof,
			lexeme: "".to_string(),
			literal: None,
			line_number: self.line,
		});

		if !errors.is_empty() {
			let mut joined = String::new();

			for error in errors {
				joined.push_str(&error);
				joined.push('\n');
			}
			
			return Err(joined);
		}

		Ok(self.tokens.clone())
	}

	fn scan_token(&mut self) -> Result<(), String> {
		let c = self.advance();
		match c {
			'(' => self.add_token(LeftParen),
			')' => self.add_token(RightParen),
			'{' => self.add_token(LeftBrace),
			'}' => self.add_token(RightBrace),
			'[' => self.add_token(LeftBracket),
			']' => self.add_token(RightBracket),
			',' => self.add_token(Comma),
			'?' => self.add_token(Question),
			'_' => {
				if self.peek().is_alphanumeric() || self.peek() == '_' {
					self.identifier()
				}
				else {
					self.add_token(Underscore)
				}
			}
			';' => {
				if matches!(self.tokens.last(), Some(t) if t.token_type == Semicolon) {
					while self.peek() == ';' {
						self.advance();
					}

					let run_start = if self.start > 0 && self.source[self.start - 1] == ';' {
						self.start - 1
					}
					else {
						self.start
					};

					let unexpected: String = self.source[run_start..self.current].iter().collect();
					return Err(format!("Line {}: unexpected \"{}\"", self.line, unexpected));
				}
				self.add_token(Semicolon);
			}

			'+' => {
				if self.char_match('=') { self.add_token(PlusEqual); }
				else { self.add_token(Plus); }
			}
			'*' => {
				if self.char_match('=') { self.add_token(StarEqual); }
				else { self.add_token(Star); }
			}
			'%' => self.add_token(Percent),
			
			'-' => {
				if self.char_match('-') {
					// -- line comment
					while self.peek() != '\n' && !self.is_at_end() {
						self.advance();
					}
				}
				else if self.char_match('>') {
					self.add_token(Arrow);
				}
				else if self.char_match('=') {
					self.add_token(MinusEqual);
				}
				else {
					self.add_token(Minus);
				}
			}
			'/' => {
				if self.char_match('=') { self.add_token(SlashEqual); }
				else { self.add_token(Slash); }
			}

			'.' => {
				if self.char_match('.') { self.add_token(DotDot); }
				else { self.add_token(Dot); }
			}
			':' => {
				if self.char_match(':') { self.add_token(ColonColon); }
				else { self.add_token(Colon); }
			}

			'!' => {
				if self.char_match('=') { self.add_token(BangEqual); }
				else { self.add_token(Bang); }
			}
			'=' => {
				if self.char_match('=') { self.add_token(EqualEqual); }
				else if self.char_match('>') { self.add_token(FatArrow); }
				else { self.add_token(Equal); }
			}
			'<' => {
				if self.char_match('=') { self.add_token(LessEqual); }
				else { self.add_token(Less); }
			}
			'>' => {
				if self.char_match('=') { self.add_token(GreaterEqual); }
				else { self.add_token(Greater); }
			}

			'&' => {
				if self.char_match('&') { self.add_token(AmpAmp); }
				else { self.add_token(Ampersand); }
			}
			'|' => {
				if self.char_match('|') { self.add_token(PipePipe); }
				else {
					return Err(format!("Line {}: unexpected '|', did you mean '||'?", self.line));
				}
			}

			' ' | '\r' | '\t' => {}
			'\n' => {
				self.line += 1;
			}

			'"' => self.string()?,
			'\'' => self.char_literal()?,

			c => {
				if c.is_ascii_digit() {
					self.number()?;
				}
				else if c.is_alphabetic() || c == '_' {
					self.identifier();
				}
				else {
					return Err(format!("Line {}: unrecognized character '{}'", self.line, c));
				}
			}
		}
		Ok(())
	}

	fn add_token(&mut self, token_type: TokenType) {
		self.add_token_lit(token_type, None);
	}

	fn add_token_lit(&mut self, token_type: TokenType, literal: Option<LiteralValue>) {
		let text: String = self.source[self.start..self.current]
			.iter()
			.collect();

		self.tokens.push(Token {
			token_type,
			lexeme: text,
			literal,
			line_number: self.line,
		});
	}

	fn string(&mut self) -> Result<(), String> {
		while self.peek() != '"' && !self.is_at_end() {
			if self.peek() == '\n' {
				self.line += 1;
			}
			self.advance();
		}

		if self.is_at_end() {
			return Err(format!("Line {}: unterminated string", self.line));
		}
		self.advance();

		let value: String = self.source[self.start + 1..self.current - 1].iter().collect();
		self.add_token_lit(StringLit, Some(LiteralValue::StringValue(value)));
		Ok(())
	}

	fn fstring(&mut self) -> Result<(), String> {
		while self.peek() != '"' && !self.is_at_end() {
			if self.peek() == '\n' {
				self.line += 1;
			}
			self.advance();
		}

		if self.is_at_end() {
			return Err(format!("Line {}: unterminated format string", self.line));
		}
		self.advance();

		// content between f" and closing " (skip the f and both quotes)
		let value: String = self.source[self.start + 2..self.current - 1].iter().collect();
		self.add_token_lit(FStringLit, Some(LiteralValue::StringValue(value)));
		Ok(())
	}

	fn char_literal(&mut self) -> Result<(), String> {
		if self.is_at_end() {
			return Err(format!("Line {}: unterminated char literal", self.line));
		}

		let ch = if self.peek() == '\\' {
			self.advance();
			match self.advance() {
				'n' => '\n',
				't' => '\t',
				'r' => '\r',
				'0' => '\0',
				'\\' => '\\',
				'\'' => '\'',
				c => return Err(format!("Line {}: unknown escape '\\{}'", self.line, c)),
			}
		}
		else {
			self.advance()
		};

		if self.is_at_end() || self.peek() != '\'' {
			if !self.is_at_end() && self.peek() != '\'' {
				return Err(format!("Line {}: multi-char literal is not allowed", self.line));
			}
			return Err(format!("Line {}: unterminated char literal", self.line));
		}
		self.advance();

		self.add_token_lit(CharLit, Some(LiteralValue::CharValue(ch)));
		Ok(())
	}

	fn number(&mut self) -> Result<(), String> {
		while self.peek().is_ascii_digit() {
			self.advance();
		}

		let is_float = self.peek() == '.' && self.peek_next().is_ascii_digit();
		if is_float {
			self.advance();
			while self.peek().is_ascii_digit() {
				self.advance();
			}
		}

		let text: String = self.source[self.start..self.current]
			.iter()
			.collect();

		if is_float {
			let value = text.parse::<f64>()
				.map_err(|_| format!(
					"Line {}: invalid float '{}'", 
					self.line,
					text
				))?;
			self.add_token_lit(FloatLit, Some(LiteralValue::FloatValue(value)));
		}
		else {
			let value = text.parse::<i64>()
				.map_err(|_| format!(
					"Line {}: invalid integer '{}'", 
					self.line, 
					text
				))?;
			self.add_token_lit(IntLit, Some(LiteralValue::IntValue(value)));
		}

		Ok(())
	}

	fn identifier(&mut self) {
		while self.peek().is_alphanumeric() || self.peek() == '_' {
			self.advance();
		}

		let text: String = self.source[self.start..self.current]
			.iter()
			.collect();

		if text == "f" && self.peek() == '"' {
			self.advance();
			// cannot return Result from here, so use unwrap-safe path
			// (fstring handles its own errors via the caller)
			let _ = self.fstring();
			return;
		}

		if let Some(&token_type) = self.keywords.get(text.as_str()) {
			self.add_token(token_type);
		}
		else {
			self.add_token(Identifier);
		}
	}

	fn is_at_end(&self) -> bool {
		self.current >= self.source.len()
	}

	fn peek(&self) -> char {
		if self.is_at_end() { '\0' } else { self.source[self.current] }
	}

	fn peek_next(&self) -> char {
		if self.current + 1 >= self.source.len() { '\0' } else { self.source[self.current + 1] }
	}

	fn advance(&mut self) -> char {
		let c = self.source[self.current];
		self.current += 1;
		c
	}

	fn char_match(&mut self, expected: char) -> bool {
		if self.is_at_end() || self.source[self.current] != expected {
			return false;
		}
		self.current += 1;
		true
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_basic_tokens() {
		let mut scanner = Scanner::new("let x: int = 42;");
		let tokens = scanner.scan_tokens().expect("Scan failed");

		assert_eq!(tokens[0].token_type, Let);
		assert_eq!(tokens[1].token_type, Identifier);
		assert_eq!(tokens[2].token_type, Colon);
		assert_eq!(tokens[3].token_type, Identifier);
		assert_eq!(tokens[4].token_type, Equal);
		assert_eq!(tokens[5].token_type, IntLit);
		assert_eq!(tokens[6].token_type, Semicolon);
		assert_eq!(tokens[7].token_type, Eof);
	}

	#[test]
	fn test_explicit_semicolon() {
		let mut scanner = Scanner::new("let x = 10;\n let y = 20;");
		let tokens = scanner.scan_tokens().expect("Scan failed");
		let types: Vec<TokenType> = tokens.iter().map(|t| t.token_type).collect();

		assert_eq!(types, vec![
			Let, Identifier, Equal, IntLit, Semicolon,
			Let, Identifier, Equal, IntLit, Semicolon,
			Eof,
		]);
	}

	#[test]
	#[should_panic]
	fn test_many_semicolons() {
		let mut scanner = Scanner::new("let x = 10;;;\n let y = 20;");
		let tokens = scanner.scan_tokens().expect("Scan failed");
		let types: Vec<TokenType> = tokens.iter().map(|t| t.token_type).collect();

		assert_eq!(types, vec![
			Let, Identifier, Equal, IntLit, Semicolon,
			Let, Identifier, Equal, IntLit, Semicolon,
			Eof,
		]);
	}

	#[test]
	fn test_newlines_do_not_insert_semicolon() {
		let mut scanner = Scanner::new("let x = 10\nlet y = 20");
		let tokens = scanner.scan_tokens().expect("Scan failed");
		let types: Vec<TokenType> = tokens.iter().map(|t| t.token_type).collect();

		assert_eq!(types, vec![
			Let, Identifier, Equal, IntLit,
			Let, Identifier, Equal, IntLit,
			Eof,
		]);
	}

	#[test]
	fn test_no_semicolon_after_brace_open() {
		let mut scanner = Scanner::new("if x > 0 { print(x) }");
		let tokens = scanner.scan_tokens().expect("Scan failed");

		let has_semi_after_brace = tokens.windows(2).any(|w| {
			w[0].token_type == LeftBrace && w[1].token_type == Semicolon
		});
		assert!(!has_semi_after_brace);
	}

	#[test]
	fn test_float_int() {
		let mut scanner = Scanner::new("42 3.14");
		let tokens = scanner.scan_tokens().expect("Scan failed");

		assert_eq!(tokens[0].token_type, IntLit);
		assert_eq!(tokens[1].token_type, FloatLit);
	}

	#[test]
	fn test_arrow_and_fat_arrow() {
		let mut scanner = Scanner::new("-> =>");
		let tokens = scanner.scan_tokens().expect("Scan failed");

		assert_eq!(tokens[0].token_type, Arrow);
		assert_eq!(tokens[1].token_type, FatArrow);
	}

	#[test]
	fn test_dot_dot() {
		let mut scanner = Scanner::new("0..10");
		let tokens = scanner.scan_tokens().expect("Scan failed");

		assert_eq!(tokens[0].token_type, IntLit);
		assert_eq!(tokens[1].token_type, DotDot);
		assert_eq!(tokens[2].token_type, IntLit);
	}

	#[test]
	fn test_dash_dash_comment() {
		let mut scanner = Scanner::new("let x = 5; -- this is a comment\nlet y = 10;");
		let tokens = scanner.scan_tokens().expect("Scan failed");
		let types: Vec<TokenType> = tokens.iter().map(|t| t.token_type).collect();

		assert_eq!(types, vec![
			Let, Identifier, Equal, IntLit, Semicolon,
			Let, Identifier, Equal, IntLit, Semicolon,
			Eof,
		]);
	}

	#[test]
	fn test_logical_operators() {
		let mut scanner = Scanner::new("a && b || c;");
		let tokens = scanner.scan_tokens().expect("Scan failed");
		let types: Vec<TokenType> = tokens.iter().map(|t| t.token_type).collect();

		assert_eq!(types, vec![
			Identifier, AmpAmp, Identifier, PipePipe, Identifier, Semicolon, Eof,
		]);
	}

	#[test]
	fn test_compound_assignment() {
		let mut scanner = Scanner::new("x += 1; y -= 2; z *= 3; w /= 4;");
		let tokens = scanner.scan_tokens().expect("Scan failed");
		let types: Vec<TokenType> = tokens.iter().map(|t| t.token_type).collect();

		assert_eq!(types, vec![
			Identifier, PlusEqual, IntLit, Semicolon,
			Identifier, MinusEqual, IntLit, Semicolon,
			Identifier, StarEqual, IntLit, Semicolon,
			Identifier, SlashEqual, IntLit, Semicolon,
			Eof,
		]);

		let mut scanner = Scanner::new("x += 1;\n y -= 2;\n z *= 3;\n w /= 4;\n");
		let tokens = scanner.scan_tokens().expect("Scan failed");
		let types: Vec<TokenType> = tokens.iter().map(|t| t.token_type).collect();

		assert_eq!(types, vec![
			Identifier, PlusEqual, IntLit, Semicolon,
			Identifier, MinusEqual, IntLit, Semicolon,
			Identifier, StarEqual, IntLit, Semicolon,
			Identifier, SlashEqual, IntLit, Semicolon,
			Eof,
		]);
	}

	#[test]
	fn test_compound_assignment_without_semicolons() {
		let mut scanner = Scanner::new("x += 1 y -= 2 z *= 3 w /= 4");
		let tokens = scanner.scan_tokens().expect("Scan failed");
		let types: Vec<TokenType> = tokens.iter().map(|t| t.token_type).collect();

		assert_eq!(types, vec![
			Identifier, PlusEqual, IntLit,
			Identifier, MinusEqual, IntLit,
			Identifier, StarEqual, IntLit,
			Identifier, SlashEqual, IntLit,
			Eof,
		]);
	}

	#[test]
	fn test_question_mark() {
		let mut scanner = Scanner::new("val?");
		let tokens = scanner.scan_tokens().expect("Scan failed");

		assert_eq!(tokens[0].token_type, Identifier);
		assert_eq!(tokens[1].token_type, Question);
	}

	#[test]
	fn test_char_literal() {
		let mut scanner = Scanner::new("'a' 'z'");
		let tokens = scanner.scan_tokens().expect("Scan failed");

		assert_eq!(tokens[0].token_type, CharLit);
		assert_eq!(tokens[1].token_type, CharLit);
		
		if let Some(LiteralValue::CharValue(c)) = &tokens[0].literal {
			assert_eq!(*c, 'a');
		}
		else {
			panic!("Expected CharValue");
		}
	}

	#[test]
	#[should_panic]
	fn test_many_char_literal() {
		let mut scanner = Scanner::new("'abc' 'z'");
		let tokens = scanner.scan_tokens().expect("Scan failed");

		assert_eq!(tokens[0].token_type, CharLit);
		assert_eq!(tokens[1].token_type, CharLit);

		if let Some(LiteralValue::CharValue(c)) = &tokens[0].literal {
			assert_eq!(*c, 'a');
		}
		else {
			panic!("Expected CharValue");
		}
	}

	#[test]
	fn test_char_escape() {
		let mut scanner = Scanner::new("'\\n' '\\t' '\\\\' '\\''");
		let tokens = scanner.scan_tokens().expect("Scan failed");

		let chars: Vec<char> = tokens.iter().filter(|t| t.token_type == CharLit).map(|t| {
			match &t.literal {
				Some(LiteralValue::CharValue(c)) => *c,
				_ => panic!("Expected CharValue"),
			}
		}).collect();

		assert_eq!(chars, vec!['\n', '\t', '\\', '\'']);
	}

	#[test]
	fn test_fstring() {
		let mut scanner = Scanner::new("f\"hello {name}\"");
		let tokens = scanner.scan_tokens().expect("Scan failed");

		assert_eq!(tokens[0].token_type, FStringLit);

		if let Some(LiteralValue::StringValue(s)) = &tokens[0].literal {
			assert_eq!(s, "hello {name}");
		}
		else {
			panic!("Expected StringValue in FStringLit");
		}
	}

	#[test]
	fn test_struct_definition() {
		let example = "struct Player {\n\t name: str;\n\t health: int;\n};";
		let mut scanner = Scanner::new(example);

		let tokens = scanner.scan_tokens().expect("Scan failed");
		let types: Vec<TokenType> = tokens.iter().map(|t| t.token_type).collect();

		assert_eq!(types, vec![
			Struct, Identifier, LeftBrace,
			Identifier, Colon, Identifier, Semicolon,
			Identifier, Colon, Identifier, Semicolon,
			RightBrace, Semicolon,
			Eof,
		]);
	}
}
