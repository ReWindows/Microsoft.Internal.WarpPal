#pragma once
#include "../windissect_forwards.h"

// Reconstructed from Microsoft.Internal.WarpPal.dll by Windissect. 4 member(s).
class CWarpPalCreateSharedResourceExtension {
public:
    // Category: Method | Source: PDB Internal
    // Symbol: ?BeginCreateSharedResource@CWarpPalCreateSharedResourceExtension@@UEAAJPEAU_SECURITY_ATTRIBUTES@@@Z
    virtual long BeginCreateSharedResource(_SECURITY_ATTRIBUTES *);
    // Category: Method | Source: PDB Internal
    // Symbol: ?EndCreateSharedResource@CWarpPalCreateSharedResourceExtension@@UEAAJPEAGI@Z
    virtual long EndCreateSharedResource(unsigned short *, unsigned int);
protected:
    // Category: Ctor | Source: PDB Internal
    // Symbol: ??0CWarpPalCreateSharedResourceExtension@@IEAA@PEAVIWarpPrivateAPI@@@Z
    CWarpPalCreateSharedResourceExtension(IWarpPrivateAPI *);
    // Category: Dtor | Source: PDB Internal
    // Symbol: ??1CWarpPalCreateSharedResourceExtension@@IEAA@XZ
    ~CWarpPalCreateSharedResourceExtension();
};
