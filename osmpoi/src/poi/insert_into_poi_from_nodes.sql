INSERT INTO poi (poi_type, lat, lon, d_lat, d_lon, tags)
SELECT 0, lat, lon, 0, 0, tags FROM nodes
WHERE has_name = 1;
