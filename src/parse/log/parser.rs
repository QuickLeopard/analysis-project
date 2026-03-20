use std::sync::LazyLock;

use crate::parse::combinators::basic::*;
use crate::parse::combinators::choice::*;
use crate::parse::log::kinds::*;
use crate::parse::primitives::stdp;
use crate::parse::traits::{Parsable, Parser};

/// Полностью распарсенная строка лога.
///
/// Формат: `<LogKind> requestid=<u32>`.
#[derive(Debug, Clone, PartialEq)]
pub struct LogLine {
    /// Содержимое и тип события.
    pub kind: LogKind,
    /// Идентификатор запроса, связывающий события между собой.
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

/// Ленивая глобальная инстанция парсера строки логов.
pub static LOG_LINE_PARSER: LazyLock<<LogLine as Parsable>::Parser> =
    LazyLock::new(LogLine::parser);

/// Тонкая обёртка над [LOG_LINE_PARSER] для единообразного вызова.
pub struct LogLineParser;

impl LogLineParser {
    /// Пытается распарсить одну строку лога.
    pub fn parse(input: &str) -> Result<(&str, LogLine), ()> {
        LOG_LINE_PARSER.parse(input)
    }
}

/// Вспомогательная функция для быстрого парсинга любого [Parsable]-типа.
pub fn just_parse<T: Parsable>(input: &str) -> Result<(&str, T), ()> {
    T::parser().parse(input)
}
