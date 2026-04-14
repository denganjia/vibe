-- src/state/schema.sql
CREATE TABLE IF NOT EXISTS panes (
    vibe_id TEXT PRIMARY KEY,
    physical_id TEXT NOT NULL,
    terminal_type TEXT NOT NULL,
    metadata TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
