SELECT MIN(lat), MAX(lat), MIN(lon), MAX(lon) 
FROM nodes
WHERE node_id IN (SELECT node_id
                  FROM way_nodes
                  WHERE way_id = ?);
