#pragma once

#ifdef __cplusplus
extern "C" {
#endif

	__declspec(dllexport) void foo(void);

	__declspec(dllexport) int bar(int a, int b);

#ifdef __cplusplus
}
#endif
