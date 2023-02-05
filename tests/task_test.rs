#![cfg(test)]

extern crate completely_fair_scheduler as cfs;
extern crate raw_pointer as rptr;

use cfs::proc::task::{Task, TaskStatus};
use rptr::Pointer;

#[test]
fn test_execute() {
    // Initialize a variable `x` with value 3
    let mut x = 3;

    // Create a vector `tasks` to store pointers to `Task` objects
    let mut tasks: Vec<Pointer<Task>> = Vec::new();

    // Create two `Task` objects
    let mut my_task = Task::new(1, 5, 5, 0, 1, 1);
    let mut other_task = Task::new(2, 10, 5, 0, 1, 1);

    // Push the pointers to the `Task` objects into the `tasks` vector
    tasks.push(Pointer::new(&mut my_task));
    tasks.push(Pointer::new(&mut other_task));

    // Keeps running tasks while there is any task in the task queue
    while tasks.len() > 0 {
        let y = tasks.len();

        // Vector to store the indices of tasks that need to be removed
        let mut to_remove: Vec<usize> = Vec::new();

        // Loop through all tasks
        for i in 0..y {
            let mut task = tasks[i];

            println!("Performing sequence for task {:?}", task.get_id());

            // Check the task status and perform respective operations
            match task.get_status() {
                TaskStatus::Running => {
                    match task.get_id() {
                        1 => {
                            x = x + 1;
                            println!("Executing task 1");
                        }
                        id => println!("Executing task {:?}", id),
                    };
                    task.cpu_cycle();
                }
                TaskStatus::Waiting => {
                    task.run();
                }
                TaskStatus::Idle => {
                    task.io_cycle();
                }
                TaskStatus::New => {
                    task.schedule();
                }
                TaskStatus::Terminated => {
                    to_remove.push(i);
                }
            };
        }

        // Remove all terminated tasks from the task queue
        for idx in to_remove {
            tasks.remove(idx);
        }
    }

    println!("All tasks terminated");

    assert_eq!(x, 8);
}

#[test]
fn test_preempt() {
    let mut x = 3;

    let mut tasks: Vec<Pointer<Task>> = Vec::new();

    let mut my_task = Task::new(1, 8, 2, 5, 1, 1);
    let mut other_task = Task::new(2, 10, 7, 3, 1, 1);

    my_task.schedule();
    other_task.schedule();

    tasks.push(Pointer::new(&mut my_task));
    tasks.push(Pointer::new(&mut other_task));

    while tasks.len() > 0 {
        let y = tasks.len();
        let mut to_remove: Vec<usize> = Vec::new();

        let mut task = {
            let mut min_key = 0;
            for i in 1..y {
                if tasks[i].get_runtime() < tasks[min_key].get_runtime() {
                    if tasks[min_key].get_status() == TaskStatus::Running {
                        tasks[min_key].schedule(); // preempt
                    }

                    min_key = i;
                }
            }
            println!("Performing sequence for task {:?}", tasks[min_key].get_id());
            tasks[min_key]
        };

        // If the task's status is "Waiting", run the task.
        if task.get_status() == TaskStatus::Waiting {
            task.run();
        }

        // If the task's status is "Running", execute the task's code.
        if task.get_status() == TaskStatus::Running {
            match task.get_id() {
                1 => {
                    x = x + 1;
                }
                id => println!("Executing task {:?}", id),
            };
            task.cpu_cycle();
        }

        // Check all tasks for termination or idle status.
        for i in 0..y {
            let curr = tasks[i];
            if curr.get_status() == TaskStatus::Terminated {
                to_remove.push(i);
            }
            if curr.get_status() == TaskStatus::Idle {
                curr.io_cycle();
            }
        }

        // Remove all terminated tasks from the vector.
        for idx in to_remove {
            tasks.remove(idx);
        }
    }

    println!("All tasks terminated");

    assert_eq!(x, 11);
}
