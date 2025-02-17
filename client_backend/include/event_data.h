#ifndef EVENT_DATA_H
#define EVENT_DATA_H

#include <iostream>
#include <mutex>
#include <string>

enum EventType {
    HOTKEY
};

struct HotKeyData {
    int modfn;
    int vk;
};

union EventData{
    HotKeyData hotkeyData;
};

class Event {
public:
    Event(EventType type, EventData data): eventType(type), eventData(data){};
    std::string GetEventType(){ 
        switch(this->eventType){
            case HOTKEY: 
                return "Hotkey";
                break;
            default: 
                return "no event type";
                break;
        }
    };
private:
    EventType eventType;
    EventData eventData;
};

#endif
