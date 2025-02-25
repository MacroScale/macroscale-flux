#include "application_data.h"

AppData& AppData::Instance(){
    static AppData inst;
    if (inst.hasInit == false){
        inst.hasInit = true;
    }
    return inst; 
};


std::pair<HWND, std::string> AppData::GetCurrentGameWin(){
    return currentGameWin;
}

void AppData::SetGameWin(HWND hwnd, std::string title){
    currentGameWin = std::pair<HWND, std::string>(hwnd, title);
}


