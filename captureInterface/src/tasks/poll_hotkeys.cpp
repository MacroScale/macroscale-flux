#include "event_data.h"
#include "event_loop.h"
#include "logger.h"
#include "tasks.h"

#include <sstream>
#include <strstream>
#include <thread>
#include <windows.h>

Tasks::PollHotkeys::PollHotkeys() { 
    this->SetName("PollHotKeys"); 
}

void Tasks::PollHotkeys::Execute(){
    this->SetRunning(true);

    // register hotkeys
    bool status;

    status = RegisterHotKey(NULL, 1, MOD_ALT, 'Q'); 
    {
        std::ostringstream oss;
        oss << "registered quit hotkey: ALT + Q: " << status;
        SLOG.info(oss.str());
    }

    status = RegisterHotKey(NULL, 2, MOD_ALT, 'R'); 
    {
        std::ostringstream oss;
        oss << "registered start capture hotkey: ALT + R: " << status;
        SLOG.info(oss.str());
    }

    status = RegisterHotKey(NULL, 3, MOD_ALT, 'E'); 
    {
        std::ostringstream oss;
        oss << "registered stop capture hotkey: ALT + E: " << status;
        SLOG.info(oss.str());
    }

    status = RegisterHotKey(NULL, 4, MOD_ALT, 'W'); 
    {
        std::ostringstream oss;
        oss << "registered log processes hotkey: ALT + W: " << status;
        SLOG.info(oss.str());
    }

    EventLoop* evInst = EventLoop::Instance(); 

    while(this->GetRunning()){
        MSG msg = {0};
        while (GetMessage(&msg, NULL, 0, 0) != 0) {
            if (msg.message == WM_HOTKEY) {
                EventData data; 
                data.hotkeyData = HotKeyData {
                    .id = static_cast<int>(msg.wParam),
                    .vks = static_cast<int>(msg.lParam)
                };
                Event e(EventType::HOTKEY, data);
                evInst->AddEvent(e);
            }
        } 
        std::this_thread::sleep_for(std::chrono::milliseconds(50));
    }

    this->SetRunning(false);
}
