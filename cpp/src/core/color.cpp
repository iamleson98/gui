#include "lumen/core/color.hpp"
namespace lumen {
static int hex_digit(char c) { if(c>='0'&&c<='9') return c-'0'; if(c>='a'&&c<='f') return c-'a'+10; if(c>='A'&&c<='F') return c-'A'+10; return -1; }
std::optional<Color> Color::from_hex(const std::string& hex) {
    if(hex.size()<7||hex[0]!='#') return std::nullopt;
    auto hp=[&](size_t i)->std::optional<uint8_t>{ if(i+1>=hex.size()) return std::nullopt; int hi=hex_digit(hex[i]), lo=hex_digit(hex[i+1]); if(hi<0||lo<0) return std::nullopt; return (uint8_t)((hi<<4)|lo); };
    auto r=hp(1), g=hp(3), b=hp(5); if(!r||!g||!b) return std::nullopt;
    if(hex.size()==7) return Color::rgb(*r,*g,*b);
    if(hex.size()==9) { auto a=hp(7); if(!a) return std::nullopt; return Color::rgba(*r,*g,*b,*a); }
    return std::nullopt;
}
} // namespace lumen
