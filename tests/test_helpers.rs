#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    use diesel::r2d2::Pool;
    use diesel::sqlite::SqliteConnection;
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    use serde_json::json;
    use rust_demo::handlers;
}