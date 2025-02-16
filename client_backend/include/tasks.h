
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
};

namespace Tasks {

    class RegisterHotKeys : public Task { public: RegisterHotKeys(); void Execute() override; }; 
    
}

#endif
