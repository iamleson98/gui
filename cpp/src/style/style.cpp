#include "lumen/style/style.hpp"
#include <sstream>
namespace lumen {
Style& Style::apply(const std::string& classes) {
    std::istringstream ss(classes); std::string tok;
    while(ss >> tok) {
        if(tok=="p-4") p(16); else if(tok=="p-6") p_6(); else if(tok=="p-8") p_8();
        else if(tok=="px-4") px(16); else if(tok=="px-6") px_6();
        else if(tok=="py-2") py(8); else if(tok=="py-3") py_3(); else if(tok=="py-4") py_4();
        else if(tok=="rounded-md") rounded_md(); else if(tok=="rounded-lg") rounded_lg(); else if(tok=="rounded-xl") rounded_xl();
        else if(tok=="bg-white") bg_white(); else if(tok=="bg-primary") bg_primary(); else if(tok=="bg-surface") bg_surface(); else if(tok=="bg-muted") bg_muted();
        else if(tok=="text-white") text_white(); else if(tok=="text-muted") text_muted();
        else if(tok=="text-xs") text_xs(); else if(tok=="text-sm") text_sm(); else if(tok=="text-base") text_base(); else if(tok=="text-lg") text_lg(); else if(tok=="text-xl") text_xl(); else if(tok=="text-2xl") text_2xl();
        else if(tok=="font-bold") font_bold(); else if(tok=="font-medium") font_medium();
        else if(tok=="flex") flex(); else if(tok=="flex-col") flex_col();
        else if(tok=="items-center") items_center(); else if(tok=="items-start") items_start(); else if(tok=="items-end") items_end(); else if(tok=="items-stretch") items_stretch();
        else if(tok=="justify-center") justify_center(); else if(tok=="justify-start") justify_start(); else if(tok=="justify-end") justify_end(); else if(tok=="justify-between") justify_between();
        else if(tok=="gap-2") gap_2(); else if(tok=="gap-4") gap_4(); else if(tok=="gap-6") gap_6(); else if(tok=="gap-8") gap_8();
        else if(tok=="text-center") text_center(); else if(tok=="text-left") text_left(); else if(tok=="text-right") text_right();
        else if(tok=="w-full") w_full(); else if(tok=="h-full") h_full();
        else if(tok=="cursor-pointer") cursor_pointer();
        else if(tok.rfind("p-",0)==0) { try{p(std::stof(tok.substr(2)));}catch(...){} }
        else if(tok.rfind("w-",0)==0) { try{w(std::stof(tok.substr(2)));}catch(...){} }
        else if(tok.rfind("h-",0)==0) { try{h(std::stof(tok.substr(2)));}catch(...){} }
        else if(tok.rfind("gap-",0)==0) { try{gap(std::stof(tok.substr(4)));}catch(...){} }
        else if(tok.rfind("rounded-",0)==0) { try{rounded(std::stof(tok.substr(8)));}catch(...){} }
        else if(tok.rfind("grid-cols-",0)==0) { try{grid_cols(std::stoul(tok.substr(10)));}catch(...){} }
    }
    return *this;
}

Theme Theme::light() {
    Theme t;
    t.kind = Kind::Light;
    t.palette.bg = Color::TW_SLATE_100;
    t.palette.surface = Color::WHITE;
    t.palette.surface_elevated = Color::WHITE;
    t.palette.muted = Color::TW_SLATE_50;
    t.palette.primary = Color::TW_INDIGO_600;
    t.palette.primary_hover = Color::TW_INDIGO_500;
    t.palette.primary_active = Color::TW_INDIGO_700;
    t.palette.accent = Color::TW_EMERALD_500;
    t.palette.text = Color::TW_SLATE_900;
    t.palette.text_muted = Color::TW_SLATE_500;
    t.palette.border = Color::TW_SLATE_200;
    t.palette.shadow = Color(15, 23, 42, 40);
    t.palette.danger = Color::TW_ROSE_500;
    t.palette.success = Color::TW_EMERALD_500;
    t.palette.warning = Color::TW_AMBER_500;
    t.palette.info = Color::TW_INDIGO_500;
    return t;
}

Theme Theme::dark() {
    Theme t;
    t.kind = Kind::Dark;
    t.palette.bg = Color::TW_SLATE_900;
    t.palette.surface = Color::TW_SLATE_800;
    t.palette.surface_elevated = Color::TW_SLATE_700;
    t.palette.muted = Color::TW_SLATE_800;
    t.palette.primary = Color::TW_INDIGO_500;
    t.palette.primary_hover = Color::TW_INDIGO_400;
    t.palette.primary_active = Color::TW_INDIGO_600;
    t.palette.accent = Color::TW_EMERALD_400;
    t.palette.text = Color::TW_SLATE_50;
    t.palette.text_muted = Color::TW_SLATE_400;
    t.palette.border = Color::TW_SLATE_700;
    t.palette.shadow = Color(0, 0, 0, 120);
    t.palette.danger = Color::TW_ROSE_500;
    t.palette.success = Color::TW_EMERALD_500;
    t.palette.warning = Color::TW_AMBER_500;
    t.palette.info = Color::TW_INDIGO_500;
    return t;
}
} // namespace lumen
