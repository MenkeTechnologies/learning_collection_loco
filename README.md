```
 ██╗      ██████╗  ██████╗ ██████╗
 ██║     ██╔═══██╗██╔════╝██╔═══██╗
 ██║     ██║   ██║██║     ██║   ██║
 ██║     ██║   ██║██║     ██║   ██║
 ███████╗╚██████╔╝╚██████╗╚██████╔╝
 ╚══════╝ ╚═════╝  ╚═════╝ ╚═════╝
 ▙ LEARNING_COLLECTION ▟ KOTLIN/SPRING → RUST/LOCO ▙ 488 LINES ▟
```

[![CI](https://github.com/MenkeTechnologies/learning_collection_loco/actions/workflows/ci.yaml/badge.svg)](https://github.com/MenkeTechnologies/learning_collection_loco/actions/workflows/ci.yaml)
[![Docs](https://img.shields.io/badge/docs-online-blue.svg)](https://menketechnologies.github.io/learning_collection_loco/)
[![Report](https://img.shields.io/badge/engineering-report-ff2a6d.svg)](https://menketechnologies.github.io/learning_collection_loco/report.html)
[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org)
[![Loco](https://img.shields.io/badge/loco-0.16-05d9e8.svg)](https://loco.rs)
[![Axum](https://img.shields.io/badge/axum-0.8-d300c5.svg)](https://github.com/tokio-rs/axum)
[![SeaORM](https://img.shields.io/badge/sea--orm-1.1-39ff14.svg)](https://www.sea-ql.org/SeaORM/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

### `[1:1 RUST PORT OF SPRING BOOT 4.0.4 + KOTLIN 2.3.20]`

> *"No JVM. No GC pauses. No Hibernate session-per-request. No 2-second startup banner."*

A complete Rust replacement for the Kotlin/Spring Boot [`LearningCollectionAPI`](https://github.com/MenkeTechnologies/LearningCollectionAPI). Same routes, same shapes, native binary. 488 lines of Rust across 25 files, sub-second cold boot, SQLite by default with a one-flag swap to MySQL / Postgres.

### [`Read the Docs`](https://menketechnologies.github.io/learning_collection_loco/) &middot; [`Engineering Report`](https://menketechnologies.github.io/learning_collection_loco/report.html)

---

## Table of Contents

- [\[0x00\] Overview](#0x00-overview)
- [\[0x01\] Install &amp; Run](#0x01-install--run)
- [\[0x02\] Routes](#0x02-routes)
- [\[0x03\] Spring &rarr; Loco Mapping](#0x03-spring--loco-mapping)
- [\[0x04\] Smoke Test](#0x04-smoke-test)
- [\[0x05\] Layout](#0x05-layout)
- [\[0x06\] Intentional Drift](#0x06-intentional-drift)
- [\[0x07\] Stats](#0x07-stats)
- [\[0xFF\] License](#0xff-license)

---

## [0x00] OVERVIEW

`learning_collection_loco` is a port of a small but complete Spring Boot CRUD service onto the [Loco](https://loco.rs) framework. The original was Spring Boot 4.0.4 + Kotlin 2.3.20 + JPA + Spring Data REST against MySQL with one entity (`LearningCollection`) and a controller exposing thirteen verbs (`add`, `filter`, `recents`, `recent`, `random`, `randoms`, `dump`, plus Spring Data REST's auto-generated `/learning` collection / item / delete).

Stack:

- **Loco 0.16** &mdash; `Hooks` lifecycle, route assembly, built-in `/_health` / `/_ping` / `/_readiness`
- **Axum 0.8** &mdash; HTTP handlers, extractors, response building
- **SeaORM 1.1** &mdash; async ORM with `sqlx-sqlite` + `sqlx-postgres`
- **sea-orm-migration** &mdash; replaces Flyway / `spring.jpa.hibernate.ddl-auto`
- **Tokio 1.45** multi-thread runtime
- **`chrono` / `serde` / `serde_json` / `tracing` / `rand` / `validator` / `uuid` / `regex`** &mdash; stock conveniences

Trade-offs versus the original:

| Disappears with the JVM | Gained with Loco/Rust |
|---|---|
| Lombok boilerplate | `#[derive(...)]` |
| QueryDSL Q-types | SeaORM `Column` + type-safe query builder |
| Spring Data REST auto-HAL pagination | Explicit handlers (PATCH/POST/PUT add via `cargo loco generate scaffold`) |
| Spring `WebConfig` CORS bean | `middlewares.cors` YAML stanza |
| Flyway + `ddl-auto` | `sea-orm-migration` with explicit up/down |
| JIT warmup, GC pauses, 2-sec startup banner | Sub-second cold boot, no GC |
| `application.properties` MySQL DSN | `config/{dev,test,prod}.yaml` `database.uri` |

---

## [0x01] INSTALL &amp; RUN

```sh
git clone https://github.com/MenkeTechnologies/learning_collection_loco
cd learning_collection_loco

cargo loco db migrate       # apply migrations
cargo loco start            # http://localhost:5150
cargo loco routes           # list every registered route
```

Swap database: edit `database.uri` in `config/development.yaml` and flip the SeaORM driver feature in `Cargo.toml` (`sqlx-sqlite` → `sqlx-mysql` / `sqlx-postgres`).

---

## [0x02] ROUTES

Thirteen routes, registered in `src/controllers/learning.rs::routes()` &mdash; 1:1 with the original Spring controller.

| Method | Path | Handler | Returns | Spring origin |
|---|---|---|---|---|
| `GET` | `/add?learning=…` | `add` | JSON row | `LearningController.add` |
| `GET` | `/filter?learning=…` | `filter_learn` | JSON `[String]` | `findAllByLearningContaining` |
| `GET` | `/recents` | `recents_default` | JSON `[String]` | last `SHORT_CNT=20` |
| `GET` | `/recents/{count}` | `recents_count` | JSON `[String]` | strings only |
| `GET` | `/recent/{count}` | `recent_count` | JSON `[Model]` | full rows |
| `GET` | `/random` | `random_one` | JSON `Model` | full row |
| `GET` | `/randoms` | `randoms_one` | JSON `String` | string only |
| `GET` | `/randoms/{count}` | `randoms_count` | JSON `[String]` | shuffle + take |
| `GET` | `/random/{count}` | `random_count` | JSON `[Model]` | shuffle + take |
| `GET` | `/dump` | `dump` | `text/plain` | shells out to `mysqldump` |
| `GET` | `/learning` | `list_all` | JSON `[Model]` | Spring Data REST collection |
| `GET` | `/learning/{id}` | `get_one` | JSON `Model` | Spring Data REST item |
| `DELETE` | `/learning/{id}` | `delete_one` | 204 empty | Spring Data REST DELETE |

Plus Loco built-ins for free: `/_health`, `/_ping`, `/_readiness`.

Constants from `Consts.kt` live at the top of `controllers/learning.rs`:

```rust
pub const SHORT_CNT: usize = 20;
pub const DEFAULT_CAT: &str = "programming";
pub const DB_NAME: &str = "root";

fn dump_cmd() -> String {
    format!("mysqldump --extended-insert=FALSE {DB_NAME}")
}
```

---

## [0x03] SPRING &rarr; LOCO MAPPING

| Spring (Kotlin) | Loco (Rust) |
|---|---|
| `@Entity LearningCollection` | `models/_entities/learning_collections.rs` (SeaORM `DeriveEntityModel`) |
| `LCRepo : CrudRepository<…>` + `@RepositoryRestResource(path="learning")` | `controllers/learning.rs` routes for `/learning`, `/learning/{id}` |
| `findAllByLearningContaining(s)` | `Column::Learning.like("%s%")` inside `filter_learn` |
| `LearningController` | `src/controllers/learning.rs` |
| `WebConfig` CORS | `config/development.yaml` → `middlewares.cors` |
| `application.properties` MySQL DSN | `config/development.yaml` → `database.uri` (sqlite) |
| Flyway / `spring.jpa.hibernate.ddl-auto` | `sea-orm-migration` under `migration/` + `auto_migrate: true` |
| `SpringApplication.run` / `@SpringBootApplication` | `cargo loco start` &rarr; `Hooks::boot` &rarr; `create_app::<App, Migrator>` |
| `Consts.kt` | `pub const`s at top of `controllers/learning.rs` |
| Lombok `@Data` / `@Builder` | `#[derive(Serialize, Deserialize, Clone, Debug, …)]` |
| QueryDSL Q-types | SeaORM `Column` enum + query builder |
| springdoc-openapi | `utoipa` crate (opt-in, not currently wired) |

---

## [0x04] SMOKE TEST

```sh
curl 'http://localhost:5150/add?learning=axum%20is%20fast'
curl 'http://localhost:5150/add?learning=spring%20has%20gc%20pauses'
curl 'http://localhost:5150/recents'                       # last 20 learnings, strings only
curl 'http://localhost:5150/filter?learning=spring'        # substring filter via LIKE %pat%
curl 'http://localhost:5150/random'                        # full row picked uniformly
curl 'http://localhost:5150/learning'                      # Spring Data REST-style collection
curl -X DELETE 'http://localhost:5150/learning/1'          # Spring Data REST-style item delete
curl 'http://localhost:5150/_health'                       # Loco built-in healthcheck
```

---

## [0x05] LAYOUT

```
learning_collection_loco/
├── src/
│   ├── app.rs                                    # 66 ln · Hooks impl, route registration
│   ├── bin/main.rs                               # 8  ln · binary entrypoint (delegates to loco_rs::cli)
│   ├── controllers/
│   │   ├── home.rs                               # 12 ln · placeholder home
│   │   └── learning.rs                           # 202 ln · the port (13 handlers + helper + routes)
│   ├── models/
│   │   ├── _entities/learning_collections.rs     # 19 ln · generated SeaORM entity
│   │   └── learning_collections.rs               # 28 ln · user-extendable wrapper
│   ├── views/home.rs                             # 16 ln · placeholder view
│   └── lib.rs                                    # 7  ln · crate root
├── migration/
│   ├── src/lib.rs                                # 16 ln · MigratorTrait impl
│   └── src/m20260601_044042_learning_collections.rs   # 27 ln · up/down schema
├── tests/
│   ├── models/learning_collections.rs            # 31 ln · insta-snapshot scaffold
│   └── requests/{home,learning}.rs               # 31 ln · integration test scaffolds
└── config/{development,test,production}.yaml     # server, DB, CORS, logger, auto_migrate
```

---

## [0x06] INTENTIONAL DRIFT

- **DB** &mdash; SQLite for zero-setup dev. The original Spring app talked to MySQL (`root`). Driver swap is a YAML edit plus a `Cargo.toml` feature toggle.
- **`/dump`** &mdash; still invokes `mysqldump`. Behaviorally identical to the original but only meaningful when the DB is actually MySQL. For SQLite use `sqlite3 db .dump`.
- **`@RepositoryRestResource`** &mdash; Spring auto-generates HAL pagination, `_links`, PATCH/POST/PUT for free. The port exposes GET-list, GET-one, DELETE explicitly. Add POST/PUT scaffolds with `cargo loco generate scaffold` when needed.
- **No Lombok / QueryDSL / springdoc-openapi equivalents wired in** &mdash; SeaORM types replace QueryDSL; OpenAPI can be added via the [`utoipa`](https://docs.rs/utoipa) crate.
- **Timestamps** &mdash; SeaORM adds `created_at` / `updated_at` automatically; `date_added` is preserved as in the original.

---

## [0x07] STATS

| Metric | Value |
|---|---|
| Total Rust lines | **488** |
| Source files (`.rs`) | **25** |
| HTTP routes | **13** |
| Handler fns | **13** |
| Total fns | **33** |
| Entities | **1** (`LearningCollection`) |
| Migrations | **1** (`m20260601_044042_learning_collections`) |
| Test modules | **3** |
| Direct dependencies | **14** (10 runtime + 4 dev) |
| Transitive packages (`Cargo.lock`) | **559** |
| Loco version | **0.16** |
| Axum version | **0.8** |
| SeaORM version | **1.1** |
| Tokio version | **1.45** |
| Rust edition | **2021** |

Numbers measured directly from `src/`, `tests/`, `migration/`, and `Cargo.lock`. Full breakdown in the [engineering report](https://menketechnologies.github.io/learning_collection_loco/report.html).

---

## [0xFF] LICENSE

MIT. See [`LICENSE`](LICENSE) if present, otherwise the standard MIT terms apply.
