#pragma once
#include "../windissect_forwards.h"

// Reconstructed from Microsoft.Internal.WarpPal.dll by Windissect. 4 member(s).
class CWarpPalLockSubresourceExtension {
public:
    // Category: Method | Source: PDB Internal
    // Symbol: ?LockSubresource@CWarpPalLockSubresourceExtension@@UEAAJPEAUIDXGIResource@@IW4D3D10_MAP@@IPEAUD3D10_MAPPED_TEXTURE2D@@@Z
    virtual long LockSubresource(IDXGIResource *, unsigned int, int, unsigned int, D3D10_MAPPED_TEXTURE2D *);
    // Category: Method | Source: PDB Internal
    // Symbol: ?UnlockSubresource@CWarpPalLockSubresourceExtension@@UEAAJPEAUIDXGIResource@@I@Z
    virtual long UnlockSubresource(IDXGIResource *, unsigned int);
protected:
    // Category: Ctor | Source: PDB Internal
    // Symbol: ??0CWarpPalLockSubresourceExtension@@IEAA@PEAVIWarpPrivateAPI@@@Z
    CWarpPalLockSubresourceExtension(IWarpPrivateAPI *);
    // Category: Dtor | Source: PDB Internal
    // Symbol: ??1CWarpPalLockSubresourceExtension@@IEAA@XZ
    ~CWarpPalLockSubresourceExtension();
};
