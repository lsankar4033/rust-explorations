# Polymarket Indexer - Project Plan

## Project Goal

Build a Rust-based indexer for Polymarket prediction markets as part of a broader Rust learning journey. The indexer captures on-chain market registration events and enriches them with metadata from Polymarket's APIs.

## Architecture Overview

```
┌─────────────────────────────────────────────────┐
│  LAYER 1: ON-CHAIN EVENT CAPTURE               │
│  - Monitor CTFExchange contract (Polygon)       │
│  - Capture TokenRegistered events               │
│  - Extract: conditionId, token0, token1         │
└─────────────────┬───────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────┐
│  LAYER 2: API ENRICHMENT                        │
│  - Gamma API: Query market metadata             │
│  - Lookup by conditionId                        │
│  - Get: question, description, volume, etc.     │
└─────────────────┬───────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────┐
│  LAYER 3: DATA STORAGE (TBD)                    │
│  - Store enriched market data                   │
│  - Enable fast queries                          │
└─────────────────────────────────────────────────┘
```

## Current Status

**What Works:**
- ✅ EVM client infrastructure (HTTP & WebSocket for Polygon via Alchemy)
- ✅ Log retrieval from CTFExchange contract
- ✅ Backfill binary successfully fetches raw TokenRegistered events
- ✅ Tested on block 78975130 with known market registrations

**Current Phase:**
- 🔄 Implementing log parsing (raw Log → TokenRegistered struct)
- 🔄 Building Gamma API client for metadata enrichment

**What's Next:**
- Parse TokenRegistered events from raw logs
- Create Gamma API HTTP client
- Implement enrichment pipeline (conditionId → market metadata)
- Choose storage layer (PostgreSQL/SQLite)

## Data Flow

### 1. On-Chain Event Capture

**Contract:** CTFExchange on Polygon
**Address:** `0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E`
**Event:** `TokenRegistered(uint256 indexed token0, uint256 indexed token1, bytes32 indexed conditionId)`

**What we extract:**
- `conditionId` - Unique identifier for the market (32 bytes)
- `token0` - ERC1155 token ID for YES outcome
- `token1` - ERC1155 token ID for NO outcome
- `block_number` - When market was registered
- `tx_hash` - Registration transaction

**Key insight:** Each TokenRegistered event represents a new binary prediction market. The two tokens are complementary outcomes (YES/NO) backed by 1 USDC collateral.

### 2. API Enrichment via Gamma API

**Base URL:** `https://gamma-api.polymarket.com`
**Authentication:** None required (public API)
**Primary Endpoint:** `GET /markets`

**Query pattern:**
```bash
GET https://gamma-api.polymarket.com/markets?condition_ids={conditionId}
```

**Metadata we get:**
- Market question and description
- Category (Politics, Sports, Crypto, etc.)
- Outcome labels (typically ["Yes", "No"])
- Current outcome prices
- Trading volume and liquidity
- Market status (active, closed, archived)
- Start/end dates
- Associated events and tags

**Important:** Convert conditionId from bytes32 to hex string with `0x` prefix for API query.

## Key Resources

### Contracts
- **CTFExchange:** `0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E` (emits TokenRegistered)
- **CTF (Conditional Tokens):** `0x4D97DCd97eC945f40cF65F87097ACe5EA0476045` (token framework)

### APIs
- **Gamma API Docs:** https://docs.polymarket.com/quickstart/introduction/main
- **Gamma Markets Endpoint:** `https://gamma-api.polymarket.com/markets`
- **Alternative (if Gamma down):** `https://strapi-matic.poly.market`

### Development
- **RPC Provider:** Alchemy (Polygon)
- **Test Block:** 78975130 (has known TokenRegistered events)

## Implementation Roadmap

### Phase 1: Event Parsing (Current)
- [x] Basic log retrieval
- [ ] Parse raw logs into TokenRegistered structs
- [ ] Extract conditionId, token0, token1 from log topics
- [ ] Add proper error handling for malformed logs

### Phase 2: Gamma API Integration
- [ ] Create HTTP client for Gamma API (using reqwest)
- [ ] Implement `get_market_by_condition_id()` function
- [ ] Add retry logic with exponential backoff
- [ ] Handle API rate limiting (100-200ms delays)
- [ ] Gracefully handle missing/delayed markets

### Phase 3: Enrichment Pipeline
- [ ] Define Market metadata struct
- [ ] Combine event parsing + API enrichment
- [ ] Handle data validation (token IDs should match)
- [ ] Add logging for pipeline steps

### Phase 4: Storage Layer
- [ ] Choose database (PostgreSQL recommended)
- [ ] Design schema for markets table
- [ ] Implement storage operations
- [ ] Add indexing for fast lookups

### Phase 5: Full Indexer
- [ ] Complete backfill binary (historical sync)
- [ ] Complete stream binary (live events)
- [ ] Add monitoring and health checks
- [ ] Handle edge cases and errors gracefully

## Important Notes

### Gamma API Status
- **Warning:** Polymarket documentation mentions Gamma API is being rebuilt/deprecated
- **Current Status:** Gamma API still works and is widely used
- **Fallback:** `strapi-matic.poly.market` available as alternative
- **Action:** Monitor for migration announcements

### Rate Limiting
- No official limits documented for Gamma API
- **Recommendation:** 100-200ms delay between requests
- Support batch queries (multiple condition_ids in array)
- Implement exponential backoff for retries

### Data Consistency
- New markets may have delay between on-chain event and API availability
- Implement retry logic for newly registered markets
- Validate that API `clobTokenIds` match on-chain `token0/token1`

### Conditional Token Framework
- All markets are binary (2 outcomes: YES/NO)
- 1 USDC can be "split" into 1 YES + 1 NO token
- 1 YES + 1 NO token can be "merged" back to 1 USDC
- Resolution via UMA oracle - winner takes all (1 token → 1 USDC)

## Code Structure

```
polymarket-indexer/
├── src/
│   ├── client/
│   │   ├── evm.rs          # On-chain RPC client (HTTP/WS)
│   │   ├── gamma.rs        # TODO: Gamma API client
│   │   └── mod.rs
│   ├── polymarket/
│   │   ├── events.rs       # TokenRegistered event
│   │   ├── market.rs       # TODO: Market metadata struct
│   │   ├── addresses.rs    # Contract addresses
│   │   ├── core.rs         # High-level business logic
│   │   └── mod.rs
│   ├── storage/            # TODO: Database layer
│   └── lib.rs
├── bin/
│   ├── backfill.rs         # Historical event sync
│   └── stream.rs           # Live event streaming
└── PLAN.md                 # This file
```

## Session Recovery

If you lose context, review:
1. This PLAN.md for high-level architecture
2. TODOs in code comments for next steps
3. Git log for recent changes
4. Backfill binary to see what's currently working
