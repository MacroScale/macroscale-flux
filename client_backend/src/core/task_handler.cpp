#include "task_handler.h"
#include "logger.h"
#include <cstdio>
#include <mutex>
#include <vector>
#include <map>

TaskHandler *TaskHandler::instancePtr = NULL; 
std::mutex TaskHandler::instMutex; 

TaskHandler* TaskHandler::Instance() {
    if (instancePtr == nullptr) {
        if (instancePtr == nullptr) {
            std::lock_guard<std::mutex> lock(instMutex);
            instancePtr = new TaskHandler();
        }
    }
    return instancePtr;
}

void TaskHandler::Start(){
    this->running = true;
    SLOG.info("task handler loop started");

    std::vector<Task*> locTaskBatch; 
    while(this->running){
        locTaskBatch.clear();

        { 
            std::lock_guard<std::mutex> lock(this->tasksBufMutex);
            if (!this->tasksBuf.empty()) {
                locTaskBatch = std::move(tasksBuf);
                tasksBuf.clear();
            }
        }

        // begin execution of tasks
        for (Task* t: locTaskBatch){
            this->runTask(t);
        }

        // clear handles of completed tasks
        this->cleanHandles();
    }

    SLOG.info("task handler exiting");
}

void TaskHandler::runTask(Task* t){
    std::ostringstream oss;
    oss << "task handler: executing " << t->GetName();
    SLOG.info(oss.str());
    
    std::thread tThread(&Task::Execute, t);

    {
        lock_guard<std::mutex> lock(taskHandlesMutex);
        lock_guard<std::mutex> lock2(threadNamesMutex);

        threadNames[tThread.get_id()] = t->GetName();
        taskHandles.push_back(std::move(tThread));
    }
}

void TaskHandler::cleanHandles() {
    std::lock_guard<std::mutex> lock(taskHandlesMutex);
    for (auto it = taskHandles.begin(); it != taskHandles.end(); ){
        if (it->joinable()) {
            std::ostringstream oss;
            oss << "task handler: completed " << threadNames[it->get_id()];
            SLOG.info(oss.str());

            it->join(); 
            it = taskHandles.erase(it);
        } else { ++it; }
    }
}


void TaskHandler::AddTask(Task& t){
    std::ostringstream oss;
    oss << "adding task: " << t.GetName();
    SLOG.info(oss.str());
    std::lock_guard<std::mutex> lock(this->tasksBufMutex);
    this->tasksBuf.push_back(&t);
}

void TaskHandler::End(){ this->running = false; }
