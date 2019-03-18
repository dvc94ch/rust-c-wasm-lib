#include <stdio.h>
#include "bindings.h"

int main() {
  void* fib = fib_new();
  for (int i = 0; i < 7; i++) {
    printf("%d\n", fib_next(fib));
  }
  fib_drop(fib);
  return 0;
}
