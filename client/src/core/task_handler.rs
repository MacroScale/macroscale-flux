/*
------------------------------------
            Task Handler 
------------------------------------
Handles the executing of all tasks in the system, this 
includes tasks that will run for the lifetime of the application
for instances such as polling events.

*/

use std::{collections::VecDeque, sync::Arc, time::Duration};

use tokio::{sync::Mutex, task::{spawn_local, JoinHandle}, time};

use crate::base::{event_loop::{EventDispatcher, EventLoop}, task::{Task, TaskMeta}};

pub struct TaskHandler {
    task_queue: Arc<Mutex<VecDeque<Box<dyn Task>>>>,
    handles: Arc<Mutex<Vec<TaskHandle>>>,
}

struct TaskHandle {
    task_meta: TaskMeta,
    handle: JoinHandle<()> 
}

impl TaskHandle{
    fn create(t: Box<dyn Task>, event_loop: Arc<EventLoop>, dispatcher: EventDispatcher, task_handler: Arc<TaskHandler>) -> TaskHandle {
        let meta = t.data().clone();
        log::info!("creating task handle for task: {}", meta.name);
        let task_handle = spawn_local(t.execute(task_handler.clone(), event_loop.clone(), dispatcher));

        TaskHandle{
            task_meta: meta,
            handle: task_handle 
        }   
    }
}

impl TaskHandler {
    pub fn new() -> Arc<TaskHandler> {
        Arc::new(TaskHandler { 
            task_queue: Arc::new(Mutex::new(VecDeque::new())),
            handles: Arc::new(Mutex::new(Vec::new())),
        })
    }

    pub async fn add_task(&self, task: Box<dyn Task>) {
        log::info!("adding task to task queue: {}", task.data().name);
        self.task_queue.lock().await.push_back(task) 
    }

    async fn push_handle(&self, handle: TaskHandle) {
        self.handles.lock().await.push(handle) 
    }

    async fn pop_task(&self) -> Option<Box<dyn Task>> {
        self.task_queue.lock().await.pop_front() 
    }

    async fn run_tasks(&self, task_handler: Arc<TaskHandler>, event_loop: Arc<EventLoop>, dispatcher: EventDispatcher){
        let next_task = self.pop_task().await;

        if let Some(t) = next_task {
            let t_handle = TaskHandle::create(t, event_loop.clone(), dispatcher.clone(), task_handler.clone());
            log::info!("Starting task: {}", t_handle.task_meta.name);
            self.push_handle(t_handle).await;
        }
    }

    async fn clean_handles(&self){
        let mut handles = self.handles.lock().await;

        // clean handles by removing finished tasks
        handles.retain(|handle| {
            if handle.handle.is_finished() {
                log::info!("Task completed: {}", handle.task_meta.name);
                false // remove handle
            } else {
                true // keep handle
            }
        });
    }

}

pub async fn start(task_handler: Arc<TaskHandler>, event_loop: Arc<EventLoop>, dispatcher: EventDispatcher) {
    loop {
        task_handler.run_tasks(task_handler.clone(), event_loop.clone(), dispatcher.clone()).await;
        task_handler.clean_handles().await;
        time::sleep(Duration::from_millis(50)).await;
    }
}
