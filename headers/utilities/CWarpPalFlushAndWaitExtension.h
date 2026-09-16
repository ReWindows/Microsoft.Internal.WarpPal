#pragma once
#include "../windissect_forwards.h"

// Reconstructed from Microsoft.Internal.WarpPal.dll by Windissect. 3 member(s).
class CWarpPalFlushAndWaitExtension {
public:
    // Category: Method | Source: PDB Internal
    // Symbol: ?FlushAndWait@CWarpPalFlushAndWaitExtension@@UEAAJXZ
    virtual long FlushAndWait();
protected:
    // Category: Ctor | Source: PDB Internal
    // Symbol: ??0CWarpPalFlushAndWaitExtension@@IEAA@PEAVIWarpPrivateAPI@@@Z
    CWarpPalFlushAndWaitExtension(IWarpPrivateAPI *);
    // Category: Dtor | Source: PDB Internal
    // Symbol: ??1CWarpPalFlushAndWaitExtension@@IEAA@XZ
    ~CWarpPalFlushAndWaitExtension();
};
