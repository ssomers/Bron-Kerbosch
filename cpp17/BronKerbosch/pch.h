#pragma once

#ifdef _MSC_VER
#  pragma warning(disable:4355) // 'this': used in base member initializer list (for boost)
#  pragma warning(disable:4365) // conversion from ... to ..., signed/unsigned mismatch
#  pragma warning(disable:4371) // layout of class may have changed from a previous version of the compiler due to better packing (for boost)
#  pragma warning(disable:4514) // unreferenced inline function has been removed
#  pragma warning(disable:4571) // catch (...) semantics changed since Visual C++ 7.1; structured exceptions(SEH) are no longer caught (for boost)
#  pragma warning(disable:4619) // #pragma warning: there is no warning number ... (for boost)
#  pragma warning(disable:4710) // function not inlined
#  pragma warning(disable:4711) // function ...  selected for automatic inline expansion
#  pragma warning(disable:4625) // copy constructor was implicitly defined as deleted
#  pragma warning(disable:4626) // assignment operator was implicitly defined as deleted
#  pragma warning(disable:4643) // Forward declaring ... in namespace std is not permitted by the C++ Standard (for boost)
#  pragma warning(disable:4668) // ... is not defined as a preprocessor macro, replacing with '0' for '#if/#elif' (for boost)
#  pragma warning(disable:4820) // bytes padding added after data member
#  pragma warning(disable:5026) // move constructor was implicitly defined as deleted
#  pragma warning(disable:5027) // move assignment operator was implicitly defined as deleted
#  pragma warning(disable:5031) // #pragma warning(pop) : likely mismatch, popping warning state pushed in different file (for boost)
#  pragma warning(disable:5039) // pointer or reference to potentially throwing function passed to 'extern "C"' function under -EHc.
#  pragma warning(disable:5045) // Compiler will insert Spectre mitigation for memory load if /Qspectre switch specified
#  pragma warning(disable:5204) // class has virtual functions, but its trivial destructor is not virtual (for boost)
#  pragma warning(disable:6285) // (<non - zero constant> || <non - zero constant>) is always a non - zero constant (for boost)
#  pragma warning(disable:26110) // Caller failing to hold lock ... before calling function (for boost)
#  pragma warning(disable:26495) // Variable ... is uninitialized (for boost)
#  pragma warning(disable:26812) // The enum type ... is unscoped.Prefer 'enum class' over 'enum' (for boost)
#endif

#include <algorithm>
#include <cassert>
#include <iterator>
#include <optional>
#include <set>
#include <vector>
