#pragma once
#include "../windissect_forwards.h"

// Reconstructed from Microsoft.Internal.WarpPal.dll by Windissect. 5 member(s).
class CWarpPalExtensionFactory {
public:
    // Category: Method | Source: PDB Internal
    // Symbol: ?CreateExtension@CWarpPalExtensionFactory@@UEAAJAEBU_GUID@@PEAPEAX@Z
    virtual long CreateExtension(_GUID const &, void * *);
    // Category: Refcount | Source: PDB Internal
    // Symbol: ?QueryInterface@CWarpPalExtensionFactory@@UEAAJAEBU_GUID@@PEAPEAX@Z
    virtual long QueryInterface(_GUID const &, void * *);
protected:
    // Category: Ctor | Source: PDB Internal
    // Symbol: ??0CWarpPalExtensionFactory@@IEAA@PEAVIWarpPrivateAPI@@I@Z
    CWarpPalExtensionFactory(IWarpPrivateAPI *, unsigned int);
    // Category: Dtor | Source: PDB Internal
    // Symbol: ??1CWarpPalExtensionFactory@@IEAA@XZ
    ~CWarpPalExtensionFactory();
private:
    // Category: Accessor | Source: PDB Internal
    // Symbol: ?IsExtensionEnabled@CWarpPalExtensionFactory@@AEBA_NW4Enum@WarpExtension@@@Z
    bool IsExtensionEnabled(int) const;
};
