use sqlx::PgPool;

use crate::models::firmware::FirmwareRelease;

pub struct FirmwareRepository;

impl FirmwareRepository {
    /// Buscar o release mais recente (maior build_number)
    pub async fn get_latest(pool: &PgPool) -> Result<Option<FirmwareRelease>, sqlx::Error> {
        sqlx::query_as!(
            FirmwareRelease,
            r#"
            SELECT id, version, build_number, filename, sha256, release_notes, created_at
            FROM firmware_releases
            ORDER BY build_number DESC
            LIMIT 1
            "#
        )
        .fetch_optional(pool)
        .await
    }

    /// Maior build_number já publicado (para validação de monotonicidade)
    pub async fn get_max_build_number(pool: &PgPool) -> Result<Option<i32>, sqlx::Error> {
        let row = sqlx::query!(
            r#"SELECT MAX(build_number) AS "max_build: i32" FROM firmware_releases"#
        )
        .fetch_one(pool)
        .await?;

        Ok(row.max_build)
    }

    /// Inserir um novo release
    pub async fn insert_release(
        pool: &PgPool,
        version: &str,
        build_number: i32,
        filename: &str,
        sha256: &str,
        release_notes: Option<&str>,
    ) -> Result<FirmwareRelease, sqlx::Error> {
        sqlx::query_as!(
            FirmwareRelease,
            r#"
            INSERT INTO firmware_releases (version, build_number, filename, sha256, release_notes)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, version, build_number, filename, sha256, release_notes, created_at
            "#,
            version,
            build_number,
            filename,
            sha256,
            release_notes
        )
        .fetch_one(pool)
        .await
    }
}
