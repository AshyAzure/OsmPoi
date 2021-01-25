SELECT MIN(lat), MAX(lat), MIN(lon), MAX(lon) 
FROM nodes
WHERE node_id IN (SELECT reference_id
                  FROM relation_references
                  WHERE relation_id = ?
                  AND reference_type = 0);
