//! Evidence-labelled vftable metadata and raw slot access.
//! Slot calls remain the consumer's responsibility until a signature is unique.

use core::ffi::c_void;

#[derive(Clone, Copy, Debug)]
pub struct VTableInfo { pub name: &'static str, pub rva: u32, pub first_slot: usize, pub slot_count: usize, pub confidence: &'static str }

#[derive(Clone, Copy, Debug)]
pub struct VTableSlot { pub table_rva: u32, pub slot: u32, pub byte_offset: u32, pub target_rva: u32, pub target_id: Option<&'static str>, pub target_name: Option<&'static str>, pub ambiguous: bool, pub this_adjustment: Option<i32> }

pub static VTABLES: &[VTableInfo] = &[
    VTableInfo { name: "const type_info::`vftable'", rva: 0x12008, first_slot: 0, slot_count: 1, confidence: "likely" },
    VTableInfo { name: "const std::exception::`vftable'", rva: 0x12028, first_slot: 1, slot_count: 2, confidence: "likely" },
    VTableInfo { name: "const std::bad_alloc::`vftable'", rva: 0x12040, first_slot: 3, slot_count: 2, confidence: "likely" },
    VTableInfo { name: "const std::bad_array_new_length::`vftable'", rva: 0x12058, first_slot: 5, slot_count: 2, confidence: "likely" },
    VTableInfo { name: "const std::logic_error::`vftable'", rva: 0x12068, first_slot: 7, slot_count: 2, confidence: "likely" },
    VTableInfo { name: "const std::invalid_argument::`vftable'", rva: 0x12078, first_slot: 9, slot_count: 2, confidence: "likely" },
    VTableInfo { name: "const wil::ResultException::`vftable'", rva: 0x12088, first_slot: 11, slot_count: 2, confidence: "likely" },
    VTableInfo { name: "const CRefCountImpl<class CWarpPalExtensionFactory>::`vftable'", rva: 0x12098, first_slot: 13, slot_count: 4, confidence: "likely" },
    VTableInfo { name: "const CRefCountImpl<class CWarpPalLockSubresourceExtension>::`vftable'", rva: 0x120B8, first_slot: 17, slot_count: 5, confidence: "likely" },
    VTableInfo { name: "const CRefCountImpl<class CWarpPalFlushAndWaitExtension>::`vftable'", rva: 0x120E0, first_slot: 22, slot_count: 4, confidence: "likely" },
    VTableInfo { name: "const CRefCountImpl<class CWarpPalCreateSharedResourceExtension>::`vftable'", rva: 0x12100, first_slot: 26, slot_count: 5, confidence: "likely" },
    VTableInfo { name: "const CRefCountImpl<class CWarpPalAlphaBltExtension>::`vftable'", rva: 0x12128, first_slot: 31, slot_count: 9, confidence: "likely" },
    VTableInfo { name: "const IWarpPalAlphaBltExtension::`vftable'", rva: 0x12170, first_slot: 40, slot_count: 9, confidence: "likely" },
    VTableInfo { name: "const IWarpPalCreateSharedResourceExtension::`vftable'", rva: 0x121B8, first_slot: 49, slot_count: 5, confidence: "likely" },
    VTableInfo { name: "const IWarpPalExtensionFactory::`vftable'", rva: 0x121E0, first_slot: 54, slot_count: 4, confidence: "likely" },
    VTableInfo { name: "const CWarpExtensionBase<class IWarpPalAlphaBltExtension>::`vftable'", rva: 0x12200, first_slot: 58, slot_count: 9, confidence: "likely" },
    VTableInfo { name: "const CWarpExtensionBase<class IWarpPalCreateSharedResourceExtension>::`vftable'", rva: 0x12248, first_slot: 67, slot_count: 5, confidence: "likely" },
    VTableInfo { name: "const CWarpExtensionBase<class IWarpPalFlushAndWaitExtension>::`vftable'", rva: 0x12270, first_slot: 72, slot_count: 4, confidence: "likely" },
    VTableInfo { name: "const CWarpExtensionBase<class IWarpPalLockSubresourceExtension>::`vftable'", rva: 0x12290, first_slot: 76, slot_count: 5, confidence: "likely" },
];

pub static VTABLE_SLOTS: &[VTableSlot] = &[
    VTableSlot { table_rva: 0x12008, slot: 0, byte_offset: 0, target_rva: 0x1960, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12028, slot: 0, byte_offset: 0, target_rva: 0x4E00, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12028, slot: 1, byte_offset: 8, target_rva: 0xBAC0, target_id: Some("?what@exception@std@@UEBAPEBDXZ"), target_name: Some("public: virtual char const * __cdecl std::exception::what(void) const"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x12040, slot: 0, byte_offset: 0, target_rva: 0x4DB0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12040, slot: 1, byte_offset: 8, target_rva: 0xBAC0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12058, slot: 0, byte_offset: 0, target_rva: 0x2130, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12058, slot: 1, byte_offset: 8, target_rva: 0xBAC0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12068, slot: 0, byte_offset: 0, target_rva: 0x4DB0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12068, slot: 1, byte_offset: 8, target_rva: 0xBAC0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12078, slot: 0, byte_offset: 0, target_rva: 0x4E50, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12078, slot: 1, byte_offset: 8, target_rva: 0xBAC0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12088, slot: 0, byte_offset: 0, target_rva: 0x4D60, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12088, slot: 1, byte_offset: 8, target_rva: 0xB940, target_id: Some("?what@ResultException@wil@@UEBAPEBDXZ"), target_name: Some("public: virtual char const * __cdecl wil::ResultException::what(void) const"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x12098, slot: 0, byte_offset: 0, target_rva: 0xE450, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12098, slot: 1, byte_offset: 8, target_rva: 0xCE20, target_id: Some("?AddRef@?$CRefCountImpl@VCWarpPalExtensionFactory@@@@UEAAKXZ"), target_name: Some("public: virtual unsigned long __cdecl CRefCountImpl<class CWarpPalExtensionFactory>::AddRef(void)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x12098, slot: 2, byte_offset: 16, target_rva: 0xE5D0, target_id: Some("?Release@?$CRefCountImpl@VCWarpPalExtensionFactory@@@@UEAAKXZ"), target_name: Some("public: virtual unsigned long __cdecl CRefCountImpl<class CWarpPalExtensionFactory>::Release(void)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x12098, slot: 3, byte_offset: 24, target_rva: 0xCFA0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x120B8, slot: 0, byte_offset: 0, target_rva: 0xE3B0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x120B8, slot: 1, byte_offset: 8, target_rva: 0xCDF0, target_id: Some("?AddRef@?$CRefCountImpl@VCWarpPalLockSubresourceExtension@@@@UEAAKXZ"), target_name: Some("public: virtual unsigned long __cdecl CRefCountImpl<class CWarpPalLockSubresourceExtension>::AddRef(void)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x120B8, slot: 2, byte_offset: 16, target_rva: 0xE560, target_id: Some("?Release@?$CRefCountImpl@VCWarpPalLockSubresourceExtension@@@@UEAAKXZ"), target_name: Some("public: virtual unsigned long __cdecl CRefCountImpl<class CWarpPalLockSubresourceExtension>::Release(void)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x120B8, slot: 3, byte_offset: 24, target_rva: 0xE040, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x120B8, slot: 4, byte_offset: 32, target_rva: 0xEBB0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x120E0, slot: 0, byte_offset: 0, target_rva: 0xE310, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x120E0, slot: 1, byte_offset: 8, target_rva: 0xCDF0, target_id: Some("?AddRef@?$CRefCountImpl@VCWarpPalFlushAndWaitExtension@@@@UEAAKXZ"), target_name: Some("public: virtual unsigned long __cdecl CRefCountImpl<class CWarpPalFlushAndWaitExtension>::AddRef(void)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x120E0, slot: 2, byte_offset: 16, target_rva: 0xE560, target_id: Some("?Release@?$CRefCountImpl@VCWarpPalFlushAndWaitExtension@@@@UEAAKXZ"), target_name: Some("public: virtual unsigned long __cdecl CRefCountImpl<class CWarpPalFlushAndWaitExtension>::Release(void)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x120E0, slot: 3, byte_offset: 24, target_rva: 0xDD80, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12100, slot: 0, byte_offset: 0, target_rva: 0xE270, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12100, slot: 1, byte_offset: 8, target_rva: 0xCDF0, target_id: Some("?AddRef@?$CRefCountImpl@VCWarpPalCreateSharedResourceExtension@@@@UEAAKXZ"), target_name: Some("public: virtual unsigned long __cdecl CRefCountImpl<class CWarpPalCreateSharedResourceExtension>::AddRef(void)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x12100, slot: 2, byte_offset: 16, target_rva: 0xE560, target_id: Some("?Release@?$CRefCountImpl@VCWarpPalCreateSharedResourceExtension@@@@UEAAKXZ"), target_name: Some("public: virtual unsigned long __cdecl CRefCountImpl<class CWarpPalCreateSharedResourceExtension>::Release(void)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x12100, slot: 3, byte_offset: 24, target_rva: 0xCEB0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12100, slot: 4, byte_offset: 32, target_rva: 0xDC20, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12128, slot: 0, byte_offset: 0, target_rva: 0xE1D0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12128, slot: 1, byte_offset: 8, target_rva: 0xCDC0, target_id: Some("?AddRef@?$CRefCountImpl@VCWarpPalAlphaBltExtension@@@@UEAAKXZ"), target_name: Some("public: virtual unsigned long __cdecl CRefCountImpl<class CWarpPalAlphaBltExtension>::AddRef(void)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x12128, slot: 2, byte_offset: 16, target_rva: 0xE4F0, target_id: Some("?Release@?$CRefCountImpl@VCWarpPalAlphaBltExtension@@@@UEAAKXZ"), target_name: Some("public: virtual unsigned long __cdecl CRefCountImpl<class CWarpPalAlphaBltExtension>::Release(void)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x12128, slot: 3, byte_offset: 24, target_rva: 0xCF70, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12128, slot: 4, byte_offset: 32, target_rva: 0xE670, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12128, slot: 5, byte_offset: 40, target_rva: 0xE6A0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12128, slot: 6, byte_offset: 48, target_rva: 0xE7A0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12128, slot: 7, byte_offset: 56, target_rva: 0xD9D0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12128, slot: 8, byte_offset: 64, target_rva: 0xEAF0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12170, slot: 0, byte_offset: 0, target_rva: 0x2280, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12170, slot: 1, byte_offset: 8, target_rva: 0x2280, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12170, slot: 2, byte_offset: 16, target_rva: 0x2280, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12170, slot: 3, byte_offset: 24, target_rva: 0x2280, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12170, slot: 4, byte_offset: 32, target_rva: 0x2280, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12170, slot: 5, byte_offset: 40, target_rva: 0x2280, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12170, slot: 6, byte_offset: 48, target_rva: 0x2280, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12170, slot: 7, byte_offset: 56, target_rva: 0x2280, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12170, slot: 8, byte_offset: 64, target_rva: 0x2280, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x121B8, slot: 0, byte_offset: 0, target_rva: 0x2280, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x121B8, slot: 1, byte_offset: 8, target_rva: 0x2280, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x121B8, slot: 2, byte_offset: 16, target_rva: 0x2280, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x121B8, slot: 3, byte_offset: 24, target_rva: 0x2280, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x121B8, slot: 4, byte_offset: 32, target_rva: 0x2280, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x121E0, slot: 0, byte_offset: 0, target_rva: 0x2280, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x121E0, slot: 1, byte_offset: 8, target_rva: 0x2280, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x121E0, slot: 2, byte_offset: 16, target_rva: 0x2280, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x121E0, slot: 3, byte_offset: 24, target_rva: 0x2280, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12200, slot: 0, byte_offset: 0, target_rva: 0xE1D0, target_id: Some("?QueryInterface@?$CWarpExtensionBase@VIWarpPalAlphaBltExtension@@@@UEAAJAEBU_GUID@@PEAPEAX@Z"), target_name: Some("public: virtual long __cdecl CWarpExtensionBase<class IWarpPalAlphaBltExtension>::QueryInterface(struct _GUID const &, void * *)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x12200, slot: 1, byte_offset: 8, target_rva: 0x2280, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12200, slot: 2, byte_offset: 16, target_rva: 0x2280, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12200, slot: 3, byte_offset: 24, target_rva: 0x2280, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12200, slot: 4, byte_offset: 32, target_rva: 0x2280, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12200, slot: 5, byte_offset: 40, target_rva: 0x2280, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12200, slot: 6, byte_offset: 48, target_rva: 0x2280, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12200, slot: 7, byte_offset: 56, target_rva: 0x2280, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12200, slot: 8, byte_offset: 64, target_rva: 0x2280, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12248, slot: 0, byte_offset: 0, target_rva: 0xE270, target_id: Some("?QueryInterface@?$CWarpExtensionBase@VIWarpPalCreateSharedResourceExtension@@@@UEAAJAEBU_GUID@@PEAPEAX@Z"), target_name: Some("public: virtual long __cdecl CWarpExtensionBase<class IWarpPalCreateSharedResourceExtension>::QueryInterface(struct _GUID const &, void * *)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x12248, slot: 1, byte_offset: 8, target_rva: 0x2280, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12248, slot: 2, byte_offset: 16, target_rva: 0x2280, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12248, slot: 3, byte_offset: 24, target_rva: 0x2280, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12248, slot: 4, byte_offset: 32, target_rva: 0x2280, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12270, slot: 0, byte_offset: 0, target_rva: 0xE310, target_id: Some("?QueryInterface@?$CWarpExtensionBase@VIWarpPalFlushAndWaitExtension@@@@UEAAJAEBU_GUID@@PEAPEAX@Z"), target_name: Some("public: virtual long __cdecl CWarpExtensionBase<class IWarpPalFlushAndWaitExtension>::QueryInterface(struct _GUID const &, void * *)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x12270, slot: 1, byte_offset: 8, target_rva: 0x2280, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12270, slot: 2, byte_offset: 16, target_rva: 0x2280, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12270, slot: 3, byte_offset: 24, target_rva: 0x2280, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12290, slot: 0, byte_offset: 0, target_rva: 0xE3B0, target_id: Some("?QueryInterface@?$CWarpExtensionBase@VIWarpPalLockSubresourceExtension@@@@UEAAJAEBU_GUID@@PEAPEAX@Z"), target_name: Some("public: virtual long __cdecl CWarpExtensionBase<class IWarpPalLockSubresourceExtension>::QueryInterface(struct _GUID const &, void * *)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x12290, slot: 1, byte_offset: 8, target_rva: 0x2280, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12290, slot: 2, byte_offset: 16, target_rva: 0x2280, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12290, slot: 3, byte_offset: 24, target_rva: 0x2280, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x12290, slot: 4, byte_offset: 32, target_rva: 0x2280, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
];

/// Reads a raw function pointer from an object's primary vftable.
///
/// # Safety
/// `object` must point to a live object with a readable primary vftable,
/// and `slot` must be valid for that concrete object. This function does
/// not invent or transmute a callable signature.
pub unsafe fn raw_object_slot(object: *const c_void, slot: usize) -> Option<*const ()> {
if object.is_null() { return None; }
let table = unsafe { *(object.cast::<*const *const ()>()) };
if table.is_null() { return None; }
let target = unsafe { *table.add(slot) };
(!target.is_null()).then_some(target)
}
