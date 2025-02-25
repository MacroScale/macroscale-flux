#ifndef TASK_HANDLER_H
#define TASK_HANDLER_H

#include <atomic>
#include <memory>
#include <queue>
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
    // use move on this function
    void AddTask(std::unique_ptr<Task> t);

private:
    // Static pointer to the Singleton instance
    static TaskHandler* instancePtr;
    static std::mutex instMutex;

    struct TaskThreadHandle {
        std::thread tThread; 
        std::shared_ptr<std::atomic<bool>> complete;
        std::string tTitle;
    };

    bool running;
    std::queue<std::unique_ptr<Task>> tasksBuf;
    std::vector<std::unique_ptr<TaskThreadHandle>> taskHandles;
    std::mutex tasksBufMutex;
    std::mutex taskHandlesMutex;
    std::mutex threadNamesMutex;

    TaskHandler(){};

    void runTask(std::unique_ptr<Task> t);
    void cleanHandles();
    
    // deleting the copy constructor to prevent copies
    TaskHandler(const TaskHandler& obj) = delete;
    void operator=(TaskHandler const&) = delete;
};

#endif
