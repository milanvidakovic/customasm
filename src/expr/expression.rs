use diagn::Span;
use num::BigInt;


#[derive(Debug)]
pub enum Expression
{
	Literal(Span, ExpressionValue),
	Variable(Span, String),
	UnaryOp(Span, Span, UnaryOp, Box<Expression>),
	BinaryOp(Span, Span, BinaryOp, Box<Expression>, Box<Expression>),
	BitSlice(Span, Span, usize, usize, Box<Expression>)
}


#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExpressionValue
{
	Integer(BigInt),
	Bool(bool)
}


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ExpressionType
{
	Integer,
	Bool
}


#[derive(Copy, Clone, Debug)]
pub enum UnaryOp
{
	Neg,
	Not
}


#[derive(Copy, Clone, Debug)]
pub enum BinaryOp
{
	Add, Sub, Mul, Div, Mod,
	Shl, Shr,
	And, Or, Xor,
	
	Eq, Ne,
	Lt, Le,
	Gt, Ge,
	
	LazyAnd, LazyOr,
	
	Concat
}


impl Expression
{
	pub fn span(&self) -> Span
	{
		match self
		{
			&Expression::Literal (ref span, ..) => span.clone(),
			&Expression::Variable(ref span, ..) => span.clone(),
			&Expression::UnaryOp (ref span, ..) => span.clone(),
			&Expression::BinaryOp(ref span, ..) => span.clone(),
			&Expression::BitSlice(ref span, ..) => span.clone()
		}
	}
}


impl ExpressionValue
{
	pub fn is_of_type(&self, typ: ExpressionType) -> bool
	{
		match (self, typ)
		{
			(&ExpressionValue::Integer(_), ExpressionType::Integer) |
			(&ExpressionValue::Bool(_),    ExpressionType::Bool)
				=> true,
				
			_ => false
		}
	}
}