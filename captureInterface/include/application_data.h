#ifndef APPLICATION_DATA_H
#define APPLICATION_DATA_H

#include <mutex>
#include <string>
#include <windef.h>

class AppData {

public:
    static AppData& Instance();
    void SetGameWin(HWND hwnd, std::string title);
    std::pair<HWND, std::string> GetCurrentGameWin();

private:
    // Static pointer to the Singleton instance
    static AppData* instancePtr;
    static std::mutex instMutex;

    std::pair<HWND, std::string> currentGameWin;

    bool hasInit = false;

    AppData(){};
    
    // deleting the copy constructor to prevent copies
    AppData(const AppData& obj) = delete;
    void operator=(AppData const&) = delete;
};

static AppData& APPDATA = AppData::Instance();

#endif
