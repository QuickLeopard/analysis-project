# analysis-project

Небольшая библиотека и CLI для парсинга и фильтрации логов.

## Что это за проект

- Библиотека предоставляет набор parser combinators и доменные парсеры логов.
- CLI (`log-analysis-cli`) читает файл, парсит строки и выводит распознанные записи.
- Основной публичный API: `read_log(input, mode, request_ids)`.

## Быстрый старт

### Запуск CLI

```bash
cargo run --bin log-analysis-cli -- <path-to-log-file>
```

### Запуск тестов

```bash
cargo test
```

## Эволюция проекта (кратко)

### 1) Базовая структура
- `Initial commit`
- Создан каркас библиотеки и CLI.

**Почему:** нужен минимальный рабочий baseline для итеративного рефакторинга.

### 2) Модульная переработка парсера
- `refactor: split parse module into modular structure`
- `refactor: restructure parser library into modular architecture`
- `refactor: complete parser library modularization`

**Почему:** упростить навигацию по коду и разделить ответственность по модулям.

### 3) Улучшение API и типобезопасности
- `refactor: change Parser trait to use &str instead of String`
- `refactor: modernize API with generics and improve code organization`
- `refactor: replace mode constants with enum, improve thread safety`

**Почему:** уменьшить лишние аллокации, сделать API более идиоматичным и удобным.

### 4) Развитие комбинаторов и устойчивости парсинга
- `feat: add alternative_n function and Alt<Vec<A>> Parser implementation`
- `fix: correct WithdrawCash mapping and refactor parser organization`
- `fix: remove incorrect zero byte validation in Byte parser`
- `fix: apply mode filter correctly and strengthen parser/tests documentation`

**Почему:** расширить выразительность грамматики и убрать ошибки в критичных ветках парсинга.

### 5) Улучшение CLI и DX
- `refactor: improve CLI code quality and test style`
- `refactor(cli): adopt anyhow-based error handling in main`
- удалён singleton-подход для парсера строки лога
- удалена публичная helper-функция `parse_log_line`, разбор теперь идёт через типизированный парсер `LogLine::parser()`

**Почему:** сделать диагностику ошибок CLI понятнее и упростить сопровождение кода.

### 6) Инфраструктура
- `ci: add GitHub Actions workflow for CI`

**Почему:** автоматизировать проверку изменений и снизить риск регрессий.

## Текущее состояние

Проект прошёл путь от учебного прототипа к структурированной Rust-кодовой базе:
- модульная архитектура parser combinators;
- улучшенная типобезопасность;
- исправления важных edge-cases в логике разбора;
- более качественная обработка ошибок в CLI;
- CI-пайплайн для базовой проверки качества.