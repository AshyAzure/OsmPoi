#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * The C representation type of OsmCount.
 */
typedef struct OSMPOI_OsmCount {
  int64_t node;
  int64_t way;
  int64_t relation;
} OSMPOI_OsmCount;

struct OSMPOI_OsmCount osmpoi_count(const int8_t *path);

int32_t osmpoi_dump(const int8_t *pbf_path, const int8_t *db_path);

int32_t osmpoi_parse_ways(const int8_t *dataset_path);

int32_t osmpoi_parse_relations(const int8_t *dataset_path);

int32_t osmpoi_refine(const int8_t *dataset_path);

int32_t osmpoi_query_csv(const int8_t *input_path,
                         const int8_t *output_path,
                         const int8_t *dataset_path,
                         float distance,
                         bool strict);
