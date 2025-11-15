// Market tags database operations

use crate::db::models::Tag as DbTag;
use crate::polymarket::market::Tag as ApiTag;
use eyre::Result;
use sqlx::PgPool;

/// Insert tags for a market
///
/// This is a two-step process:
/// 1. Upsert tags into the tags table (idempotent)
/// 2. Insert relationships into market_tags join table (idempotent)
pub async fn insert_market_tags(pool: &PgPool, condition_id: &str, tags: &[ApiTag]) -> Result<()> {
    for tag in tags {
        // Step 1: Upsert into tags table
        sqlx::query!(
            r#"
            INSERT INTO tags (pm_tag_id, label, slug)
            VALUES ($1, $2, $3)
            ON CONFLICT (pm_tag_id) DO UPDATE SET
                label = COALESCE(EXCLUDED.label, tags.label),
                slug = COALESCE(EXCLUDED.slug, tags.slug)
            "#,
            tag.id,
            tag.label.as_deref(),
            tag.slug.as_deref()
        )
        .execute(pool)
        .await?;

        // Step 2: Insert into market_tags join table
        sqlx::query!(
            r#"
            INSERT INTO market_tags (condition_id, pm_tag_id)
            VALUES ($1, $2)
            ON CONFLICT (condition_id, pm_tag_id) DO NOTHING
            "#,
            condition_id,
            tag.id
        )
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Get all tags for a market (with tag metadata via JOIN)
pub async fn get_tags_for_market(pool: &PgPool, condition_id: &str) -> Result<Vec<DbTag>> {
    let tags = sqlx::query_as!(
        DbTag,
        r#"
        SELECT t.pm_tag_id, t.label, t.slug
        FROM market_tags mt
        JOIN tags t ON mt.pm_tag_id = t.pm_tag_id
        WHERE mt.condition_id = $1
        ORDER BY t.label
        "#,
        condition_id
    )
    .fetch_all(pool)
    .await?;

    Ok(tags)
}
