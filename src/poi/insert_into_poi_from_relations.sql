INSERT INTO poi (poi_type, lat, lon, d_lat, d_lon, tags)
SELECT 1, (lat_lb + lat_rt) / 2, (lon_lb + lon_rt) / 2, (lat_rt - lat_lb) / 2, (lon_rt - lon_lb) / 2, tags FROM relations
WHERE has_name = 1;
