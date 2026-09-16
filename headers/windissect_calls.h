// Windissect exact-build RVA call helpers — DO NOT EDIT.
// Configure set_resolver() with a loader that validates the expected build.
#pragma once
#include <cstddef>
#include <cstdint>
#include <cstdlib>

#if defined(_M_IX86) || defined(__i386__)
#error "Windissect typed direct calls currently require the Windows x64 ABI"
#endif

namespace windissect_calls {
using ResolveRva = const void* (*)(std::uint32_t);
#if defined(_WIN32)
extern "C" __declspec(dllimport) void* __stdcall GetModuleHandleW(const wchar_t*);
[[nodiscard]] inline const void* default_resolver(std::uint32_t rva) {
static constexpr wchar_t module_name[31] = {0x004D, 0x0069, 0x0063, 0x0072, 0x006F, 0x0073, 0x006F, 0x0066, 0x0074, 0x002E, 0x0049, 0x006E, 0x0074, 0x0065, 0x0072, 0x006E, 0x0061, 0x006C, 0x002E, 0x0057, 0x0061, 0x0072, 0x0070, 0x0050, 0x0061, 0x006C, 0x002E, 0x0064, 0x006C, 0x006C, 0x0000};
const auto base = static_cast<const std::uint8_t*>(GetModuleHandleW(module_name));
if (base == nullptr) return nullptr;
const auto pe = *reinterpret_cast<const std::uint32_t*>(base + 0x3c);
const auto live_timestamp = *reinterpret_cast<const std::uint32_t*>(base + pe + 8);
const auto image_size = *reinterpret_cast<const std::uint32_t*>(base + pe + 80);
if (live_timestamp != 1689278011U || rva >= image_size) return nullptr;
return base + rva;
}
inline ResolveRva resolver = default_resolver;
#else
inline ResolveRva resolver = nullptr;
#endif
inline void set_resolver(ResolveRva value) noexcept { resolver = value; }
[[nodiscard]] inline const void* require(std::uint32_t rva) {
if (resolver == nullptr) std::abort();
const void* address = resolver(rva);
if (address == nullptr) std::abort();
return address;
}

} // namespace windissect_calls
