# Polymarket Indexer - System Overview

## Purpose

A Rust-based indexer that captures Polymarket market creation events from Polygon blockchain and enriches them with metadata from Polymarket's Gamma API. Stores data in PostgreSQL for analysis.

Coming soon: indexing trades as well.

## Architecture

```
Polygon Blockchain → Parse Events → Enrich with Gamma API → PostgreSQL
                                                                ↓
                                                           market_tags (join)
                                                                ↓
                                                              tags
```

## Current System Status

### ✅ Complete

- Event parsing (`TokenRegistered` from CTFExchange contract)
- Gamma API client (market metadata + tags)
- PostgreSQL schema with normalized tags
- Market backfill job with deduplication
- Local dev database setup

### 🔄 In Progress

- Adding `pm_market_id` to markets table
- Integrating tags into backfill pipeline

## Data Flow

### 1. Blockchain Events

**Contract:** `0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E` (CTFExchange on Polygon)
**Event:** `TokenRegistered(token0, token1, conditionId)`

Each event = one binary prediction market (YES/NO outcomes)

### 2. Gamma API Enrichment

**Endpoints:**

- `/markets?condition_ids={conditionId}` - Market metadata
- `/markets/{pm_market_id}/tags` - Market tags/categories

**Important:** TokenRegistered events emit twice per market (tokens swapped). We deduplicate by `condition_id`.

### 3. Database Schema

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

**Key Design:** Tags are normalized (separate table) to allow future tag metadata extensions without schema changes.

## Binaries

### `market_backfill`

Indexes historical markets with flexible time ranges:

```bash
cargo run --bin market_backfill -- --days 7
cargo run --bin market_backfill -- --hours 6
cargo run --bin market_backfill -- --minutes 30
cargo run --bin market_backfill -- --from-block X --to-block Y
```

Features:

- Deduplicates events (2 events → 1 market)
- Skips existing markets (idempotent)
- Fetches metadata + tags from Gamma API
- Handles missing metadata gracefully

### `stream` (TODO)

Live event streaming via WebSocket

### `test_db`

Quick database connection test

## Development Workflow

### Database Change Workflow

When making schema changes, follow these steps in order:

1. **Write migration** - Create SQL file in `migrations/YYYYMMDD_description.sql`
2. **Update Rust code** - Modify models and queries to match new schema
3. **Run migrations** - `DATABASE_URL="..." sqlx migrate run` (applies to local DB)
4. **Regenerate sqlx cache** - `DATABASE_URL="..." cargo sqlx prepare` (updates `.sqlx/` for editor)
5. **Verify** - Check editor shows no errors, code compiles
6. **Commit both** - Git add both `migrations/` AND `.sqlx/` directories

**Important:** Always commit the `.sqlx` directory with migrations! It enables offline compilation and CI/CD.

### Local Development

```bash
# Database: postgresql://lakshmansankar@localhost/polymarket
sqlx migrate run                # Apply migrations
cargo sqlx prepare              # Regenerate .sqlx cache for editor
cargo run --bin market_backfill -- --hours 1
```

Add DATABASE_URL env var to project editor settings (for sqlx syntax checking)

### Production

Switch `.env` to Neon database URL

## Code Structure

```
src/
├── client/
│   ├── evm.rs          # Polygon RPC (HTTP/WebSocket)
│   ├── gamma.rs        # Gamma API (markets + tags)
│   └── mod.rs
├── polymarket/
│   ├── events.rs       # TokenRegistered parsing
│   ├── market.rs       # Market + Tag structs (API models)
│   ├── constants.rs    # Contract addresses, signatures
│   └── mod.rs
├── db/
│   ├── markets.rs      # Market DB operations
│   ├── market_tags.rs  # Tag DB operations
│   ├── models.rs       # DB row models (Market, Tag, MarketTag)
│   └── mod.rs
└── bin/
    ├── market_backfill.rs
    ├── stream.rs
    └── test_db.rs
```

## Resources

- **Gamma API Docs:** https://docs.polymarket.com/
- **CTFExchange:** `0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E`
- **Test Block:** 78975130 (known market registrations)
- **Polygon RPC:** Alchemy

## Recovery Notes

If context is lost:

1. Check this file for system architecture
2. Run `cargo run --bin test_db` to verify DB connection
3. Check migrations in `migrations/` for current schema
4. Review `market_backfill.rs` for current pipeline
