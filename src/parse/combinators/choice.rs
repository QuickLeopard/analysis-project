use crate::parse::combinators::basic::*;
use crate::parse::traits::Parser;

/// Комбинатор, который вернёт тот результат, который будет успешно
/// получен первым из дочерних комбинаторов
/// (аналог `alt` из `nom`)
#[derive(Debug, Clone)]
pub struct Alt<T> {
    parser: T,
}
impl<A0, A1, Dest> Parser for Alt<(A0, A1)>
where
    A0: Parser<Dest = Dest>,
    A1: Parser<Dest = Dest>,
{
    type Dest = Dest;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        if let Ok(ok) = self.parser.0.parse(input) {
            return Ok(ok);
        }
        self.parser.1.parse(input)
    }
}
/// Конструктор [Alt] для двух парсеров
/// (в Rust нет чего-то, вроде variadic templates из C++)
pub fn alt2<Dest, A0: Parser<Dest = Dest>, A1: Parser<Dest = Dest>>(
    a0: A0,
    a1: A1,
) -> Alt<(A0, A1)> {
    Alt { parser: (a0, a1) }
}
impl<A0, A1, A2, Dest> Parser for Alt<(A0, A1, A2)>
where
    A0: Parser<Dest = Dest>,
    A1: Parser<Dest = Dest>,
    A2: Parser<Dest = Dest>,
{
    type Dest = Dest;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        // match вместо тут не подойдёт - нужно лениво
        if let Ok(ok) = self.parser.0.parse(input) {
            return Ok(ok);
        }
        if let Ok(ok) = self.parser.1.parse(input) {
            return Ok(ok);
        }
        self.parser.2.parse(input)
    }
}

/// Alt for dynamic collection of different parser types
pub type DynParsers<Dest> = Vec<Box<dyn Parser<Dest = Dest>>>;

impl<Dest> Parser for Alt<DynParsers<Dest>> {
    type Dest = Dest;

    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        self.parser
            .iter()
            .find_map(|parser| parser.parse(input).ok())
            .ok_or(())
    }
}

/// Constructor for dynamic alternatives
pub fn alternative_from_n<Dest>(
    alternatives: Vec<Box<dyn Parser<Dest = Dest>>>,
) -> Alt<DynParsers<Dest>> {
    Alt {
        parser: alternatives,
    }
}

/// Конструктор [Alt] для трёх парсеров
/// (в Rust нет чего-то, вроде variadic templates из C++)
pub fn alt3<Dest, A0: Parser<Dest = Dest>, A1: Parser<Dest = Dest>, A2: Parser<Dest = Dest>>(
    a0: A0,
    a1: A1,
    a2: A2,
) -> Alt<(A0, A1, A2)> {
    Alt {
        parser: (a0, a1, a2),
    }
}
impl<A0, A1, A2, A3, Dest> Parser for Alt<(A0, A1, A2, A3)>
where
    A0: Parser<Dest = Dest>,
    A1: Parser<Dest = Dest>,
    A2: Parser<Dest = Dest>,
    A3: Parser<Dest = Dest>,
{
    type Dest = Dest;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        if let Ok(ok) = self.parser.0.parse(input) {
            return Ok(ok);
        }
        if let Ok(ok) = self.parser.1.parse(input) {
            return Ok(ok);
        }
        if let Ok(ok) = self.parser.2.parse(input) {
            return Ok(ok);
        }
        self.parser.3.parse(input)
    }
}
/// Конструктор [Alt] для четырёх парсеров
/// (в Rust нет чего-то, вроде variadic templates из C++)
pub fn alt4<
    Dest,
    A0: Parser<Dest = Dest>,
    A1: Parser<Dest = Dest>,
    A2: Parser<Dest = Dest>,
    A3: Parser<Dest = Dest>,
>(
    a0: A0,
    a1: A1,
    a2: A2,
    a3: A3,
) -> Alt<(A0, A1, A2, A3)> {
    Alt {
        parser: (a0, a1, a2, a3),
    }
}
impl<A0, A1, A2, A3, A4, A5, A6, A7, Dest> Parser for Alt<(A0, A1, A2, A3, A4, A5, A6, A7)>
where
    A0: Parser<Dest = Dest>,
    A1: Parser<Dest = Dest>,
    A2: Parser<Dest = Dest>,
    A3: Parser<Dest = Dest>,
    A4: Parser<Dest = Dest>,
    A5: Parser<Dest = Dest>,
    A6: Parser<Dest = Dest>,
    A7: Parser<Dest = Dest>,
{
    type Dest = Dest;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        if let Ok(ok) = self.parser.0.parse(input) {
            return Ok(ok);
        }
        if let Ok(ok) = self.parser.1.parse(input) {
            return Ok(ok);
        }
        if let Ok(ok) = self.parser.2.parse(input) {
            return Ok(ok);
        }
        if let Ok(ok) = self.parser.3.parse(input) {
            return Ok(ok);
        }
        if let Ok(ok) = self.parser.4.parse(input) {
            return Ok(ok);
        }
        if let Ok(ok) = self.parser.5.parse(input) {
            return Ok(ok);
        }
        if let Ok(ok) = self.parser.6.parse(input) {
            return Ok(ok);
        }
        self.parser.7.parse(input)
    }
}
/// Конструктор [Alt] для восьми парсеров
/// (в Rust нет чего-то, вроде variadic templates из C++)
pub fn alt8<
    Dest,
    A0: Parser<Dest = Dest>,
    A1: Parser<Dest = Dest>,
    A2: Parser<Dest = Dest>,
    A3: Parser<Dest = Dest>,
    A4: Parser<Dest = Dest>,
    A5: Parser<Dest = Dest>,
    A6: Parser<Dest = Dest>,
    A7: Parser<Dest = Dest>,
>(
    a0: A0,
    a1: A1,
    a2: A2,
    a3: A3,
    a4: A4,
    a5: A5,
    a6: A6,
    a7: A7,
) -> Alt<(A0, A1, A2, A3, A4, A5, A6, A7)> {
    Alt {
        parser: (a0, a1, a2, a3, a4, a5, a6, a7),
    }
}
/// Комбинатор, который требует, чтобы все дочерние парсеры отработали,
/// (аналог `all` из `nom`)
#[derive(Debug, Clone)]
pub struct All<T> {
    parser: T,
}
impl<A0, A1> Parser for All<(A0, A1)>
where
    A0: Parser,
    A1: Parser,
{
    type Dest = (A0::Dest, A1::Dest);
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        let (remaining, a0) = self.parser.0.parse(input)?;
        self.parser
            .1
            .parse(remaining)
            .map(|(remaining, a1)| (remaining, (a0, a1)))
    }
}
/// Конструктор [All] для двух парсеров
/// (в Rust нет чего-то, вроде variadic templates из C++)
pub fn all2<A0: Parser, A1: Parser>(a0: A0, a1: A1) -> All<(A0, A1)> {
    All { parser: (a0, a1) }
}
impl<A0, A1, A2> Parser for All<(A0, A1, A2)>
where
    A0: Parser,
    A1: Parser,
    A2: Parser,
{
    type Dest = (A0::Dest, A1::Dest, A2::Dest);
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        let (remaining, a0) = self.parser.0.parse(input)?;
        let (remaining, a1) = self.parser.1.parse(remaining)?;
        self.parser
            .2
            .parse(remaining)
            .map(|(remaining, a2)| (remaining, (a0, a1, a2)))
    }
}
/// Конструктор [All] для трёх парсеров
/// (в Rust нет чего-то, вроде variadic templates из C++)
pub fn all3<A0: Parser, A1: Parser, A2: Parser>(a0: A0, a1: A1, a2: A2) -> All<(A0, A1, A2)> {
    All {
        parser: (a0, a1, a2),
    }
}
impl<A0, A1, A2, A3> Parser for All<(A0, A1, A2, A3)>
where
    A0: Parser,
    A1: Parser,
    A2: Parser,
    A3: Parser,
{
    type Dest = (A0::Dest, A1::Dest, A2::Dest, A3::Dest);
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        let (remaining, a0) = self.parser.0.parse(input)?;
        let (remaining, a1) = self.parser.1.parse(remaining)?;
        let (remaining, a2) = self.parser.2.parse(remaining)?;
        self.parser
            .3
            .parse(remaining)
            .map(|(remaining, a3)| (remaining, (a0, a1, a2, a3)))
    }
}
/// Конструктор [All] для четырёх парсеров
/// (в Rust нет чего-то, вроде variadic templates из C++)
pub fn all4<A0: Parser, A1: Parser, A2: Parser, A3: Parser>(
    a0: A0,
    a1: A1,
    a2: A2,
    a3: A3,
) -> All<(A0, A1, A2, A3)> {
    All {
        parser: (a0, a1, a2, a3),
    }
}

/// Комбинатор, который вытаскивает значения из пары `"ключ":значение,`.
/// Для простоты реализации, запятая всегда нужна в конце пары ключ-значение,
/// простое '"ключ":значение' читаться не будет
#[derive(Debug, Clone)]
pub struct KeyValue<T> {
    parser: Delimited<
        All<(StripWhitespace<QuotedTag>, StripWhitespace<Tag>)>,
        StripWhitespace<T>,
        StripWhitespace<Tag>,
    >,
}
impl<T> Parser for KeyValue<T>
where
    T: Parser,
{
    type Dest = T::Dest;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        self.parser.parse(input)
    }
}
/// Конструктор [KeyValue]
pub fn key_value<T: Parser>(key: &'static str, value_parser: T) -> KeyValue<T> {
    KeyValue {
        parser: delimited(
            all2(
                strip_whitespace(quoted_tag(key)),
                strip_whitespace(tag(":")),
            ),
            strip_whitespace(value_parser),
            strip_whitespace(tag(",")),
        ),
    }
}
