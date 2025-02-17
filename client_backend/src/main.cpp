#include <thread>
#include "NvFBC.h"
#include "event_loop.h"
#include "logger.h"
#include "task_handler.h"
#include "tasks.h"
#include <windows.h>

void initNvfbc(){
}

int main() {
    SLOG.info("starting program");

    EventLoop *event_loop_inst = EventLoop::Instance();
    TaskHandler *task_handler_inst = TaskHandler::Instance();

    // start core systems 
    std::thread event_thread(&EventLoop::Start, event_loop_inst);
    std::thread task_thread(&TaskHandler::Start, task_handler_inst);

    // create initial tasks
    Tasks::PollHotkeys poll_hotkeys_task;

    // add tasks to task handler
    task_handler_inst->AddTask(poll_hotkeys_task);

    event_thread.join();
    task_thread.join();

    SLOG.info("exiting program");
    return 0;
}
