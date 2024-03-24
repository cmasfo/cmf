
#include <stdio.h>

#include "prelude.h"

void println(const char* const format, ...) {
  printf(format);
  printf("\n");
}
