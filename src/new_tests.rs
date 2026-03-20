#[cfg(test)]
mod test {
    use crate::parse::combinators::basic::{AsIs, map, quoted_tag, tag, unquote};
    use crate::parse::combinators::choice::{alternative_from_n, key_value};
    use crate::parse::combinators::permutation::{permutation2, permutation3};
    use crate::parse::log::kinds::{AppLogKind, LogKind, SystemLogErrorKind, SystemLogKind};
    use crate::parse::log::parser::{just_parse, parse_log_line};
    use crate::parse::primitives::stdp;
    use crate::parse::traits::Parser;
    use crate::parse::types::domain::{Announcements, Status};
    use crate::{ReadMode, read_log};

    const LOGS_FOR_FILTERS: &str = r#"
System::Error NetworkError "network is down" requestid=10
System::Trace SendRequest "heartbeat" requestid=10
App::Error SystemError "db unavailable" requestid=11
App::Journal DepositCash UserCash{"user_id":"Bob","count":10,} requestid=11
"#;

    fn to_ok(_: ()) -> Status {
        Status::Ok
    }

    /// Проверяет, что динамический альтернативный парсер берёт первый успешный вариант.
    #[test]
    fn test_alt_n() {
        let alt_n = alternative_from_n(vec![Box::new(AsIs)]);

        assert_eq!(alt_n.parse("hello"), Ok(("", "hello".into())));

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

    /// Проверяет, что вектор альтернатив поддерживает разные типы комбинаторов.
    #[test]
    fn test_alt_n_with_different_combinators_in_vector() {
        let mixed_alternatives = alternative_from_n(vec![
            Box::new(map(tag("Hello"), to_ok)),
            Box::new(map(quoted_tag("Bye"), to_ok)),
        ]);

        assert_eq!(
            mixed_alternatives.parse("Hello there"),
            Ok((" there", Status::Ok))
        );
        assert_eq!(mixed_alternatives.parse(r#""Bye""#), Ok(("", Status::Ok)));
        assert_eq!(mixed_alternatives.parse("Bye"), Err(()));
    }

    /// Проверяет, что пустой набор альтернатив возвращает ошибку, а не паникует.
    #[test]
    fn test_alt_n_empty_collection() {
        let empty_alt = alternative_from_n::<Status>(Vec::new());
        assert_eq!(empty_alt.parse("Ok"), Err(()));
    }

    /// Проверяет, что фильтр по режиму работает даже при пустом списке request_id.
    #[test]
    fn test_read_log_applies_mode_when_request_ids_are_empty() {
        let only_errors = read_log(LOGS_FOR_FILTERS.as_bytes(), ReadMode::Errors, vec![]);
        assert_eq!(only_errors.len(), 2);
        assert!(matches!(
            only_errors[0].kind,
            LogKind::System(SystemLogKind::Error(SystemLogErrorKind::NetworkError(_)))
        ));
        assert!(matches!(
            only_errors[1].kind,
            LogKind::App(AppLogKind::Error(_))
        ));
    }

    /// Проверяет, что фильтры по request_id и режиму объединяются через И (AND).
    #[test]
    fn test_read_log_combines_request_id_and_mode_filters() {
        let request_10_errors = read_log(LOGS_FOR_FILTERS.as_bytes(), ReadMode::Errors, vec![10]);
        assert_eq!(request_10_errors.len(), 1);
        assert!(matches!(
            request_10_errors[0].kind,
            LogKind::System(SystemLogKind::Error(SystemLogErrorKind::NetworkError(_)))
        ));
    }

    /// Проверяет, что строка лога без request_id отклоняется парсером строки.
    #[test]
    fn test_log_line_parser_rejects_line_without_request_id() {
        let line = r#"System::Error NetworkError "network is down""#;
        assert_eq!(parse_log_line(line), Err(()));
    }

    /// Проверяет обобщённый helper-парсер на вложенной доменной структуре.
    #[test]
    fn test_just_parse_announcements_happy_path() {
        let payload =
            r#"[UserBackets{"user_id":"Bob","backets":[Backet{"asset_id":"milk","count":3,},],},]"#;
        let parsed = just_parse::<Announcements>(payload).unwrap();
        assert_eq!(parsed.0, "");
    }

    /// Проверяет permutation2 напрямую: порядок ключей может быть любым.
    #[test]
    fn test_permutation2_direct_order_independent() {
        let parser = permutation2(
            key_value("user_id", unquote()),
            key_value("count", stdp::U32),
        );

        assert_eq!(
            parser.parse(r#""user_id":"Bob","count":10,tail"#),
            Ok(("tail", ("Bob".to_string(), 10)))
        );
        assert_eq!(
            parser.parse(r#""count":10,"user_id":"Bob",tail"#),
            Ok(("tail", ("Bob".to_string(), 10)))
        );
        assert_eq!(parser.parse(r#""user_id":"Bob",tail"#), Err(()));
    }

    /// Проверяет permutation3 напрямую: корректно собирает результат при разном порядке полей.
    #[test]
    fn test_permutation3_direct_order_independent() {
        let parser = permutation3(
            key_value("asset_id", unquote()),
            key_value("user_id", unquote()),
            key_value("liquidity", stdp::U32),
        );

        assert_eq!(
            parser.parse(r#""asset_id":"bayc","user_id":"Alice","liquidity":1000,tail"#),
            Ok(("tail", ("bayc".to_string(), "Alice".to_string(), 1000)))
        );
        assert_eq!(
            parser.parse(r#""liquidity":1000,"asset_id":"bayc","user_id":"Alice",tail"#),
            Ok(("tail", ("bayc".to_string(), "Alice".to_string(), 1000)))
        );
        assert_eq!(
            parser.parse(r#""asset_id":"bayc","liquidity":1000,tail"#),
            Err(())
        );
    }
}
