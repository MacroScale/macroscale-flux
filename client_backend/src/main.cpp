#include <thread>
#include "NvFBC.h"
#include "event_loop.h"
#include "logger.h"
#include "task_handler.h"
#include "tasks.h"

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
    Tasks::RegisterHotKeys hotkey_task;

    // add tasks to task handler
    task_handler_inst->AddTask(hotkey_task);

    event_thread.join();
    task_thread.join();

    SLOG.info("exiting program");
    return 0;
}
