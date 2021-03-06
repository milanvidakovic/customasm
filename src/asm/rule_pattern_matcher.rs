use syntax::{TokenKind, Parser};
use expr::Expression;
use instrset::{Rule, RulePatternPart};
use std::collections::HashMap;


#[derive(Debug)]
pub struct RulePatternMatcher
{
	root_step: MatchStep
}


#[derive(Debug)]
struct MatchStep
{
	rule_indices: Vec<usize>,
	children_exact: HashMap<MatchStepExact, MatchStep>,
	children_param: HashMap<MatchStepParameter, MatchStep>
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct MatchStepExact(TokenKind, Option<String>);


#[derive(Debug, Eq, PartialEq, Hash)]
struct MatchStepParameter;


#[derive(Debug)]
pub struct Match
{
	pub rule_indices: Vec<usize>,
	pub exprs: Vec<Expression>
}


impl RulePatternMatcher
{
	pub fn new(rules: &[Rule]) -> RulePatternMatcher
	{
		let mut root_step = MatchStep::new();
		
		for i in 0..rules.len()
			{ RulePatternMatcher::build_step(&mut root_step, &rules[i].pattern_parts, i); }
		
		
		RulePatternMatcher
		{
			root_step: root_step
		}
	}
	

	fn build_step(step: &mut MatchStep, next_parts: &[RulePatternPart], rule_index: usize)
	{
		if next_parts.len() == 0
		{
			step.rule_indices.push(rule_index);
			step.rule_indices.sort();
			return;
		}
		
		match next_parts[0]
		{
			RulePatternPart::Exact(kind, ref excerpt) =>
			{
				let step_kind = MatchStepExact(kind, excerpt.clone());
				
				if let Some(next_step) = step.children_exact.get_mut(&step_kind)
				{
					RulePatternMatcher::build_step(next_step, &next_parts[1..], rule_index);
					return;
				}
				
				let mut next_step = MatchStep::new();
				RulePatternMatcher::build_step(&mut next_step, &next_parts[1..], rule_index);
				step.children_exact.insert(step_kind, next_step);
			}
			
			RulePatternPart::Parameter(_) =>
			{
				let step_kind = MatchStepParameter;
				
				if let Some(next_step) = step.children_param.get_mut(&step_kind)
				{
					RulePatternMatcher::build_step(next_step, &next_parts[1..], rule_index);
					return;
				}
				
				let mut next_step = MatchStep::new();
				RulePatternMatcher::build_step(&mut next_step, &next_parts[1..], rule_index);
				step.children_param.insert(step_kind, next_step);
			}
		}
	}
	
	
	pub fn parse_match(&self, parser: &mut Parser) -> Option<Match>
	{
		let mut exprs = Vec::new();
		
		match self.parse_match_step(parser, &self.root_step, &mut exprs)
		{
			Some(indices) =>
			{
				let result = Match
				{
					rule_indices: indices.iter().cloned().collect(),
					exprs: exprs
				};
				
				Some(result)
			}
			
			None => None
		}
	}
	
	
	fn parse_match_step<'s>(&'s self, parser: &mut Parser, step: &'s MatchStep, exprs: &mut Vec<Expression>) -> Option<&'s [usize]>
	{
		if !parser.next_is_linebreak()
		{
			// Try to match fixed tokens first, if some rule accepts that.
			let parser_state = parser.save(); 
			
			let tk = parser.advance();
			let step_exact = MatchStepExact(tk.kind, tk.excerpt);
			
			if let Some(next_step) = step.children_exact.get(&step_exact)
			{
				if let Some(result) = self.parse_match_step(parser, &next_step, exprs)
					{ return Some(result); }
			}
			
			parser.restore(parser_state);
			
			// Then try to match argument expressions, if some rule accepts that.
			if let Some(next_step) = step.children_param.get(&MatchStepParameter)
			{
				let parser_state = parser.save(); 
				
				let expr = match Expression::parse(parser)
				{
					Ok(expr) => expr,
					Err(()) => return None
				};
				
				exprs.push(expr);
				
				if let Some(result) = self.parse_match_step(parser, &next_step, exprs)
					{ return Some(result); }
					
				exprs.pop();
				parser.restore(parser_state);
			}
		}
		
		// Finally, return a match if some rule ends here.
		if step.rule_indices.len() != 0
		{
			if !parser.next_is_linebreak()
				{ return None }
			
			return Some(&step.rule_indices);
		}
		
		// Else, return no match.
		None
	}
}


impl MatchStep
{
	fn new() -> MatchStep
	{
		MatchStep
		{
			rule_indices: Vec::new(),
			children_exact: HashMap::new(),
			children_param: HashMap::new()
		}
	}
}