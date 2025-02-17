#ifndef TASK_HANDLER_H
#define TASK_HANDLER_H

#include <atomic>
#include <tasks.h>
#include <mutex>
#include <thread>
#include <vector>
#include <map>

class TaskHandler {

public:
    static TaskHandler* Instance();
    void Start();
    void End();
    void AddTask(Task& t);

private:
    // Static pointer to the Singleton instance
    static TaskHandler* instancePtr;
    static std::mutex instMutex;

    struct TaskThreadHandle {
        std::thread tThread; 
        std::atomic<bool>* complete;
    };

    bool running;
    std::vector<Task*> tasksBuf;
    std::vector<TaskThreadHandle> taskHandles;
    std::map<std::thread::id, std::string> threadNames;
    std::mutex tasksBufMutex;
    std::mutex taskHandlesMutex;
    std::mutex threadNamesMutex;

    TaskHandler(){};

    void runTask(Task* t);
    void cleanHandles();
    
    // deleting the copy constructor to prevent copies
    TaskHandler(const TaskHandler& obj) = delete;
    void operator=(TaskHandler const&) = delete;
};

#endif
