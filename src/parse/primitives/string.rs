pub fn just_parse<T: Parsable>(input: String) -> Result<(String, T), ()> {
    T::parser().parse(input)
}
