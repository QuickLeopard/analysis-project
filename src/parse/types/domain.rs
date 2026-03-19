/// Пара 'сокращённое название предмета' - 'его описание'
#[derive(Debug, Clone, PartialEq)]
pub struct AssetDsc {
    // `dsc` aka `description`
    pub id: String,
    pub dsc: String,
}

/// Сведение о предмете в некотором количестве
#[derive(Debug, Clone, PartialEq)]
pub struct Backet {
    pub asset_id: String,
    pub count: u32,
}

/// Фиатные деньги конкретного пользователя
#[derive(Debug, Clone, PartialEq)]
pub struct UserCash {
    pub user_id: String,
    pub count: u32,
}

/// [Backet] конкретного пользователя
#[derive(Debug, Clone, PartialEq)]
pub struct UserBacket {
    pub user_id: String,
    pub backet: Backet,
}

/// [Бакеты](Backet) конкретного пользователя
#[derive(Debug, Clone, PartialEq)]
pub struct UserBackets {
    pub user_id: String,
    pub backets: Vec<Backet>,
}

/// Список опубликованных бакетов
#[derive(Debug, Clone, PartialEq)]
pub struct Announcements(Vec<UserBackets>);
impl Parsable for Announcements {
    type Parser = Map<List<<UserBackets as Parsable>::Parser>, fn(Vec<UserBackets>) -> Self>;
    fn parser() -> Self::Parser {
        fn from_vec(vec: Vec<UserBackets>) -> Announcements {
            Announcements(vec)
        }
        map(list(UserBackets::parser()), from_vec)
    }
}