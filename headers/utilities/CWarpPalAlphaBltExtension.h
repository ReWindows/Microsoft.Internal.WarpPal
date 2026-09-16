#pragma once
#include "../windissect_forwards.h"

// Reconstructed from Microsoft.Internal.WarpPal.dll by Windissect. 12 member(s).
class CWarpPalAlphaBltExtension {
public:
    // Category: Method | Source: PDB Internal
    // Symbol: ?ClearState@CWarpPalAlphaBltExtension@@UEAAXXZ
    virtual void ClearState();
    // Category: Method | Source: PDB Internal
    // Symbol: ?DrawRect@CWarpPalAlphaBltExtension@@UEAAJAEBUD2D_RECT_F@@PEBU_D3DCOLORVALUE@@PEBUD2D_MATRIX_4X3_F@@M@Z
    virtual long DrawRect(D2D_RECT_F const &, _D3DCOLORVALUE const *, D2D_MATRIX_4X3_F const *, float);
    // Category: Accessor | Source: PDB Internal
    // Symbol: ?SetBlendMode@CWarpPalAlphaBltExtension@@UEAAXW4WarpPalBlendMode@@@Z
    virtual void SetBlendMode(int);
    // Category: Accessor | Source: PDB Internal
    // Symbol: ?SetRasterStateFlags@CWarpPalAlphaBltExtension@@UEAAXI@Z
    virtual void SetRasterStateFlags(unsigned int);
    // Category: Accessor | Source: PDB Internal
    // Symbol: ?SetTextureStates@CWarpPalAlphaBltExtension@@UEAAXPEBUWarpPalTextureState@@I@Z
    virtual void SetTextureStates(WarpPalTextureState const *, unsigned int);
    // Category: Method | Source: PDB Internal
    // Symbol: ?SnapAliasedEdgesToPixelAlignment@CWarpPalAlphaBltExtension@@UEAAXAEBUD2D_RECT_F@@IPEAU2@@Z
    virtual void SnapAliasedEdgesToPixelAlignment(D2D_RECT_F const &, unsigned int, D2D_RECT_F *);
protected:
    // Category: Ctor | Source: PDB Internal
    // Symbol: ??0CWarpPalAlphaBltExtension@@IEAA@PEAVIWarpPrivateAPI@@@Z
    CWarpPalAlphaBltExtension(IWarpPrivateAPI *);
    // Category: Dtor | Source: PDB Internal
    // Symbol: ??1CWarpPalAlphaBltExtension@@IEAA@XZ
    ~CWarpPalAlphaBltExtension();
};
