#include "pch.h"

#ifdef _MSC_VER

#define WIN32_LEAN_AND_MEAN             // Exclude rarely-used stuff from Windows headers
#define NOMINMAX
#include <windows.h>

#endif

void console_init() {
#ifdef _MSC_VER
    SetConsoleOutputCP(65001); // UTF-8
#endif
}
