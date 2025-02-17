#ifndef EVENT_DATA_H
#define EVENT_DATA_H

#include <iostream>
#include <mutex>
#include <string>

enum EventType {
    HOTKEY
};

struct HotKeyData {
    int id;
    int vks;
};

union EventData{
    HotKeyData hotkeyData;
};

class Event {
public:
    Event(EventType type, EventData data): eventType(type), eventData(data){};
    EventType GetEventType(){ return this->eventType; };
    EventData GetEventData(){ return this->eventData; };
    std::string GetEventTypeStr(){ 
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
