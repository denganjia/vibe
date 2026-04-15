-- src/state/schema.sql
CREATE TABLE IF NOT EXISTS panes (
    vibe_id TEXT PRIMARY KEY,
    physical_id TEXT NOT NULL,
    terminal_type TEXT NOT NULL,
    role TEXT,
    status TEXT,
    summary TEXT,
    pid INTEGER,
    last_heartbeat_at DATETIME,
    metadata TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
