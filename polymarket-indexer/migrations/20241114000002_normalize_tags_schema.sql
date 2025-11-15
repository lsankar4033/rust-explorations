-- Normalize tags schema: create separate tags table and make market_tags a pure join table

-- Create tags table to store tag metadata
CREATE TABLE tags (
    pm_tag_id TEXT PRIMARY KEY,
    label TEXT,
    slug TEXT
);

-- Create index on slug for efficient filtering
CREATE INDEX idx_tags_slug ON tags(slug);

-- Drop existing market_tags table
DROP TABLE IF EXISTS market_tags;

-- Recreate market_tags as a pure join table
CREATE TABLE market_tags (
    condition_id TEXT NOT NULL REFERENCES markets(condition_id) ON DELETE CASCADE,
    pm_tag_id TEXT NOT NULL REFERENCES tags(pm_tag_id) ON DELETE CASCADE,
    PRIMARY KEY (condition_id, pm_tag_id)
);

-- Indexes for efficient queries
CREATE INDEX idx_market_tags_condition_id ON market_tags(condition_id);
CREATE INDEX idx_market_tags_pm_tag_id ON market_tags(pm_tag_id);
