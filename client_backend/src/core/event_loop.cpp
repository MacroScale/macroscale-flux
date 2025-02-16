#include "event_loop.h"
#include "logger.h"
#include <mutex>
#include <vector>

EventLoop *EventLoop::instancePtr = NULL; 
std::mutex EventLoop::instMutex; 

EventLoop* EventLoop::instance() {
    if (instancePtr == nullptr) {
        if (instancePtr == nullptr) {
            std::lock_guard<std::mutex> lock(instMutex);
            instancePtr = new EventLoop();
        }
    }
    return instancePtr;
}

void EventLoop::start(){
    SLOG.info("event loop started");

    std::vector<Event> locEventBatch; 
    while(running){
        { 
            std::lock_guard<std::mutex> lock(eventBufMutex);
            if (!eventBuf.empty()) {
                locEventBatch = std::move(eventBuf);
                eventBuf.clear();
            }
        }

        for (Event e: locEventBatch){
            // process Event 
        }
    }

    SLOG.info("event loop exiting");
}

void EventLoop::add_event(Event e){
    SLOG.info("adding event: TODO");
    std::lock_guard<std::mutex> lock(eventBufMutex);
    eventBuf.push_back(e);
}

void EventLoop::end(){ running = false; }
