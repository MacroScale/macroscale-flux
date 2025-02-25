#ifndef APPLICATION_DATA_H
#define APPLICATION_DATA_H

#include <mutex>
#include <string>
#include <unordered_map>
#include <windef.h>

class AppData {

public:
    static AppData& Instance();
    std::unordered_map<HWND, std::string>& GetFGWins();
    void SetFGWins(std::unordered_map<HWND, std::string>&& wins);
    std::pair<HWND, std::string> GetCurrentGWin();

private:
    // Static pointer to the Singleton instance
    static AppData* instancePtr;
    static std::mutex instMutex;

    std::unordered_map<HWND, std::string> fgWins;
    std::pair<HWND, std::string> currentGameWin;

    bool hasInit = false;

    AppData(){};
    
    // deleting the copy constructor to prevent copies
    AppData(const AppData& obj) = delete;
    void operator=(AppData const&) = delete;
};

static AppData& APPDATA = AppData::Instance();

#endif
