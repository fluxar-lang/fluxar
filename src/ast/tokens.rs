#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
	// single char
	LeftParen, RightParen,
	LeftBrace, RightBrace,
	LeftBracket, RightBracket,
	Comma, Dot, Colon, Semicolon,
	Plus, Minus, Star, Slash, Percent,
	Ampersand, Underscore, Question,

	// one or two chars
	Bang, BangEqual,
	Equal, EqualEqual,
	Greater, GreaterEqual,
	Less, LessEqual,
	Arrow,
	FatArrow,
	DotDot,
	ColonColon,
	AmpAmp,
	PipePipe,
	PlusEqual,
	MinusEqual,
	StarEqual,
	SlashEqual,

	// literals
	Identifier,
	StringLit,
	FStringLit,
	CharLit,
	IntLit,
	FloatLit,

	// keywords
	Fn, Let, Return,
	Struct, Enum, Extend, Interface,
	Match, If, Else, While, For, In, Loop,
	Break, Continue,
	Defer, Region,
	Use, Public,
	Async,

	True, False,
	Or,      	// unwrap-or fallback
	As,      	// type cast
	SelfKw,  	// self
	Mut,
	Move,

	Eof,
}

impl std::fmt::Display for TokenType {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{:?}", self)
	}
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LiteralValue {
	IntValue(i64),
	FloatValue(f64),
	StringValue(String),
	// TODO:
	CharValue(char),
}

#[derive(Debug, Clone)]
pub struct Token {
	pub token_type: TokenType,
	pub lexeme: String,
	pub literal: Option<LiteralValue>,
	pub line_number: usize,
}

impl Token {
	pub fn _to_string(&self) -> String {
		format!("{} {} {:?}", self.token_type, self.lexeme, self.literal)
	}
}
