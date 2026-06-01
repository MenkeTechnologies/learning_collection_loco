# LearningCollection — Loco port

Rust/Loco port of the Kotlin/Spring Boot `LearningCollectionAPI`.

Original: `~/RustroverProjects/LearningCollectionAPI` (Spring Boot 4.0.4 + Kotlin 2.3.20 + JPA + Spring Data REST + MySQL).
This port: Loco 0.16 + Axum + SeaORM + SQLite.

## Mapping

| Spring | Loco |
|---|---|
| `LearningCollection` `@Entity` | `models/_entities/learning_collections.rs` (SeaORM `DeriveEntityModel`) |
| `LCRepo : CrudRepository` + `@RepositoryRestResource(path="learning")` | `controllers/learning.rs` routes `/learning`, `/learning/{id}` |
| `findAllByLearningContaining(s)` | `Column::Learning.like("%s%")` in `filter_learn` |
| `LearningController` | `controllers/learning.rs` |
| `WebConfig` CORS | `config/development.yaml` → `middlewares.cors` |
| `application.properties` MySQL DSN | `config/development.yaml` → `database.uri` (sqlite) |
| Flyway / `spring.jpa.hibernate.ddl-auto` | `sea-orm-migration` (`migration/`) + `auto_migrate: true` |
| `SpringApplication.run` | `cargo loco start` |

## Routes (1:1 with Spring controller)

| Method | Path | Handler |
|---|---|---|
| GET | `/add?learning=…` | `add` |
| GET | `/filter?learning=…` | `filter_learn` |
| GET | `/recents` | `recents_default` (last 20) |
| GET | `/recents/{count}` | `recents_count` (strings) |
| GET | `/recent/{count}` | `recent_count` (full rows) |
| GET | `/random` | `random_one` (full row) |
| GET | `/randoms` | `randoms_one` (string) |
| GET | `/randoms/{count}` | `randoms_count` (strings) |
| GET | `/random/{count}` | `random_count` (full rows) |
| GET | `/dump` | `dump` (shells out to `mysqldump`) |
| GET | `/learning` | `list_all` (Spring Data REST collection) |
| GET | `/learning/{id}` | `get_one` |
| DELETE | `/learning/{id}` | `delete_one` |
| GET | `/_health`, `/_ping`, `/_readiness` | Loco built-ins |

`Consts.kt` constants live at top of `controllers/learning.rs`:
`SHORT_CNT=20`, `DEFAULT_CAT="programming"`, `DB_NAME="root"`, `dump_cmd()`.

## Run

```bash
cargo loco db migrate
cargo loco start          # http://localhost:5150
cargo loco routes         # list all routes
```

## Smoke test

```bash
curl 'http://localhost:5150/add?learning=axum%20is%20fast'
curl 'http://localhost:5150/add?learning=spring%20has%20gc%20pauses'
curl 'http://localhost:5150/recents'
curl 'http://localhost:5150/filter?learning=spring'
curl 'http://localhost:5150/random'
curl 'http://localhost:5150/learning'
curl -X DELETE 'http://localhost:5150/learning/1'
```

## Notes / intentional drift

- **DB**: SQLite for zero-setup dev. Swap to MySQL/Postgres by changing `database.uri` in `config/development.yaml` and the SeaORM feature in `Cargo.toml`.
- **`/dump`**: still invokes `mysqldump`. Only meaningful when actually pointed at MySQL — kept for behavioral parity. For SQLite use `sqlite3 db .dump`.
- **`@RepositoryRestResource`**: Spring auto-generates HAL pagination, `_links`, PATCH/POST/PUT for free. The port exposes the routes manually (GET list, GET one, DELETE). Add POST/PUT scaffolds with `cargo loco generate scaffold` if you want full CRUD.
- **No Lombok / QueryDSL / springdoc-openapi equivalents**: SeaORM types replace QueryDSL; OpenAPI can be added via the `utoipa` crate if needed.
- **Timestamps**: SeaORM adds `created_at` / `updated_at` automatically; `date_added` is preserved as in the original.

## Project layout

```
learning_collection/
├── src/
│   ├── app.rs                          # Hooks impl, route registration
│   ├── controllers/learning.rs         # Ported controller
│   └── models/_entities/
│       └── learning_collections.rs     # SeaORM entity (generated)
├── migration/
│   └── src/m20260601_*_learning_collections.rs
└── config/{development,test,production}.yaml
```
