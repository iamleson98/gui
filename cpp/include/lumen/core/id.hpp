#pragma once
#include <cstdint>
#include <string>
#include <functional>
#include <atomic>
namespace lumen {
struct Id {
    uint64_t value = 0;
    static const Id NULL_ID;
    constexpr Id() = default;
    constexpr explicit Id(uint64_t v) : value(v) {}
    static Id from_str(const std::string& s);
    Id derive(const std::string& salt) const;
    Id derive_index(size_t idx) const;
    static Id unique();
    bool operator==(const Id& o) const { return value==o.value; }
    bool operator!=(const Id& o) const { return value!=o.value; }
    bool operator<(const Id& o) const { return value<o.value; }
};
inline const Id Id::NULL_ID(0);
} // namespace lumen
namespace std { template<> struct hash<lumen::Id> { size_t operator()(const lumen::Id& id) const noexcept { return hash<uint64_t>{}(id.value); } }; }
