#pragma once
#include "../windissect_forwards.h"

// Reconstructed from Microsoft.Internal.WarpPal.dll by Windissect. 7 member(s).
namespace winrt {
class impl {
public:
    class atomic_ref_count;
    class bstr_traits;
    class heap_traits;
    class hstring_traits;
public /*unspecified*/:
    // Category: Method | Source: PDB Internal
    // Symbol: ?create_hstring_on_heap@impl@winrt@@YAPEAUhstring_header@12@PEBGI@Z
    WindissectOpaque * create_hstring_on_heap(unsigned short const *, unsigned int);
    // Category: Method | Source: PDB Internal
    // Symbol: ?message_from_hresult@impl@winrt@@YA?AUhstring@2@Uhresult@2@@Z
    WindissectOpaque message_from_hresult(WindissectOpaque);
    // Category: Method | Source: PDB Internal
    // Symbol: ?precreate_hstring_on_heap@impl@winrt@@YAPEAUshared_hstring_header@12@I@Z
    WindissectOpaque * precreate_hstring_on_heap(unsigned int);
    // Category: Method | Source: PDB Internal
    // Symbol: ?release_hstring@impl@winrt@@YAXPEAUhstring_header@12@@Z
    void release_hstring(WindissectOpaque *);
    // Category: Method | Source: PDB Internal
    // Symbol: ?trim_hresult_message@impl@winrt@@YA?AUhstring@2@QEBGI@Z
    WindissectOpaque trim_hresult_message(unsigned short const * const, unsigned int);
};
} // namespace winrt
