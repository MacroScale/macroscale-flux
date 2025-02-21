
#ifndef TASKS_H
#define TASKS_H

#include <string>

class Task {
private:
    std::string name;

public: 
    Task() {};
    std::string GetName() { return name; };
    void SetName(std::string name) { this->name = name; };
    virtual void Execute() = 0;
    void SetRunning(bool running) { this->running = running; };
    bool GetRunning() { return this->running; };
private:
    int running;
};

namespace Tasks {

    // oneshot
    //... 
    // Polls 
    class PollHotkeys: public Task {
        public: 
            PollHotkeys(); 
            void Execute() override; 
    }; 

}

#endif
