#![allow(dead_code)]

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
	Int, Float,
	I8, I16, I32, I64, I128,
	U8, U16, U32, U64, U128,
	ISize, USize,
	F32, F64,
	Bool, Char, Str, Unit,
	Struct(String),
	Generic(String),
}

impl Type {
	pub fn from_str(s: &str) -> Self {
		match s {
			"int" => Type::Int,
			"float" => Type::Float,
			"i8" => Type::I8,
			"i16" => Type::I16,
			"i32" => Type::I32,
			"i64" => Type::I64,
			"i128" => Type::I128,
			"u8" => Type::U8,
			"u16" => Type::U16,
			"u32" => Type::U32,
			"u64" => Type::U64,
			"u128" => Type::U128,
			"isize" => Type::ISize,
			"usize" => Type::USize,
			"f32" => Type::F32,
			"f64" => Type::F64,
			"bool" => Type::Bool,
			"char" => Type::Char,
			"str" => Type::Str,
			"()" => Type::Unit,
			name => Type::Struct(name.to_string()),
		}
	}

	pub fn is_numeric(&self) -> bool {
		matches!(self,
			Type::Int | Type::Float |
			Type::I8 | Type::I16 | Type::I32 | Type::I64 | Type::I128 |
			Type::U8 | Type::U16 | Type::U32 | Type::U64 | Type::U128 |
			Type::ISize | Type::USize | Type::F32 | Type::F64
		)
	}

	pub fn is_integer(&self) -> bool {
		matches!(self,
			Type::Int |
			Type::I8 | Type::I16 | Type::I32 | Type::I64 | Type::I128 |
			Type::U8 | Type::U16 | Type::U32 | Type::U64 | Type::U128 |
			Type::ISize | Type::USize
		)
	}

	pub fn is_float(&self) -> bool {
		matches!(self, Type::Float | Type::F32 | Type::F64)
	}

	pub fn is_signed(&self) -> bool {
		matches!(self,
			Type::Int | Type::Float |
			Type::I8 | Type::I16 | Type::I32 | Type::I64 | Type::I128 |
			Type::ISize | Type::F32 | Type::F64
		)
	}
}

impl std::fmt::Display for Type {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Type::Int => write!(f, "int"),
			Type::Float => write!(f, "float"),
			Type::I8 => write!(f, "i8"),
			Type::I16 => write!(f, "i16"),
			Type::I32 => write!(f, "i32"),
			Type::I64 => write!(f, "i64"),
			Type::I128 => write!(f, "i128"),
			Type::U8 => write!(f, "u8"),
			Type::U16 => write!(f, "u16"),
			Type::U32 => write!(f, "u32"),
			Type::U64 => write!(f, "u64"),
			Type::U128 => write!(f, "u128"),
			Type::ISize => write!(f, "isize"),
			Type::USize => write!(f, "usize"),
			Type::F32 => write!(f, "f32"),
			Type::F64 => write!(f, "f64"),
			Type::Bool => write!(f, "bool"),
			Type::Char => write!(f, "char"),
			Type::Str => write!(f, "str"),
			Type::Unit => write!(f, "()"),
			Type::Struct(name) => write!(f, "{}", name),
			Type::Generic(name) => write!(f, "{}", name),
		}
	}
}