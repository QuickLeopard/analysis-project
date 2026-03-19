use crate::parse::traits::Parser;

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

/// Комбинатор списка из любого числа элементов, которые надо читать
/// вложенным парсером. Граница списка определяется квадратными (`[`&`]`)
/// скобками.
/// Для простоты реализации, после каждого элемента списка должна быть запятая
#[derive(Debug, Clone)]
pub struct List<T> {
    parser: T,
}
impl<T: Parser> Parser for List<T> {
    type Dest = Vec<T::Dest>;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        let mut remaining = input.trim_start().strip_prefix('[').ok_or(())?.trim_start();
        let mut result = Vec::new();
        while !remaining.is_empty() {
            match remaining.strip_prefix(']') {
                Some(remaining) => return Ok((remaining.trim_start(), result)),
                None => {
                    let (new_remaining, item) = self.parser.parse(remaining)?;
                    let new_remaining = new_remaining
                        .trim_start()
                        .strip_prefix(',')
                        .ok_or(())?
                        .trim_start();
                    result.push(item);
                    remaining = new_remaining;
                }
            }
        }
        Err(()) // строка кончилась, не закрыв скобку
    }
}
/// Конструктор для [List]
pub fn list<T: Parser>(parser: T) -> List<T> {
    List { parser }
}
