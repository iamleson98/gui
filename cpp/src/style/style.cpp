#include "lumen/style/style.hpp"
#include <sstream>
namespace lumen {
Style& Style::apply(const std::string& classes) {
    std::istringstream ss(classes); std::string tok;
    while(ss >> tok) {
        if(tok=="p-4") p(16); else if(tok=="px-4") px(16); else if(tok=="py-2") py(8);
        else if(tok=="rounded-md") rounded_md(); else if(tok=="bg-white") bg_white(); else if(tok=="bg-primary") bg_primary();
        else if(tok=="text-white") text_white(); else if(tok=="text-sm") text_sm(); else if(tok=="font-bold") font_bold();
        else if(tok=="flex") flex(); else if(tok=="flex-col") flex_col(); else if(tok=="items-center") items_center();
        else if(tok=="justify-center") justify_center(); else if(tok=="text-center") text_center();
        else if(tok=="w-full") w_full(); else if(tok=="h-full") h_full();
        else if(tok.rfind("p-",0)==0) { try{p(std::stof(tok.substr(2)));}catch(...){} }
        else if(tok.rfind("w-",0)==0) { try{w(std::stof(tok.substr(2)));}catch(...){} }
        else if(tok.rfind("h-",0)==0) { try{h(std::stof(tok.substr(2)));}catch(...){} }
        else if(tok.rfind("gap-",0)==0) { try{gap(std::stof(tok.substr(4)));}catch(...){} }
        else if(tok.rfind("rounded-",0)==0) { try{rounded(std::stof(tok.substr(8)));}catch(...){} }
        else if(tok.rfind("grid-cols-",0)==0) { try{grid_cols(std::stoul(tok.substr(10)));}catch(...){} }
    }
    return *this;
}
Theme Theme::light() { Theme t; t.palette.bg=Color::TW_SLATE_100; t.palette.surface=Color::WHITE; t.palette.primary=Color::TW_INDIGO_500; t.palette.accent=Color::TW_EMERALD_500; t.palette.text=Color::TW_SLATE_900; t.palette.text_muted=Color(100,116,139); t.palette.border=Color(226,232,240); t.palette.danger=Color::TW_ROSE_500; t.palette.success=Color::TW_EMERALD_500; t.palette.warning=Color::TW_AMBER_500; t.palette.info=Color::TW_INDIGO_500; return t; }
Theme Theme::dark() { Theme t; t.kind=Theme::Kind::Dark; t.palette.bg=Color(9,11,16); t.palette.surface=Color(20,24,32); t.palette.primary=Color::TW_INDIGO_500; t.palette.accent=Color::TW_EMERALD_500; t.palette.text=Color(241,245,249); t.palette.text_muted=Color(148,163,184); t.palette.border=Color(45,53,67); t.palette.danger=Color::TW_ROSE_500; t.palette.success=Color::TW_EMERALD_500; t.palette.warning=Color::TW_AMBER_500; t.palette.info=Color::TW_INDIGO_500; return t; }
} // namespace lumen
