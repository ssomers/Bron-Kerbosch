#include "pch.h"

#include "Console.h"

#ifdef _MSC_VER

#    define WIN32_LEAN_AND_MEAN  // Exclude rarely-used stuff from Windows headers
#    include <Windows.h>

#endif

void console_init() {
#ifdef _MSC_VER
    SetConsoleOutputCP(CP_UTF8);
    // Enable buffering to prevent VS from chopping up UTF-8 byte sequences
    setvbuf(stdout, nullptr, _IOFBF, 512);
#endif
}
