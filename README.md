# analysis-project

Небольшая библиотека и CLI для парсинга и фильтрации логов.

## Краткая история изменений

### 1. Старт проекта
- `Initial commit`
- Была создана базовая структура библиотеки и CLI.

**Почему:** заложить минимальный рабочий каркас для дальнейшего развития.

### 2. Модульная архитектура парсера
- `refactor: split parse module into modular structure`
- `refactor: restructure parser library into modular architecture`
- `refactor: complete parser library modularization`

**Почему:** улучшить читаемость, изоляцию ответственности и поддержку кода.

### 3. Улучшение API и базовых абстракций
- `refactor: change Parser trait to use &str instead of String`
- `refactor: modernize API with generics and improve code organization`
- `refactor: replace mode constants with enum, improve thread safety`

**Почему:** сделать API более идиоматичным для Rust, уменьшить лишние аллокации и повысить типобезопасность.

### 4. Развитие комбинаторов
- `feat: add alternative_n function and Alt<Vec<A>> Parser implementation`
- `refactor: simplify parser singleton with LazyLock`

**Почему:** расширить выразительность парсеров и упростить/ускорить повторное использование парсеров.

### 5. Исправления корректности
- `fix: correct WithdrawCash mapping and refactor parser organization`
- `fix: remove incorrect zero byte validation in Byte parser`
- `fix: apply mode filter correctly and strengthen parser/tests documentation`

**Почему:** устранить ошибки парсинга и фильтрации, повысить предсказуемость поведения.

### 6. Улучшение CLI и DX
- `refactor: improve CLI code quality and test style`
- `refactor(cli): adopt anyhow-based error handling in main`

**Почему:** сделать обработку ошибок в CLI более чистой и информативной (`anyhow`, контекст ошибок).

### 7. Инфраструктура
- `ci: add GitHub Actions workflow for CI`

**Почему:** автоматическая проверка качества кода в PR/ветке.

## Текущее состояние

Проект перешёл от базового прототипа к более структурированной и идиоматичной Rust-архитектуре:
- модульный парсер;
- улучшенная типобезопасность;
- исправленные критичные edge-cases;
- более качественная диагностика ошибок в CLI;
- CI для стабильности изменений.