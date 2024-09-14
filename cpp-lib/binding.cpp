#include <iostream>

#include "binding.h"

void foo(void) {
	std::cout << "foo" << std::endl;
}

int bar(int a, int b) {
	int c = a + b;
	std::cout << "bar = " << c << std::endl;
	return c;
}
