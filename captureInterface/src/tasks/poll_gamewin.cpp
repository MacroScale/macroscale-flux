#include "application_data.h"
#include "event_loop.h"
#include "tasks.h"
#include "utils.h"

#include <thread>
#include <windows.h>

Tasks::PollGameWin::PollGameWin() { 
    this->SetName("PollGameWin"); 
}

void Tasks::PollGameWin::Execute(){
    this->SetRunning(true);

    EventLoop* evInst = EventLoop::Instance(); 

    while(this->GetRunning()){

        // skip processing if current game window hwnd is alive 
        if (IsWindow(APPDATA.GetCurrentGWin().first)) continue;
        
        for (auto &win: APPDATA.GetFGWins()){
            std::string filePath = Utils::filepathHWND(win.first);

            std::size_t found = filePath.find("steamapps");
            if (found != std::string::npos){
                // set current game window
            }

        }

        std::this_thread::sleep_for(std::chrono::milliseconds(50));
    }

    this->SetRunning(false);
}
