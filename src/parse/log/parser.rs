use std::sync::LazyLock;

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

/// Глобальный парсер
pub static LOG_LINE_PARSER: LazyLock<<LogLine as Parsable>::Parser> =
    LazyLock::new(LogLine::parser);

/// Парсер строки логов (wrapper for consistency)
pub struct LogLineParser;

impl LogLineParser {
    pub fn parse(input: &str) -> Result<(&str, LogLine), ()> {
        LOG_LINE_PARSER.parse(input)
    }
}

pub fn just_parse<T: Parsable>(input: &str) -> Result<(&str, T), ()> {
    T::parser().parse(input)
}
