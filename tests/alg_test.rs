#![cfg(test)]

extern crate completely_fair_scheduler as cfs;

use cfs::proc::queue::TaskQueue;
use cfs::proc::task::Task;
use cfs::sched::{clock::Clock, fair::FairAlgorithm};

use std::sync::{Arc, Mutex};
use std::thread;

#[test]
fn test_no_update() {
    // Create 6 tasks
    let task_one = Task::new(1, 13191, 10, 5, 1, 5);
    let task_two = Task::new(2, 13289, 10, 5, 1, 4);
    let task_three = Task::new(3, 139, 10, 5, 2, 8);
    let task_four = Task::new(4, 31921, 5, 10, 3, 4);
    let task_five = Task::new(5, 3874, 7, 3, 5, 2);
    let task_six = Task::new(6, 17013, 10, 6, 5, 5);

    // Create a new task queue
    let mut task_queue = TaskQueue::new();

    // Append the tasks to the task queue
    task_queue.append(&[
        task_one, task_two, task_three, task_four, task_five, task_six,
    ]);

    // Create a system clock and a fair scheduling algorithm
    let mut sysclock = Clock::new();
    let mut rq = FairAlgorithm::new(&mut sysclock);

    // Advance the system clock
    sysclock.tick();

    // While the task queue is not empty, pop tasks and add to the fair scheduling algorithm
    while !task_queue.is_empty() {
        let tasks = task_queue.pop();
        rq.push(tasks);
        sysclock.tick();
    }

    // Pop the first task from the fair scheduling algorithm
    let first = rq.pop();
    // Check if the id of the first task is equal to 1
    assert_eq!(first.get_id(), 1);

    // Advance the system clock
    sysclock.tick();

    // Pop the second task from the fair scheduling algorithm
    let second = rq.pop();
    // Check if the id of the second task is equal to 2
    assert_eq!(second.get_id(), 2);

    // Advance the system clock
    sysclock.tick();

    // Pop the third task from the fair scheduling algorithm
    let third = rq.pop();
    // Check if the id of the third task is equal to 3
    assert_eq!(third.get_id(), 3);
}

#[test]
fn test_with_update() {
    // Create task objects with specified properties
    let task_one = Task::new(1, 13191, 10, 5, 1, 5);
    let task_two = Task::new(2, 13289, 10, 5, 1, 4);
    let task_three = Task::new(3, 139, 10, 5, 2, 8);
    let task_four = Task::new(4, 31921, 5, 10, 3, 4);
    let task_five = Task::new(5, 3874, 7, 3, 5, 2);
    let task_six = Task::new(6, 17013, 10, 6, 5, 5);

    // Create a task queue and add all the task objects to it
    let mut task_queue = TaskQueue::new();
    task_queue.append(&[
        task_one, task_two, task_three, task_four, task_five, task_six,
    ]);

    // Create a system clock and a Fair Algorithm object and associate the system clock with it
    let mut sysclock = Clock::new();
    let mut rq = FairAlgorithm::new(&mut sysclock);

    // Start ticking the system clock
    sysclock.tick();

    // Loop through the task queue, pop tasks and push them to the Fair Algorithm, ticking the system clock after each iteration
    while !task_queue.is_empty() {
        let tasks = task_queue.pop();
        rq.push(tasks);
        sysclock.tick();
    }

    // Pop the first task from the Fair Algorithm and check if it's id is 1
    let mut curr = rq.pop();
    assert_eq!(curr.get_id(), 1);

    // Perform a CPU cycle on the task, tick the system clock, and insert the task back into the Fair Algorithm
    curr.cpu_cycle();
    sysclock.tick();
    rq.insert(*curr);

    // Pop the next task from the Fair Algorithm and check if it's id is 2
    curr = rq.pop();
    assert_eq!(curr.get_id(), 2);

    // Perform a CPU cycle on the task, tick the system clock, and insert the task back into the Fair Algorithm
    curr.cpu_cycle();
    sysclock.tick();
    rq.insert(*curr);

    // Pop the next task from the Fair Algorithm and check if it's id is 3
    curr = rq.pop();
    assert_eq!(curr.get_id(), 3);
}

#[test]
fn test_multithreaded_clock() {
    // Create a shared reference-counted mutex wrapped clock
    let mut sysclock = Arc::new(Mutex::new(Clock::new()));

    // A vector to store the spawned threads
    let mut threads = vec![];
    // Create a channel to send and receive clock values between threads
    let (sender, receiver) = std::sync::mpsc::channel();

    // Clone the shared clock and spawn a thread to send clock values through the channel
    let clk = Arc::clone(&mut sysclock);
    let sending = thread::spawn(move || {
        // 50 clock ticks
        for _ in 0..50 {
            // Acquire a lock on the clock and get its current value
            let mut lock = clk.lock().unwrap();
            sender.send(lock.time()).unwrap();
            // Tick the clock
            lock.tick();
        }
        // Drop the sender channel
        drop(sender);
    });
    // Push the sending thread to the vector
    threads.push(sending);

    // Clone the shared clock and spawn a thread to receive and use clock values from the channel
    let c_clk = Arc::clone(&mut sysclock);
    let receiving = thread::spawn(move || {
        // Create a new fair algorithm instance using the shared clock
        let mut rq = FairAlgorithm::new(&mut c_clk.lock().unwrap());

        // Get the first value from the channel
        let mut curr_time = receiver.recv().unwrap();
        // Create two tasks using the received clock value
        let task_one = Task::new(1, 15, 5, 3, curr_time, 1);
        // Try to get the next value from the channel, if not available use the previous value
        curr_time = match receiver.try_recv() {
            Ok(tick) => tick,
            Err(_) => curr_time,
        };
        let task_two = Task::new(2, 15, 3, 5, curr_time, 1);

        // Create a new task queue and append the two tasks
        let mut task_queue = TaskQueue::new();
        task_queue.append(&[task_one, task_two]);

        // Pop tasks from the queue and push them to the fair algorithm
        while !task_queue.is_empty() {
            let tasks = task_queue.pop();
            rq.push(tasks);
        }

        // Continuously receive clock values from the channel and perform actions on the fair algorithm
        loop {
            // Get the next value from the channel, if unavailable break from the loop
            let _time = match receiver.recv() {
                Ok(tick) => tick,
                Err(_) => break,
            };

            // If the fair algorithm is not empty, pop a task and perform a CPU cycle on it
            if !rq.is_empty() {
                let mut curr = rq.pop();
                curr.cpu_cycle();
                rq.insert(*curr);
            }
            // Call the idle function on the fair algorithm
            rq.idle();
        }
        // Drop the receiver channel
        drop(receiver);
    });
    threads.push(receiving);

    for thread in threads {
        thread.join().unwrap();
    }

    assert_eq!(sysclock.lock().unwrap().time(), 50);
}

#[test]
fn test_efficient_threads() {
    let mut sysclock = Arc::new(Mutex::new(Clock::new()));
    // Create a vector to store all the threads
    let mut threads = vec![];
    // Create two channels to send the current clock time to task_spawning thread
    let (clock_sender_1, spawner_clock_recv) = std::sync::mpsc::channel();
    let (clock_sender_2, clock_recv) = std::sync::mpsc::channel();
    // Create a channel to send the newly created tasks to the task scheduling thread
    let (born_sender, born_recv) = std::sync::mpsc::channel();

    // Clone the reference to the system clock for use in this thread
    let clk = Arc::clone(&mut sysclock);
    // Spawn a thread that ticks the system clock
    let ticking = thread::spawn(move || {
        for _ in 0..100 {
            let mut lock = clk.lock().unwrap();
            // Send the current time to both task_spawning and task scheduling threads
            match clock_sender_1.send(lock.time()) {
                Ok(_) => {}
                _ => {}
            };
            clock_sender_2.send(lock.time()).unwrap();
            lock.tick();
        }
        // Drop the two channels after use
        drop(clock_sender_1);
        drop(clock_sender_2);
    });
    // Add the ticking thread to the vector of threads
    threads.push(ticking);

    // Clone the reference to the system clock for use in this thread
    let c_clk = Arc::clone(&mut sysclock);
    // Spawn a thread that creates two tasks and sends them to the task scheduling thread
    let task_spawning = thread::spawn(move || {
        // Receive the current clock time from the ticking thread
        let mut time = match spawner_clock_recv.recv() {
            Ok(tick) => tick,
            Err(_) => panic!("Ran out of time before the processes could be born, check bounds"),
        };
        // Create the first task
        let task_one = Task::new(1, 15, 5, 3, time, 1);
        // Send the first task to the task scheduling thread
        match born_sender.send(task_one) {
            Ok(_) => {}
            _ => {}
        };
        // Receive the updated clock time from the ticking thread
        time = match spawner_clock_recv.try_recv() {
            Ok(tick) => tick,
            Err(_) => time,
        };
        // Create the second task
        let task_two = Task::new(2, 15, 3, 5, time, 1);
        // Send the second task to the task scheduling thread
        match born_sender.send(task_two) {
            Ok(_) => {}
            _ => {}
        };
    });
    // Add the task_spawning thread to the vector of threads
    threads.push(task_spawning);

    let receiving = thread::spawn(move || {
        // Create a TaskQueue and a FairAlgorithm instance
        let mut task_queue = TaskQueue::new();
        let mut rq = FairAlgorithm::new(&mut c_clk.lock().unwrap());

        loop {
            // Receive the tick time from the clock
            let _time = match clock_recv.recv() {
                Ok(tick) => tick,
                Err(_) => break, // Break the loop if there's no tick time
            };

            // Try to receive a new task from the `born_recv` channel
            match born_recv.try_recv() {
                Ok(task) => task_queue.add(task), // Add the task to the task queue
                _ => {}                           // No task received, do nothing
            };

            // Get the newly born tasks from the task queue
            let born_tasks = task_queue.pop();

            // Push the newly born tasks to the FairAlgorithm instance
            rq.push(born_tasks);

            // If there are tasks in the FairAlgorithm instance
            if !rq.is_empty() {
                // Pop the first task and run it for one cycle
                let mut curr = rq.pop();
                curr.cpu_cycle();

                // Insert the task back to the FairAlgorithm instance
                rq.insert(*curr);
            }

            // If there's no task to run, make the FairAlgorithm instance idle
            rq.idle();
        }
        // Drop the `clock_recv` channel
        drop(clock_recv);
    });
    // Add the `receiving` thread to the list of threads
    threads.push(receiving);

    // Join all the threads to wait for their completion
    for thread in threads {
        thread.join().unwrap();
    }

    // Assert that the time of the clock is equal to 100
    assert_eq!(sysclock.lock().unwrap().time(), 100);
}
