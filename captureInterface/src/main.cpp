#include <libloaderapi.h>
#include <minwindef.h>
#include <sysinfoapi.h>
#include <thread>
#include "event_loop.h"
#include "logger.h"
#include "task_handler.h"
#include "tasks.h"
#include "capturer.h"


void initNvfbc(){
}

typedef LONG(WINAPI *RTLGETVERSION)(PRTL_OSVERSIONINFOW);

void CheckWinVer() {
    RTLGETVERSION pRtlGetVersion = (RTLGETVERSION)GetProcAddress(GetModuleHandleA("ntdll.dll"), "RtlGetVersion");

    if (pRtlGetVersion) {
        RTL_OSVERSIONINFOW osvi = {0};
        osvi.dwOSVersionInfoSize = sizeof(RTL_OSVERSIONINFOW);

        if (pRtlGetVersion(&osvi) == 0) {
            std::cout << "Windows version: " << osvi.dwMajorVersion << "." << osvi.dwMinorVersion << " (" << osvi.dwBuildNumber << ")" << std::endl;
        } else {
            std::cout << "Failed to get Windows version using RtlGetVersion" << std::endl;
        }
    } else {
        std::cout << "Failed to get RtlGetVersion function" << std::endl;
    }
}

int main() {

    std::cout << std::unitbuf;
    SLOG.info("starting program");

    CheckWinVer();

    /*bool capturerStatus = CAPTURER.Init();*/
    /*if (capturerStatus) {*/
    /*    SLOG.error("capture is not supported on this device!");*/
    /*    return 0;*/
    /*} else {*/
    /*    SLOG.info("capture is supported!");*/
    /*}*/
    /**/

    EventLoop* eventLoopInst = EventLoop::Instance();
    TaskHandler* taskHandlerInst = TaskHandler::Instance();

    // start core systems 
    std::thread event_thread(&EventLoop::Start, eventLoopInst);
    std::thread task_thread(&TaskHandler::Start, taskHandlerInst);

    // create initial tasks
    std::unique_ptr<Task> poll_hotkeys_task = std::make_unique<Tasks::PollHotkeys>();
    std::unique_ptr<Task> poll_fgwin_task = std::make_unique<Tasks::PollFGWin>();

    // add tasks to task handler
    taskHandlerInst->AddTask(std::move(poll_hotkeys_task));
    taskHandlerInst->AddTask(std::move(poll_fgwin_task));

    event_thread.join();
    task_thread.join();

    SLOG.info("exiting program");
    return 0;
}
