#include "pch.h"

#include "Console.h"

#ifdef _MSC_VER

#    define WIN32_LEAN_AND_MEAN  // Exclude rarely-used stuff from Windows headers
#    include <Windows.h>

#endif

void console_init() {
#ifdef _MSC_VER
    SetConsoleOutputCP(65001);  // UTF-8
#endif
}
