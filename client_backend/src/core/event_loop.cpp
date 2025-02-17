#include "event_loop.h"
#include "logger.h"
#include <mutex>
#include <thread>
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
    
    while(this->running){

        { 
            std::lock_guard<std::mutex> lock(this->eventBufMutex);
            if (!this->eventBuf.empty()) {
                for (Event &e: this->eventBuf){
                    cout << "processing event: " << e.GetEventType() << endl;
                } 
                this->eventBuf.clear();
            }
        }

        std::this_thread::sleep_for(std::chrono::milliseconds(50));
    }

    SLOG.info("event loop exiting");
}

void EventLoop::AddEvent(Event& e) {

    std::ostringstream oss;
    oss << "event loop: event push: " << e.GetEventType();
    SLOG.info(oss.str());

    std::lock_guard<std::mutex> lock(this->eventBufMutex);
    this->eventBuf.push_back(e);
}

void EventLoop::End(){ this->running = false; }
