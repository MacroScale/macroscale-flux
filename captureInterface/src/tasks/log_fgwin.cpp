#include "application_data.h"
#include "event_loop.h"
#include "logger.h"
#include "tasks.h"

#include <string>
#include <unordered_map>
#include <windows.h>

Tasks::LogFGWins::LogFGWins() { 
    this->SetName("LogFGWins"); 
}

void Tasks::LogFGWins::Execute(){
    this->SetRunning(true); 

    std::unordered_map<HWND, std::string> fgWins = APPDATA.GetFGWins();
    std::string titles = "[";

    for (auto &el: fgWins){
        titles += "{ " + el.second + " }, ";
    }

    titles += "]";

    SLOG.info(titles);

    this->SetRunning(false); 
}
