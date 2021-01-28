UPDATE relations
SET dep = 1, lat_lb = ?1, lon_lb = ?2, lat_rt = ?3, lon_rt = ?4
WHERE relation_id = ?5;
