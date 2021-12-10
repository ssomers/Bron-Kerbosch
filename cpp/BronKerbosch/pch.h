#pragma once

#ifdef _MSC_VER
#    pragma warning(disable : 4365)  // conversion from ... to ..., signed/unsigned mismatch
#    pragma warning(disable : 4514)  // unreferenced inline function has been removed
#    pragma warning(disable : 4571)  // catch (…) semantics changed since Visual C++ 7.1
#    pragma warning(disable : 4710)  // function not inlined
#    pragma warning(disable : 4711)  // function …  selected for automatic inline expansion
#    pragma warning(disable : 4625)  // copy constructor was implicitly defined as deleted
#    pragma warning(disable : 4626)  // assignment operator was implicitly defined as deleted
#    pragma warning(disable : 4820)  // bytes padding added after data member
#    pragma warning(disable : 5026)  // move constructor was implicitly defined as deleted
#    pragma warning(disable : 5027)  // move assignment operator was implicitly defined as deleted
#    pragma warning(disable : 5039)  // pointer or reference to potentially throwing function passed
                                     // to 'extern "C"' function under -EHc.
#    pragma warning(disable : 5045)  // Compiler will insert Spectre mitigation for memory load if
                                     // /Qspectre switch specified
#endif

#include <algorithm>
#include <cassert>
#include <iterator>
#include <optional>
#include <set>
#include <vector>
