
#include <stdio.h>

#include "println.h"

void println(const char* const format, ...) {
  printf(format);
  printf("\n");
}
