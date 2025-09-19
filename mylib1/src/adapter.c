#include <string.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

/**
*/
typedef struct  {
  bool   flag; // demonstrating memory layout
  size_t val;
} data_t;

/**
*/
static data_t*
new_data(const char* id) {
  data_t *obj = (data_t*) malloc(sizeof(data_t));
  if (obj==NULL) {
    return NULL;
  }
  obj->flag = true;
  obj->val = 1000 +  strlen(id);
  return obj;
}

/**
*/
data_t*
lookup_data(const char* id) {
  if (!strncmp("FOREIGN", id, strlen("FOREIGN"))) {
    return new_data(id);
  }
  return NULL;
}

///
///
void
free_data(data_t* obj) {
  if (obj != NULL) {
    free(obj);
  }
}




