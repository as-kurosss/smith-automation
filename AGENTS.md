# Project Rules
### Запросы к графу

Для анализа архитектуры или поиска связей используй:
```bash
graphify-rs query "<вопрос>"
```
Убедись, что граф сгенерирован перед выполнением запроса.

### Установка

Если `graphify-rs` не установлен:
```bash
cargo install graphify-rs
```
## graphify-rs

Используй утилиту `graphify-rs` для построения графа знаний проекта.

### Генерация графа

Команда:
```bash
graphify-rs build --no-llm --output ./smith-graphify
```

### Артефакты

- `smith-graphify/graph.json` — граф в формате JSON (узлы, рёбра, сообщества)
- `smith-graphify/GRAPH_REPORT.md` — аналитический отчёт по графу

### Интеграция с smith-context

Утилита `smith-context` автоматически загружает артефакты graphify-rs при флаге `--graphify`:
```bash
cargo run -p smith-context -- --graphify
```

### Запросы к графу

Задавай вопросы по архитектуре проекта через:
```bash
graphify-rs query --graph ./smith-graphify/graph.json "<вопрос>"
```

Примеры:
```bash
graphify-rs query --graph ./smith-graphify/graph.json "how does auth work?"
graphify-rs query --graph ./smith-graphify/graph.json "какие модули зависят от smith-core?"
graphify-rs query --graph ./smith-graphify/graph.json "что делает ExecutionContext?"
```

### Установка

Если `graphify-rs` не установлен:
```bash
cargo install graphify-rs
```
