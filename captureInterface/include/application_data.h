#ifndef APPLICATION_DATA_H
#define APPLICATION_DATA_H

#include <mutex>
#include <vector>

class AppData {

public:
    static AppData& Instance();
    void Init();

private:
    // Static pointer to the Singleton instance
    static AppData* instancePtr;
    static std::mutex instMutex;

    bool hasInit = false;

    AppData(){};
    
    // deleting the copy constructor to prevent copies
    AppData(const AppData& obj) = delete;
    void operator=(AppData const&) = delete;
};

static AppData& APPDATA = AppData::Instance();

#endif
