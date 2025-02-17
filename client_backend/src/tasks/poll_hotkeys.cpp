#include "event_data.h"
#include "event_loop.h"
#include "logger.h"
#include "tasks.h"

#include <thread>
#include <windows.h>

Tasks::PollHotkeys::PollHotkeys() { 
    this->SetName("PollHotKeys"); 
}

void Tasks::PollHotkeys::Execute(){
    this->SetRunning(true);

    // register hotkeys
    std::ostringstream oss;

    bool status = RegisterHotKey(NULL, 1, MOD_ALT, 'Q'); 
    oss << "registered quit hotkey: ALT + Q: " << status;
    SLOG.info(oss.str());
    oss.clear();


    EventLoop* evInst = EventLoop::Instance(); 

    while(this->GetRunning()){
        MSG msg = {0};
        while (GetMessage(&msg, NULL, 0, 0) != 0) {
            if (msg.message == WM_HOTKEY) {
                EventData data; 
                data.hotkeyData = HotKeyData {
                    .modfn = static_cast<int>(msg.wParam),
                    .vk = static_cast<int>(msg.lParam)
                };
                Event e(EventType::HOTKEY, data);
                cout << "event type: " << e.GetEventType() << endl;
                evInst->AddEvent(e);
            }
        } 
        std::this_thread::sleep_for(std::chrono::milliseconds(50));
    }

    this->SetRunning(false);
}

