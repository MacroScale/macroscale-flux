
#include <string>

class Task {
private:
    std::string name;
public: 
    Task(std::string n): name(n) {};
    virtual void execute() = 0;
};

namespace Tasks {
    class RegisterHotKeys : public Task { using Task::Task; void execute() override;}; 
}
