use crate::parse::combinators::basic::*;
use crate::parse::combinators::choice::*;
use crate::parse::log::kinds::*;
use crate::parse::primitives::stdp;
use crate::parse::traits::{Parsable, Parser};

/// Строка логов, [лог](AppLogKind) с `request_id`
#[derive(Debug, Clone, PartialEq)]
pub struct LogLine {
    pub kind: LogKind,
    pub request_id: u32,
}
impl Parsable for LogLine {
    type Parser = Map<
        All<(
            <LogKind as Parsable>::Parser,
            StripWhitespace<Preceded<Tag, stdp::U32>>,
        )>,
        fn((LogKind, u32)) -> Self,
    >;
    fn parser() -> Self::Parser {
        map(
            all2(
                LogKind::parser(),
                strip_whitespace(preceded(tag("requestid="), stdp::U32)),
            ),
            |(kind, request_id)| LogLine { kind, request_id },
        )
    }
}

/// Парсер строки логов
pub struct LogLineParser {
    parser: std::sync::OnceLock<<LogLine as Parsable>::Parser>,
}
impl LogLineParser {
    pub fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, LogLine), ()> {
        self.parser
            .get_or_init(<LogLine as Parsable>::parser)
            .parse(input)
    }
}
// подсказка: singleton, без которого можно обойтись
// парсеры не страшно вытащить в pub
/// Единожды собранный парсер логов
pub static LOG_LINE_PARSER: LogLineParser = LogLineParser {
    parser: std::sync::OnceLock::new(),
};

pub fn just_parse<T: Parsable>(input: &str) -> Result<(&str, T), ()> {
    T::parser().parse(input)
}
