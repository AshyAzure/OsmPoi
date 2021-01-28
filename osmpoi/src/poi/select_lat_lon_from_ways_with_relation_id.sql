SELECT MIN(lat_lb), MAX(lat_rt), MIN(lon_lb), MAX(lon_rt) 
FROM ways
WHERE way_id IN (SELECT reference_id
                 FROM relation_references
                 WHERE relation_id = ?
                 AND reference_type = 1);
