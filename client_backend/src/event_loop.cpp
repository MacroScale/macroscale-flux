#include "event_loop.h"
#include "logger.h"
#include <mutex>
#include <vector>

EventLoop *EventLoop::instancePtr = NULL; 
std::mutex EventLoop::instMutex; 

EventLoop* EventLoop::Instance() {
    if (instancePtr == nullptr) {
        if (instancePtr == nullptr) {
            std::lock_guard<std::mutex> lock(instMutex);
            instancePtr = new EventLoop();
        }
    }
    return instancePtr;
}

void EventLoop::Start(){
    this->running = true;
    SLOG.info("event loop started");
    
    std::vector<Event> locEventBatch; 
    while(this->running){
        { 
            std::lock_guard<std::mutex> lock(this->eventBufMutex);
            if (!this->eventBuf.empty()) {
                locEventBatch = std::move(this->eventBuf);
                this->eventBuf.clear();
            }
        }

        for (Event e: locEventBatch){
            // process Event 
        }
    }

    SLOG.info("event loop exiting");
}

void EventLoop::AddEvent(Event e){
    SLOG.info("adding event: TODO");
    std::lock_guard<std::mutex> lock(this->eventBufMutex);
    this->eventBuf.push_back(e);
}

void EventLoop::End(){ this->running = false; }
