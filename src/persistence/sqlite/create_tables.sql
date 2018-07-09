CREATE TABLE IF NOT EXISTS nodes (
    id INTEGER PRIMARY KEY,
    name TEXT,
    version TEXT
);

CREATE TABLE IF NOT EXISTS sensors (
    id INTEGER,
    node_id INTEGER,
    type TEXT,
    description TEXT,
    PRIMARY KEY(id, node_id),
    FOREIGN KEY(node_id) REFERENCES nodes(id)
);

CREATE TABLE IF NOT EXISTS readings (
    node_id INTEGER,
    sensor_id INTEGER,
    timestamp DATETIME,
    value TEXT,
    kind TEXT,
    FOREIGN KEY(node_id) REFERENCES nodes(id)
);