/// Парсер, возвращающий результат как есть
#[derive(Debug, Clone)]
struct AsIs;
impl Parser for AsIs {
    type Dest = String;
    fn parse(&self, input: String) -> Result<(String, Self::Dest), ()> {
        Ok((input[input.len()..].to_string(), input.into()))
    }
}

/// Парсер константных строк
/// (аналог `nom::bytes::complete::tag`)
#[derive(Debug, Clone)]
struct Tag {
    tag: &'static str,
}
impl Parser for Tag {
    type Dest = ();
    fn parse(&self, input: String) -> Result<(String, Self::Dest), ()> {
        Ok((input.strip_prefix(self.tag).ok_or(())?.to_string(), ()))
    }
}
/// Конструктор [Tag]
fn tag(tag: &'static str) -> Tag {
    Tag { tag }
}
/// Парсер [тэга](Tag), обёрнутого в кавычки
#[derive(Debug, Clone)]
struct QuotedTag(Tag);
impl Parser for QuotedTag {
    type Dest = ();
    fn parse(&self, input: String) -> Result<(String, Self::Dest), ()> {
        let (remaining, candidate) = do_unquote_non_escaped(input)?;
        if !self.0.parse(candidate)?.0.is_empty() {
            return Err(());
        }
        Ok((remaining, ()))
    }
}
/// Конструктор [QuotedTag]
fn quoted_tag(tag: &'static str) -> QuotedTag {
    QuotedTag(Tag { tag })
}

