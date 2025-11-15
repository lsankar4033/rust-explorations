-- Create market_tags table for storing tags associated with markets

CREATE TABLE market_tags (
    id SERIAL PRIMARY KEY,
    condition_id TEXT NOT NULL REFERENCES markets(condition_id) ON DELETE CASCADE,
    pm_tag_id TEXT NOT NULL,
    label TEXT,
    slug TEXT,
    UNIQUE(condition_id, pm_tag_id)
);

-- Indexes for efficient queries
CREATE INDEX idx_market_tags_condition_id ON market_tags(condition_id);
CREATE INDEX idx_market_tags_slug ON market_tags(slug);
