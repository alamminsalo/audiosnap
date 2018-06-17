#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

size_t c_data(int16_t *buf, size_t _len);

size_t c_file_len(void);

size_t c_load_file(const char *file);

size_t c_split(int16_t ceil);

size_t c_splits(uint32_t *buf, size_t _len);

const char *c_version(void);
