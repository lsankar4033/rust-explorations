-- Alter markets table: add pm_market_id and remove category

ALTER TABLE markets DROP COLUMN IF EXISTS category;
ALTER TABLE markets ADD COLUMN IF NOT EXISTS pm_market_id TEXT;
