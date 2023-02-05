use super::task::{Task, TaskStatus};
use std::collections::HashMap;

pub struct TaskQueue {
    tasks: HashMap<u128, Vec<Task>>,
}

impl TaskQueue {
    // constructor to create a new instance of TaskQueue
    pub fn new() -> Self {
        Self { tasks: HashMap::new() }
    }

    // method to add a task to the task queue
    pub fn add(&mut self, task: Task) {
        // if the task is already terminated or idle, return from the function
        if task.get_status() == TaskStatus::Idle || task.get_status() == TaskStatus::Terminated {
            return;
        }
        
        // get the start time of the task
        let start_time = task.get_start_time();
        
        // if the start time is not already a key in the hashmap, insert the start time as key and an empty vector as its value
        if !self.tasks.contains_key(&start_time) {
            self.tasks.insert(start_time, Vec::new());
        }
        
        // add the task to the vector corresponding to the start time key
        self.tasks.get_mut(&start_time)
            .unwrap()
            .push(task);
    }

    // method to add multiple tasks to the task queue
    pub fn append(&mut self, tasks: &[Task]) {
        // add each task in the input vector to the task queue
        for task in tasks {
            self.add(*task);
        }
    }

    // method to retrieve and remove the task with the earliest start time from the task queue
    pub fn pop(&mut self) -> Vec<Task> {
        // if the task queue is empty, return an empty vector
        if self.is_empty() {
            return Vec::new();
        }
        
        // initialize the variable to keep track of the smallest key (i.e., start time)
        let mut smallest_key = 0;
        
        // find the smallest start time in the hashmap
        for key in self.tasks.keys() {
            if smallest_key > *key || smallest_key == 0 {
                smallest_key = *key;
            }
        }
        
        // remove the vector of tasks corresponding to the smallest start time key and return it
        self.tasks.remove(&smallest_key).unwrap()
    }

    // method to retrieve and remove the task with a specified start time from the task queue
    pub fn remove(&mut self, time: u128) -> Vec<Task> {
        // if the task queue is empty, return an empty vector
        if self.is_empty() {
            return Vec::new();
        }

        // remove the vector of tasks corresponding to the specified start time key and return it
        self.tasks.remove(&time).unwrap()
    }

    // method to check if the task queue is empty
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}
