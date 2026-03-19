// Пусть есть логи:
// System(requestid):
// - trace
// - error
// App(requestid):
// - trace
// - error
// - journal (человекочитаемая сводка)

// Есть прототип штуки, которая умеет:
// - парсить логи
// - фильтровать
//  -- по requestid
//  -- по ошибкам
//  -- по изменению счёта (купить/продать)

// Модель данных:
// - Пользователь (userid, имя)
// - Вещи
//  -- Предмет (assetid, название)
//  -- Набор (assetid, количество)
//      comment{-- Собственность (assetid, userid владельца, количество)}
//  -- Таблица предложения (assetid на assetid, userid продавца)
//  -- Таблица спроса (assetid на assetid, userid покупателя)
// - Операция App
//  -- Journal
//   --- Создать пользователя userid с уставным капиталом от 10usd и выше
//   --- Удалить пользователя
//   --- Зарегистрировать assetid с ликвидностью от 50usd
//   --- Удалить assetid (весь asset должен принадлежать пользователю)
//   --- Внести usd для userid (usd (aka доллар сша) - это тип asset)
//   --- Вывести usd для userid
//   --- Купить asset
//   --- Продать asset
//  -- Trace
//   --- Соединить с биржей
//   --- Получить данные с биржи
//   --- Локальная проверка корректности (упреждение ошибок в ответе)
//   --- Отправить запрос в биржу
//   --- Получить ответ от биржи
//  -- Error
//   --- нет asset
//   --- системная ошибка
// - Операция System
//  -- Trace
//   --- Отправить запрос
//   --- Получить ответ
//  -- Error
//   --- нет сети
//   --- отказано в доступе

use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Placeholder для экспериментов с cli");

    let args = std::env::args().collect::<Vec<_>>();

    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return Err("Not enough arguments".into());
    }

    let filename = &args[1];
    let folder = std::env::current_dir()?;

    println!(
        "Trying opening file '{}' from directory '{}'",
        filename,
        folder.to_string_lossy()
    );

    let file = File::open(filename)?;

    let logs = analysis::read_log(Box::new(file), analysis::ReadMode::All, vec![]);
    let output = logs
        .iter()
        .map(|log| format!("{:?}", log))
        .collect::<Vec<String>>()
        .join("\n");
    println!("Got logs:\n{}", output);
    //logs.iter().for_each(|parsed| println!("  {:?}", parsed));
    Ok(())
}

#[cfg(test)]
mod tests {
    use analysis::parse::types::domain::*;

    use analysis::parse_old::*;

    #[test]
    fn parse_test() {
        let parsing_demo =
            r#"[UserBackets{"user_id":"Bob","backets":[Backet{"asset_id":"milk","count":3,},],},]"#;
        let announcements = just_parse::<Announcements>(parsing_demo).unwrap();
        assert!(
            announcements
                == (
                    "",
                    Announcements::from(vec![UserBackets {
                        user_id: "Bob".to_string(),
                        backets: vec![Backet {
                            asset_id: "milk".to_string(),
                            count: 3,
                        }]
                    }])
                )
        );
    }
}
