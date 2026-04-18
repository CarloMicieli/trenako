use sqlx::PgPool;

pub async fn seed_manufacturers(pool: &PgPool) {
    sqlx::raw_sql(include_str!("../fixtures/manufacturers.sql"))
        .execute(pool)
        .await
        .expect("Failed to seed manufacturers");
}

pub async fn seed_catalog_items(pool: &PgPool) {
    sqlx::raw_sql(include_str!("../fixtures/catalog_items.sql"))
        .execute(pool)
        .await
        .expect("Failed to seed catalog items");
}

pub async fn seed_railways(pool: &PgPool) {
    sqlx::raw_sql(include_str!("../fixtures/railways.sql"))
        .execute(pool)
        .await
        .expect("Failed to seed railways");
}

pub async fn seed_scales(pool: &PgPool) {
    sqlx::raw_sql(include_str!("../fixtures/scales.sql"))
        .execute(pool)
        .await
        .expect("Failed to seed scales");
}
