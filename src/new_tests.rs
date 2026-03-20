#[cfg(test)]
mod test {

    use crate::parse::log::parser::just_parse;
    use crate::parse::types::auth::AuthData;

    use crate::parse::combinators::basic::*;
    use crate::parse::combinators::choice::*;
    use crate::parse::combinators::list::*;
    use crate::parse::combinators::permutation::*;
    use crate::parse::log::kinds::*;
    use crate::parse::primitives::stdp;
    use crate::parse::traits::{Parsable, Parser};
    use crate::parse::types::domain::*;

    #[test]
    fn test_alt_n() {
        let alt_n = alternative_from_n(vec![Box::new(AsIs)]);

        assert_eq!(alt_n.parse("hello"), Ok(("", "hello".into())));

        fn to_ok(_: ()) -> Status {
            Status::Ok
        }

        let hello_or_bye = alternative_from_n(vec![
            Box::new(map(tag("Hello"), to_ok)),
            Box::new(map(tag("Bye"), to_ok)),
        ]);

        assert_eq!(
            hello_or_bye.parse("Hello World!"),
            Ok((" World!", Status::Ok))
        );
        assert_eq!(
            hello_or_bye.parse("Bye World!"),
            Ok((" World!", Status::Ok))
        );
        assert_eq!(hello_or_bye.parse("Hllo World!"), Err(()));
        assert_eq!(hello_or_bye.parse("Byye World!"), Err(()));
    }
}
