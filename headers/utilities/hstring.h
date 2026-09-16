#pragma once
#include "../windissect_forwards.h"

// Reconstructed from Microsoft.Internal.WarpPal.dll by Windissect. 5 member(s).
namespace winrt {
class hstring {
public:
    // Category: Method | Source: PDB Internal
    // Symbol: ?begin@hstring@winrt@@QEBAPEBGXZ
    unsigned short const * begin() const;
    // Category: Method | Source: PDB Internal
    // Symbol: ?c_str@hstring@winrt@@QEBAPEBGXZ
    unsigned short const * c_str() const;
    // Category: Method | Source: PDB Internal
    // Symbol: ?empty@hstring@winrt@@QEBA_NXZ
    bool empty() const;
    // Category: Ctor | Source: PDB Internal
    // Symbol: ??0hstring@winrt@@QEAA@PEBGI@Z
    hstring(unsigned short const *, unsigned int);
    // Category: Dtor | Source: PDB Internal
    // Symbol: ??1hstring@winrt@@QEAA@XZ
    ~hstring();
};
} // namespace winrt
