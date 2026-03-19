use crate::parse::traits::Parser;

/// Обернуть строку в кавычки, экранировав кавычки, которые в строке уже есть
pub fn quote(input: &str) -> String {
    let mut result = String::from("\"");
    result.extend(input.chars().flat_map(|c| match c {
        '\\' | '"' => ['\\', c].into_iter().take(2),
        _ => [c, ' '].into_iter().take(1),
    }));
    result.push('"');
    result
}
/// Распарсить строку, которую ранее [обернули в кавычки](quote)
// `"abc\"def\\ghi"nice` -> (`abcd"def\ghi`, `nice`)
pub fn do_unquote(input: &str) -> Result<(&str, String), ()> {
    let mut result = String::new();
    let mut escaped_now = false;
    let mut chars = input.strip_prefix("\"").ok_or(())?.chars();
    while let Some(c) = chars.next() {
        match (c, escaped_now) {
            ('"' | '\\', true) => {
                result.push(c);
                escaped_now = false;
            }
            ('\\', false) => escaped_now = true,
            ('"', false) => return Ok((chars.as_str(), result)),
            (c, _) => {
                result.push(c);
                escaped_now = false;
            }
        }
    }
    Err(()) // строка кончилась, не закрыв кавычку
}
/// Распарсить строку, обёрную в кавычки
/// (сокращённая версия [do_unquote], в которой вложенные кавычки не предусмотрены)
pub fn do_unquote_non_escaped(input: &str) -> Result<(&str, &str), ()> {
    let input = input.strip_prefix("\"").ok_or(())?;
    let quote_byteidx = input.find('"').ok_or(())?;
    if 0 == quote_byteidx || Some("\\") == input.get(quote_byteidx - 1..quote_byteidx) {
        return Err(());
    }
    Ok((&input[1 + quote_byteidx..], &input[..quote_byteidx]))
}
/// Парсер кавычек
#[derive(Debug, Clone)]
pub struct Unquote;
impl Parser for Unquote {
    type Dest = String; // ← Must be String, not &'a str
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        let (remaining, unquoted) = do_unquote(input)?;
        Ok((remaining, unquoted))
    }
}
/// Конструктор [Unquote]
pub fn unquote() -> Unquote {
    Unquote
}
/// Парсер, возвращающий результат как есть
#[derive(Debug, Clone)]
pub struct AsIs;
impl Parser for AsIs {
    type Dest = String;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        Ok((&input[input.len()..], input.into()))
    }
}
/// Парсер константных строк
/// (аналог `nom::bytes::complete::tag`)
#[derive(Debug, Clone)]
pub struct Tag {
    tag: &'static str,
}
impl Parser for Tag {
    type Dest = ();
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        Ok((input.strip_prefix(self.tag).ok_or(())?, ()))
    }
}
/// Конструктор [Tag]
pub fn tag(tag: &'static str) -> Tag {
    Tag { tag }
}
/// Парсер [тэга](Tag), обёрнутого в кавычки
#[derive(Debug, Clone)]
pub struct QuotedTag(Tag);
impl Parser for QuotedTag {
    type Dest = ();
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        let (remaining, candidate) = do_unquote_non_escaped(input)?;
        if !self.0.parse(candidate)?.0.is_empty() {
            return Err(());
        }
        Ok((remaining, ()))
    }
}
/// Конструктор [QuotedTag]
pub fn quoted_tag(tag: &'static str) -> QuotedTag {
    QuotedTag(Tag { tag })
}
/// Комбинатор, пробрасывающий строку без лидирующих пробелов
#[derive(Debug, Clone)]
pub struct StripWhitespace<T> {
    parser: T,
}
impl<T: Parser> Parser for StripWhitespace<T> {
    type Dest = T::Dest;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        self.parser
            .parse(input.trim_start())
            .map(|(remaining, parsed)| (remaining.trim_start(), parsed))
    }
}
/// Конструктор [StripWhitespace]
pub fn strip_whitespace<T: Parser>(parser: T) -> StripWhitespace<T> {
    StripWhitespace { parser }
}
/// Комбинатор, чтобы распарсить нужное, окружённое в начале и в конце чем-то
/// обязательным, не участвующем в результате.
/// Пробрасывает строку в парсер1, оставшуюся строку после первого
/// парсинга - в парсер2, оставшуюся строку после второго парсинга - в парсер3.
/// Результат парсера2 будет результатом этого комбинатора, а оставшейся
/// строкой - строка, оставшаяся после парсера3.
/// (аналог `delimited` из `nom`)
#[derive(Debug, Clone)]
pub struct Delimited<Prefix, T, Suffix> {
    prefix_to_ignore: Prefix,
    dest_parser: T,
    suffix_to_ignore: Suffix,
}
impl<Prefix, T, Suffix> Parser for Delimited<Prefix, T, Suffix>
where
    Prefix: Parser,
    T: Parser,
    Suffix: Parser,
{
    type Dest = T::Dest;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        let (remaining, _) = self.prefix_to_ignore.parse(input)?;
        let (remaining, result) = self.dest_parser.parse(remaining)?;
        self.suffix_to_ignore
            .parse(remaining)
            .map(|(remaining, _)| (remaining, result))
    }
}
/// Конструктор [Delimited]
pub fn delimited<Prefix, T, Suffix>(
    prefix_to_ignore: Prefix,
    dest_parser: T,
    suffix_to_ignore: Suffix,
) -> Delimited<Prefix, T, Suffix>
where
    Prefix: Parser,
    T: Parser,
    Suffix: Parser,
{
    Delimited {
        prefix_to_ignore,
        dest_parser,
        suffix_to_ignore,
    }
}
/// Комбинатор с отбрасываемым префиксом, упрощённая версия [Delimited]
/// (аналог `preceeded` из `nom`)
#[derive(Debug, Clone)]
pub struct Preceded<Prefix, T> {
    prefix_to_ignore: Prefix,
    dest_parser: T,
}
impl<Prefix, T> Parser for Preceded<Prefix, T>
where
    Prefix: Parser,
    T: Parser,
{
    type Dest = T::Dest;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        let (remaining, _) = self.prefix_to_ignore.parse(input)?;
        self.dest_parser.parse(remaining)
    }
}
/// Конструктор [Preceded]
pub fn preceded<Prefix, T>(prefix_to_ignore: Prefix, dest_parser: T) -> Preceded<Prefix, T>
where
    Prefix: Parser,
    T: Parser,
{
    Preceded {
        prefix_to_ignore,
        dest_parser,
    }
}
/// Комбинатор-отображение. Парсит дочерним парсером, преобразует результат так,
/// как вызывающему хочется
#[derive(Debug, Clone)]
pub struct Map<T, M> {
    parser: T,
    map: M,
}
impl<T: Parser, Dest: Sized, M: Fn(T::Dest) -> Dest> Parser for Map<T, M> {
    type Dest = Dest;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        self.parser
            .parse(input)
            .map(|(remaining, pre_result)| (remaining, (self.map)(pre_result)))
    }
}
/// Конструктор [Map]
pub fn map<T: Parser, Dest: Sized, M: Fn(T::Dest) -> Dest>(parser: T, map: M) -> Map<T, M> {
    Map { parser, map }
}
