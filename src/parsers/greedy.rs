use std::hash::{SipHasher, Hash, Hasher};
use super::{ParserBase, Parser, ObjectSafeHash, ParseResult};

#[derive(Clone, Debug, Hash)]
pub struct GreedyParser {
    base: ParserBase,
    end_string: Option<String>
}

impl GreedyParser {
    pub fn new(name: String) -> GreedyParser {
        GreedyParser{ base: ParserBase::with_name(name),
                       end_string: None }
    }

    pub fn from_str(name: &str, end_string: &str) -> GreedyParser {
        let mut parser = GreedyParser::new(name.to_string());
        parser.set_end_string(end_string.to_string());
        parser
    }

    pub fn set_end_string(&mut self, end_string: String) {
        self.end_string = Some(end_string);
    }
}

impl ObjectSafeHash for GreedyParser {
    fn hash_os(&self) -> u64 {
        let mut hasher = SipHasher::new();
        "parser:greedy".hash(&mut hasher);
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl Parser for GreedyParser {
    fn parse<'a, 'b>(&'a self, value: &'b str) -> Option<ParseResult<'a, 'b>> {
        if self.end_string.is_none() {
            return Some(ParseResult::new(self, &value[..]))
        }

        if let Some(pos) = value.find(&self.end_string.as_ref().unwrap()[..]) {
            Some(ParseResult::new(self, &value[..pos]))
        } else {
            None
        }
    }

    fn name(&self) -> Option<&str> {
        self.base.name()
    }

    fn boxed_clone(&self) -> Box<Parser> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod test {
    use parsers::{GreedyParser, Parser};

    #[test]
    fn test_given_greedy_parser_when_the_end_string_is_not_found_in_the_value_then_the_parser_doesnt_match() {
        let parser = GreedyParser::from_str("name", "foo");
        assert_eq!(parser.parse("qux baz bar").is_none(), true);
    }

    #[test]
    fn test_given_greedy_parser_when_the_end_string_is_found_in_the_value_then_the_parser_matches() {
        let parser = GreedyParser::from_str("name", "foo");
        let res = parser.parse("qux foo bar").unwrap();
        assert_eq!(res.parser().name(), Some("name"));
        assert_eq!(res.value(), "qux ");
    }
}
