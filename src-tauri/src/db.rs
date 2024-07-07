use crate::models::{GeoCode, GeorgError};
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Pool, Sqlite};

#[async_trait::async_trait]
pub(crate) trait GeoCodeRepository: Send + Sync + 'static {
    async fn insert_geo_location(&self, id: &str, geo_code: &GeoCode) -> Result<(), GeorgError>;
    async fn get_geo_location(&self, id: &str) -> Result<Option<GeoCode>, GeorgError>;
}

pub struct SqliteGeoCodeRepository {
    pool: Pool<Sqlite>,
}

#[cfg(test)]
pub struct MockGeoCodeRepository;

#[cfg(test)]
impl MockGeoCodeRepository {
    pub fn new() -> Self {
        Self
    }
}

impl SqliteGeoCodeRepository {
    pub fn new(db_pool: Pool<Sqlite>) -> Self {
        Self { pool: db_pool }
    }
}

pub async fn init_database() -> Result<Pool<Sqlite>, GeorgError> {
    const DB_URL: &str = "sqlite:georg.db";

    Sqlite::create_database(DB_URL).await.or_else(|err| {
        if let sqlx::Error::Database(db_err) = &err {
            if db_err.code() == Some(std::borrow::Cow::Borrowed("SQLITE_CANTOPEN")) {
                return Ok(());
            }
        }
        Err(GeorgError::Unknown)
    })?;

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(DB_URL)
        .await
        .map_err(|_| GeorgError::Unknown)?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|_| GeorgError::Unknown)?;

    Ok(pool)
}

#[async_trait::async_trait]
impl GeoCodeRepository for SqliteGeoCodeRepository {
    async fn insert_geo_location(&self, id: &str, geo_code: &GeoCode) -> Result<(), GeorgError> {
        sqlx::query(
            r#"
            INSERT INTO geo_locations (id, lat, lon)
            VALUES (?, ?, ?)
            "#,
        )
        .bind(id)
        .bind(geo_code.lat)
        .bind(geo_code.lon)
        .execute(&self.pool)
        .await
        .map_err(|_err| {
            GeorgError::Unknown})?;

        Ok(())
    }

    async fn get_geo_location(&self, id: &str) -> Result<Option<GeoCode>, GeorgError> {
        let result: Option<(f64, f64)> = sqlx::query_as(
            r#"
            SELECT lat, lon
            FROM geo_locations
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|_err| {
        
            GeorgError::Unknown})?;

        Ok(result.map(|(lat, lon)| GeoCode::new(lat, lon)))
    }
}

#[async_trait::async_trait]
#[cfg(test)]
impl GeoCodeRepository for MockGeoCodeRepository {
    async fn insert_geo_location(&self, _id: &str, _geo_code: &GeoCode) -> Result<(), GeorgError> {
        Ok(())
    }

    async fn get_geo_location(&self, id: &str) -> Result<Option<GeoCode>, GeorgError> {

        match id {
            "1" => Ok(Some(GeoCode::new(1.0, 1.0))),
            "2" => Ok(Some(GeoCode::new(2.0, 2.0))),
            "3" => Ok(Some(GeoCode::new(3.0, 3.0))),
            "4" => Ok(Some(GeoCode::new(4.0, 4.0))),
            _ => Ok(None),
        }
    }
}