#include "application_data.h"
#include "event_loop.h"
#include "logger.h"
#include "tasks.h"
#include "utils.h"

#include <thread>
#include <unordered_map>
#include <windows.h>

Tasks::PollFGWin::PollFGWin() { 
    this->SetName("PollFGWin"); 
}

void Tasks::PollFGWin::Execute(){
    this->SetRunning(true);

    EventLoop* evInst = EventLoop::Instance(); 

    while(this->GetRunning()){
        std::unordered_map<HWND, std::string> fgWins = Utils::GetFgWins();

        // skip processing if current game window hwnd is alive 
        if (IsWindow(APPDATA.GetCurrentGameWin().first)) {
            std::this_thread::sleep_for(std::chrono::milliseconds(50));
            continue;
        }
        
        // TODO: move into util for checking if hwnd is game
        for (auto &win: fgWins){
            std::string filePath = Utils::filepathHWND(win.first);
            std::size_t found = filePath.find("steamapps");

            if (found != std::string::npos){
                std::ostringstream oss;
                oss << "game window detected: " << win.second << " path: " << filePath;
                SLOG.info(oss.str());
                APPDATA.SetGameWin(win.first, win.second);
            }

        }

        std::this_thread::sleep_for(std::chrono::milliseconds(50));
    }

    this->SetRunning(false);
}
