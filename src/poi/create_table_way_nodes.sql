CREATE TABLE way_nodes (
    way_id  INTEGER NOT NULL CHECK(way_id >= 0),
    node_id INTEGER NOT NULL CHECK(node_id >= 0)
);

