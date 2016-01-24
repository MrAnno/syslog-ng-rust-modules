// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#[macro_use]
extern crate log;

#[macro_use]
extern crate syslog_ng_common;
extern crate actiondb;

use std::borrow::Borrow;
use std::clone::Clone;

use actiondb::matcher::{Matcher, PatternLoader};
use actiondb::matcher::trie::TrieMatcherSuite;
use actiondb::matcher::suite::MatcherSuite;
use actiondb::matcher::MatcherFactory;

use syslog_ng_common::{Parser, ParserBuilder, OptionError, LogParser, LogMessage, MessageFormatter};

pub mod msgfilller;
pub mod keys;
pub mod options;

use self::msgfilller::MessageFiller;

type DefaultMatcherSuite = TrieMatcherSuite;

#[derive(Clone)]
pub struct ActiondbParserBuilder {
    matcher: Option<<DefaultMatcherSuite as MatcherSuite>::Matcher>,
    formatter: MessageFormatter
}

impl ActiondbParserBuilder {
    pub fn set_pattern_file(&mut self, path: &str) {
        match PatternLoader::from_file::<<DefaultMatcherSuite as MatcherSuite>::MatcherFactory>(path) {
            Ok(matcher) => self.matcher = Some(matcher),
            Err(err) => {
                error!("ActiondbParser: failed to set 'pattern_file': {}", err);
            }
        }
    }

    pub fn set_prefix(&mut self, prefix: String) {
        self.formatter.set_prefix(prefix);
    }
}

impl ParserBuilder for ActiondbParserBuilder {
    type Parser = ActiondbParser;
    fn new() -> Self {
        ActiondbParserBuilder {
            matcher: None,
            formatter: MessageFormatter::new(),
        }
    }
    fn option(&mut self, name: String, value: String) {
        trace!("ActiondbParserBuilder: set_option(name={}, value={})",
               &name,
               &value);

        match name.borrow() {
            options::PATTERN_FILE => {
                self.set_pattern_file(&value);
            }
            options::PREFIX => {
                self.set_prefix(value);
            }
            _ => {
                debug!("ActiondbParserBuilder: unsupported option: {:?}", &name) ;
            }
        };

    }
    fn parent(&mut self, _: LogParser) {}
    fn build(self) -> Result<Self::Parser, OptionError> {
        let ActiondbParserBuilder {matcher, formatter} = self;
        debug!("ActiondbParser: building");
        let matcher =
            try!(matcher.ok_or(OptionError::missing_required_option(options::PATTERN_FILE)));
        Ok(ActiondbParser {
            matcher: matcher,
            formatter: formatter,
        })
    }
}

pub struct ActiondbParser {
    matcher: <DefaultMatcherSuite as MatcherSuite>::Matcher,
    formatter: MessageFormatter,
}

impl Parser for ActiondbParser {
    fn parse(&mut self, msg: &mut LogMessage, input: &str) -> bool {
        if let Some(result) = self.matcher.parse(input) {
            MessageFiller::fill_logmsg(&mut self.formatter, msg, &result);
            true
        } else {
            false
        }
    }
}

impl Clone for ActiondbParser {
    fn clone(&self) -> ActiondbParser {
        ActiondbParser {
            matcher: self.matcher.clone(),
            formatter: self.formatter.clone(),
        }
    }
}

parser_plugin!(ActiondbParserBuilder);
