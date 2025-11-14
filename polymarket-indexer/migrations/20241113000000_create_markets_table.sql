-- Create markets table
CREATE TABLE IF NOT EXISTS markets (
    -- Primary key: condition ID from TokenRegistered event
    condition_id TEXT PRIMARY KEY,

    -- On-chain data from TokenRegistered event
    token0 TEXT NOT NULL,
    token1 TEXT NOT NULL,
    block_number BIGINT NOT NULL,
    tx_hash TEXT NOT NULL,

    -- Metadata from Gamma API (optional, may be null if API fails)
    question TEXT,
    slug TEXT,
    category TEXT,
    outcomes JSONB,  -- Store as JSON: ["Yes", "No"]
    start_date TEXT,
    end_date TEXT,

    -- Tracking timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    metadata_fetched_at TIMESTAMPTZ  -- NULL if not yet fetched
);

-- Indexes for common queries
CREATE INDEX IF NOT EXISTS idx_markets_block_number ON markets(block_number);
CREATE INDEX IF NOT EXISTS idx_markets_created_at ON markets(created_at);
CREATE INDEX IF NOT EXISTS idx_markets_slug ON markets(slug) WHERE slug IS NOT NULL;

-- Function to automatically update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger to call the function before updates
CREATE TRIGGER update_markets_updated_at
    BEFORE UPDATE ON markets
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
