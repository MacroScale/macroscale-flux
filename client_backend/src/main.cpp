#include <thread>
#include "NvFBC.h"
#include "event_loop.h"
#include "logger.h"
#include "tasks.h"

void init_Nvfbc(){
}


int main() {
    SLOG.info("starting program");

    EventLoop *event_loop_inst = EventLoop::instance();

    Tasks::RegisterHotKeys hotkey_task("Register HotKeys Task");

    // start program
    std::thread event_thread(&EventLoop::start, event_loop_inst);
    event_thread.join();

    SLOG.info("exiting program");
    return 0;
}
