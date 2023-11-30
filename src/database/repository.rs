use sqlx::PgPool;

pub struct PostgresRepository {
    pub pool: PgPool,
}

impl PostgresRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
