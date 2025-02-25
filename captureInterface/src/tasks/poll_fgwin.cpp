#include "application_data.h"
#include "event_loop.h"
#include "tasks.h"

#include <thread>
#include <unordered_map>
#include <windows.h>

Tasks::PollFGWin::PollFGWin() { 
    this->SetName("PollFGWin"); 
}


BOOL CALLBACK EnumWindowsProc(HWND hwnd, LPARAM lParam) {
	char class_name[80];
	char title[80];
	GetClassName(hwnd,class_name, sizeof(class_name));
	GetWindowText(hwnd,title,sizeof(title));

    std::unordered_map<HWND, std::string>* titles = 
        reinterpret_cast<std::unordered_map<HWND, std::string>*>(lParam);

    titles->insert({hwnd, title});

	return TRUE;
}

void Tasks::PollFGWin::Execute(){
    this->SetRunning(true);

    EventLoop* evInst = EventLoop::Instance(); 

    std::unordered_map<HWND, std::string> fgWins;

    while(this->GetRunning()){
        EnumWindows(EnumWindowsProc, reinterpret_cast<LPARAM>(&fgWins));

        if (APPDATA.GetFGWins() != fgWins) APPDATA.SetFGWins(std::move(fgWins));

        fgWins.clear();
        std::this_thread::sleep_for(std::chrono::milliseconds(50));
    }

    this->SetRunning(false);
}
