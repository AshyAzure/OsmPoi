CREATE TABLE relation_references (
    relation_id    INTEGER NOT NULL CHECK(relation_id >= 0),
    reference_id   INTEGER NOT NULL CHECK(reference_id >= 0),
    reference_type INTEGER NOT NULL CHECK(reference_type BETWEEN 0 AND 2)
);
