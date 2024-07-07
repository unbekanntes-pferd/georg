-- Create geo_locations table
CREATE TABLE IF NOT EXISTS geo_locations (
    id TEXT PRIMARY KEY,
    lat REAL NOT NULL,
    lon REAL NOT NULL
);

-- Create an index on the id for search
CREATE INDEX IF NOT EXISTS idx_geo_locations_id ON geo_locations(id);