#include "event_loop.h"
#include "logger.h"
#include <mutex>
#include <thread>
#include <vector>

EventLoop *EventLoop::instancePtr = NULL; 
std::mutex EventLoop::instMutex; 

EventLoop* EventLoop::Instance() {
    if (instancePtr == nullptr) {
        std::lock_guard<std::mutex> lock(instMutex);
        instancePtr = new EventLoop();
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
                    this->ProcessEvent(e);
                } 
                this->eventBuf.clear();
            }
        }

        std::this_thread::sleep_for(std::chrono::milliseconds(50));
    }

    SLOG.info("event loop exiting");
}

void EventLoop::ProcessEvent(Event& e) {

    // TODO: seperate this logic in another file
    if (e.GetEventType() == EventType::HOTKEY) {
        EventData ed = e.GetEventData();
        int id = ed.hotkeyData.id;
        if (id == 1) { 
            ostringstream oss;
            oss << "eventloop: quit application";
            SLOG.info(oss.str());
            // TODO: need to shutdown and clean thread handles for 
            // all systems gracefully
            exit(1); 
        } else if (id == 2) { 
            ostringstream oss;
            oss << "eventloop: start capture";
            SLOG.info(oss.str());
        } else if (id == 3) { 
            ostringstream oss;
            oss << "eventloop: stop capture";
            SLOG.info(oss.str());
        } else if (id == 4) { 
            ostringstream oss;
            oss << "eventloop: log processes";
            SLOG.info(oss.str());
        } else {
            ostringstream oss;
            oss << "unhandled hotkey id: " << id;
            SLOG.info(oss.str());
        }
    }
    else {
        SLOG.info("unable to process event");
    }
}

void EventLoop::AddEvent(Event& e) {

    std::ostringstream oss;
    oss << "event loop: event push: " << e.GetEventTypeStr();
    SLOG.info(oss.str());

    std::lock_guard<std::mutex> lock(this->eventBufMutex);
    this->eventBuf.push_back(e);
}

void EventLoop::End(){ this->running = false; }
