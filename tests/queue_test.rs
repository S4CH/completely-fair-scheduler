#![cfg(test)]

// Import the `completely_fair_scheduler` crate
extern crate completely_fair_scheduler as cfs;

// Use types from the imported crate
use cfs::proc::task::{Task, TaskStatus};
use cfs::proc::queue::TaskQueue;

#[test]
// Test function for popping tasks from the task queue
fn test_popping() {
    // Create 6 tasks with different parameters
    let task_one    = Task::new(1, 13191, 10, 5, 1, 1);
    let task_two    = Task::new(2, 13289, 10, 5, 1, 1);
    let task_three  = Task::new(3, 139, 10, 5, 2, 1);
    let task_four   = Task::new(4, 31921, 5, 10, 3, 1);
    let task_five   = Task::new(5, 3874, 7, 3, 5, 1);
    let task_six    = Task::new(6, 17013, 10, 6, 5, 1);

    // Create a new task queue
    let mut task_queue = TaskQueue::new();
    
    // Add the 6 tasks to the queue
    task_queue.add(task_one);
    task_queue.add(task_two);
    task_queue.add(task_three);
    task_queue.add(task_four);
    task_queue.add(task_five);
    task_queue.add(task_six);

    // Counter to keep track of the current time
    let mut idx = 1;
    // Continue popping tasks while the queue is not empty
    while !task_queue.is_empty() {
        // Pop the tasks from the queue
        let tasks = task_queue.pop();
        for mut task in tasks {
            // Schedule the task
            task.schedule();
            // Assert that the start time of the task is equal to the current time
            assert_eq!(task.get_start_time(), idx);
            // Assert that the status of the task is waiting
            assert_eq!(task.get_status(), TaskStatus::Waiting);
        }
        // Update the current time
        idx = match idx {
            1 => 2,
            2 => 3,
            3 => 5,
            _ => 0
        };
    }
    // Assert that the current time is 0 after all tasks have been popped
    assert_eq!(idx, 0);
}


#[test]
fn test_remove() {
    // Creating six tasks with different task_ids and values.
    let task_one    = Task::new(1, 13191, 10, 5, 1, 1);
    let task_two    = Task::new(2, 13289, 10, 5, 1, 1);
    let task_three  = Task::new(3, 139, 10, 5, 2, 1);
    let task_four   = Task::new(4, 31921, 5, 10, 3, 1);
    let task_five   = Task::new(5, 3874, 7, 3, 5, 1);
    let task_six    = Task::new(6, 17013, 10, 6, 5, 1);

    // Creating an empty task queue.
    let mut task_queue = TaskQueue::new();
    
    // Adding tasks to the queue.
    task_queue.add(task_one);
    task_queue.add(task_two);
    task_queue.add(task_three);
    task_queue.add(task_four);
    task_queue.add(task_five);
    task_queue.add(task_six);

    // Setting index to 1.
    let mut idx = 1;

    // Until the task queue is empty.
    while !task_queue.is_empty() {
        // Removing tasks from the queue with the task_id specified by `idx`.
        let tasks = task_queue.remove(idx);
        for mut task in tasks {
            // Scheduling the task.
            task.schedule();
            // Checking if the task's start_time is equal to the value of `idx`.
            assert_eq!(task.get_start_time(), idx);
            // Checking if the task's status is 'Waiting'.
            assert_eq!(task.get_status(), TaskStatus::Waiting);
        }
        // Updating the value of `idx` in each iteration.
        idx = match idx {
            1 => 2,
            2 => 3,
            3 => 5,
            _ => 0
        };
    }
    // Checking if the final value of `idx` is equal to 0.
    assert_eq!(idx, 0);
}
