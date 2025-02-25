#include "task_handler.h"
#include "logger.h"
#include <atomic>
#include <mutex>
#include <vector>
#include <thread>

TaskHandler *TaskHandler::instancePtr = NULL; 
std::mutex TaskHandler::instMutex; 

TaskHandler* TaskHandler::Instance() {
    if (instancePtr == nullptr) {
        std::lock_guard<std::mutex> lock(instMutex);
        instancePtr = new TaskHandler();
    }
    return instancePtr;
}

void TaskHandler::Start(){
    this->running = true;
    SLOG.info("task handler loop started");

    while(this->running){

        { 
            std::lock_guard<std::mutex> lock(this->tasksBufMutex);
            // begin execution of tasks
            while (!tasksBuf.empty())
            {
                std::unique_ptr<Task> task = std::move(tasksBuf.front());
                tasksBuf.pop(); 
                this->runTask(std::move(task)); 
            } 
        }

        // clear handles of completed tasks
        this->cleanHandles();

        std::this_thread::sleep_for(std::chrono::milliseconds(50));
    }

    SLOG.info("task handler exiting");
}

void TaskHandler::runTask(std::unique_ptr<Task> t) {

    std::ostringstream oss;
    oss << "task handler: executing " << t->GetName();
    SLOG.info(oss.str());

    auto handle = std::make_unique<TaskThreadHandle>();
    handle->tTitle = t->GetName();
    handle->complete = std::make_shared<std::atomic<bool>>(false);

    std::thread tThread([task = std::move(t), complete = handle->complete]() {
        task->Execute();
        complete->store(true); 
    });

    handle->tThread = std::move(tThread);

    {
        std::lock_guard<std::mutex> lock(taskHandlesMutex);
        taskHandles.push_back(std::move(handle));
    }
}

void TaskHandler::cleanHandles() {
    std::lock_guard<std::mutex> lock(taskHandlesMutex);

    for (auto it = taskHandles.begin(); it != taskHandles.end();){
        if ((*it)->complete->load()) { 
            std::ostringstream oss;
            oss << "task handler: completed " << (*it)->tTitle;
            SLOG.info(oss.str());

            if ((*it)->tThread.joinable()) {
                (*it)->tThread.join(); 
            }

            it = taskHandles.erase(it);
        } else { ++it; }
    }
}

void TaskHandler::AddTask(std::unique_ptr<Task> t){
    std::ostringstream oss;
    oss << "adding task: " << t->GetName();

    SLOG.info(oss.str());

    {
        std::lock_guard<std::mutex> lock(this->tasksBufMutex);
        this->tasksBuf.push(std::move(t));
    }
}

void TaskHandler::End(){ this->running = false; }
