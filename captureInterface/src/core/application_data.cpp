#include "application_data.h"

AppData& AppData::Instance(){
    static AppData inst;
    if (inst.hasInit == false){
        inst.hasInit = true;
    }
    return inst; 
};


std::unordered_map<HWND, std::string>& AppData::GetFGWins(){
    return APPDATA.fgWins;
};

void AppData::SetFGWins(std::unordered_map<HWND, std::string>&& wins){
    APPDATA.fgWins = std::move(wins);
};


std::pair<HWND, std::string> AppData::GetCurrentGWin(){
    return APPDATA.currentGameWin;
}

