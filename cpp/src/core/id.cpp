#include "lumen/core/id.hpp"
#include <sstream>
#include <iomanip>
#include <cstring>
namespace lumen {
static uint64_t fnv1a(const char* d, size_t n) { uint64_t h=0xcbf29ce484222325ULL; for(size_t i=0;i<n;++i) { h^=(uint8_t)d[i]; h*=0x100000001b3ULL; } return h; }
static uint64_t fnv1a_combine(uint64_t seed, const char* d, size_t n) { uint64_t h=seed; for(size_t i=0;i<n;++i) { h^=(uint8_t)d[i]; h*=0x100000001b3ULL; } return h; }
Id Id::from_str(const std::string& s) { return Id(fnv1a(s.data(), s.size()) | 1); }
Id Id::derive(const std::string& salt) const { return Id(fnv1a_combine(value, salt.data(), salt.size())); }
Id Id::derive_index(size_t idx) const { return Id(fnv1a_combine(value, (const char*)&idx, sizeof(idx))); }
static std::atomic<uint64_t> counter{1ULL<<63};
Id Id::unique() { return Id(counter.fetch_add(1, std::memory_order_relaxed)); }
} // namespace lumen
