#pragma once
#include "../windissect_forwards.h"

// Reconstructed from Microsoft.Internal.WarpPal.dll by Windissect. 2 member(s).
namespace winrt::impl {
class hstring_traits {
public:
    // Category: Method | Source: PDB Internal
    // Symbol: ?close@hstring_traits@impl@winrt@@SAXPEAUhstring_header@23@@Z
    static void close(WindissectOpaque *);
    // Category: Method | Source: PDB Internal
    // Symbol: ?invalid@hstring_traits@impl@winrt@@SAPEAUhstring_header@23@XZ
    static WindissectOpaque * invalid();
};
} // namespace winrt::impl
