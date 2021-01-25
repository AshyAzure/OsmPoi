SELECT COUNT(*)
FROM relations
WHERE relation_id IN (SELECT reference_id
                      FROM relation_references
                      WHERE relation_id = ?
                      AND reference_type = 2
                      AND dep = 0);
