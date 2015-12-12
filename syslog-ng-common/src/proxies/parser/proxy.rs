use syslog_ng_sys::LogMessage;
use syslog_ng_sys::LogParser;

pub use proxies::parser::{
    OptionError,
    Parser,
    RustParserBuilder
};

#[repr(C)]
#[derive(Clone)]
pub struct RustParserProxy<B> where B: RustParserBuilder {
    pub parser: Option<B::Parser>,
    pub builder: Option<B>
}

impl<B> RustParserProxy<B> where B: RustParserBuilder {
    pub fn new() -> RustParserProxy<B> {
        RustParserProxy {
            parser: None,
            builder: Some(B::new())
        }
    }

    pub fn init(&mut self) -> bool {
        let builder = self.builder.take().expect("Called init when builder was not set");
        match builder.build() {
            Ok(parser) => {
                self.parser = Some(parser);
                true
            },
            Err(error) => {
                error!("Error: {:?}", error);
                false
            }
        }
    }

    pub fn set_option(&mut self, name: String, value: String) {
        if self.builder.is_none() {
            self.builder = Some(B::new());
        }

        let builder = self.builder.as_mut().expect("Failed to get builder on a RustParserProxy");
        builder.option(name, value);
    }

    pub fn process(&mut self, msg: &mut LogMessage, input: &str) -> bool {
        self.parser.as_mut().expect("Called process on a non-existing Rust parser").parse(msg, input)
    }

    pub fn parent(&mut self, parent: *mut LogParser) {
        let builder = self.builder.as_mut().expect("Failed to get a builder on a new parser proxy instance");
        builder.parent(parent);
    }
}
