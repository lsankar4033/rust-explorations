# Polymarket Indexer

## Purpose

Indexes Polymarket markets and trades from the Polygon blockchain, enriched with metadata from Polymarket's Gamma API. Stores in PostgreSQL for analysis.

**Current:** Market creation events with full metadata and tags
**Coming soon:** Trade events

## Architecture

```
Polygon Blockchain → Parse Events → Enrich with Gamma API → PostgreSQL
                                                                ↓
                                                           market_tags (join)
                                                                ↓
                                                              tags
```

## Status

### ✅ Complete

- `TokenRegistered` event parsing (CTFExchange contract)
- Gamma API client (market metadata + tags)
- PostgreSQL schema with normalized tags
- Market backfill job with deduplication and tag integration

### 🔄 Next

- Trade event indexing

## Database Schema

```sql
markets                           tags
├── condition_id (PK)            ├── pm_tag_id (PK)
├── pm_market_id                 ├── label
├── token0, token1               └── slug
├── question, slug                    ↑
├── outcomes (JSONB)                  │
└── timestamps              market_tags (join)
                            ├── condition_id (FK)
                            └── pm_tag_id (FK)
```

**Key points:**

- CTFExchange contract: `0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E`
- Each `TokenRegistered` event emits twice per market (tokens swapped) - deduplicate by `condition_id`
- Tags are normalized for extensibility

## Usage

### market_backfill

Backfills historical markets with flexible time ranges:

```bash
cargo run --bin market_backfill -- --hours 12
cargo run --bin market_backfill -- --days 7
cargo run --bin market_backfill -- --from-block X --to-block Y
```

Idempotent - skips existing markets, handles missing metadata gracefully.

## Development Notes

**Schema changes:**

1. Write migration in `migrations/`
2. Run locally: `DATABASE_URL="..." sqlx migrate run`
3. Update Rust code
4. Generate cache: `cargo sqlx prepare`
5. Run on prod: `DATABASE_URL="..." sqlx migrate run`
6. Commit migrations + `.sqlx` directory

**Editor setup:** Add `DATABASE_URL` to editor settings for live sqlx validation

**Gamma API:** https://docs.polymarket.com/
