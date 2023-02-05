// Brings the Clock struct from the clock module in the parent module.
use super::clock::Clock;

// Brings the FairAlgorithm struct from the fair module in the parent module.
use super::fair::FairAlgorithm;

// Brings the TaskQueue struct from the queue module in the proc module.
use crate::proc::queue::TaskQueue;

// Brings the Task and TaskChar structs from the task module in the proc module.
use crate::proc::task::{Task, TaskChar};

// Brings the Mutex and mpsc types from the std library.
use std::sync::{mpsc, Arc, Mutex};
// Brings the thread module from the std library.
use std::thread;

// Defines the Scheduler struct.
pub struct Scheduler {
    // The clock field is an Arc-wrapped Mutex-protected Clock instance.
    clock: Arc<Mutex<Clock>>,
}

impl Scheduler {
    // Constructor to create a new instance of Scheduler
    pub fn new() -> Self {
        let clock = Arc::new(Mutex::new(Clock::new()));

        Self { clock }
    }

    // Function to run the scheduler
    pub fn run(&mut self, tasks: Vec<TaskChar>) {
        // Cloning the clock object for use in different threads
        let clk_1 = Arc::clone(&mut self.clock);
        let clk_2 = Arc::clone(&mut self.clock);

        let mut threads = vec![];

        // Creating channels for communication between different threads
        let (clock_sender_1, spawner_clock_recv) = mpsc::channel();
        let (clock_sender_2, clock_recv) = mpsc::channel();
        let (born_sender, born_recv) = mpsc::channel();

        // Spawning a thread to control the clock
        let clocking = thread::spawn(move || {
            // Infinite loop to keep ticking the clock
            for _ in 0..u128::MAX {
                let mut lock = clk_1.lock().unwrap();
                // Sending the current time of the clock to the spawner
                match clock_sender_1.send(lock.time()) {
                    Ok(_) => {}
                    _ => {}
                };
                // Sending the current time of the clock to the scheduler
                match clock_sender_2.send(lock.time()) {
                    Ok(_) => {}
                    _ => break,
                };
                // Ticking the clock
                lock.tick();
            }
            // Dropping the clock sender channels after the loop
            drop(clock_sender_1);
            drop(clock_sender_2);
        });
        // Adding the clock thread to the list of threads
        threads.push(clocking);

        // Creating a shared vector of tasks for communication between different threads
        let my_tasks = Arc::new(Mutex::new(tasks));
        let tasks_cp_1 = Arc::clone(&my_tasks);
        let tasks_cp_2 = Arc::clone(&my_tasks);

        // Spawning a thread to spawn tasks
        let spawning = thread::spawn(move || {
            // Receiving the current time from the clock thread
            let mut time = spawner_clock_recv.recv().unwrap();

            // Cloning the vector of tasks
            let born_tasks = tasks_cp_1.lock().unwrap().clone();

            // Loop to create tasks and send them to the scheduler
            for raw in born_tasks {
                // Try to receive the current time from the clock thread
                time = match spawner_clock_recv.try_recv() {
                    Ok(tick) => tick,
                    _ => time,
                };

                // Create a new task from the task character
                let task = Task::new(
                    raw.get_id(),
                    raw.get_cpu_time(),
                    raw.get_cpu_burst_length(),
                    raw.get_io_burst_length(),
                    time,
                    raw.get_weight(),
                );

                // Sending the created task to the scheduler
                match born_sender.send(task) {
                    Ok(_) => {}
                    _ => panic!("Running thread dropped unexpectedly"),
                };
            }

            // Clearing the vector of tasks after all tasks have been created
            tasks_cp_1.lock().unwrap().clear();
            // Dropping the clock and born sender channels after the loop
            drop(spawner_clock_recv);
            drop(born_sender);
        });

        // The function spawns two threads and pushes them onto the `threads` vector.
        threads.push(spawning);

        // The first spawned thread is named "running".
        let running = thread::spawn(move || {
            // Creates a new TaskQueue instance and assigns it to the variable `task_queue`.
            let mut task_queue = TaskQueue::new();
            // Creates a new FairAlgorithm instance and assigns it to the variable `rq`.
            let mut rq = FairAlgorithm::new(&mut clk_2.lock().unwrap());

            // An infinite loop is executed until a break statement is reached.
            loop {
                // The time is determined from the received value from the `clock_recv` channel.
                let time = match clock_recv.recv() {
                    Ok(tick) => tick,
                    // The loop breaks if there is an error in receiving from the channel.
                    _ => break,
                };

                // Tries to receive from the `born_recv` channel and adds the received task to the `task_queue`.
                match born_recv.try_recv() {
                    Ok(task) => task_queue.add(task),
                    // If there is no task received, the block is skipped.
                    _ => {}
                };

                // The function `pop` is called on the `task_queue` to get the born tasks.
                let born_tasks = task_queue.pop();
                // The `born_tasks` are pushed onto the `rq` FairAlgorithm instance.
                rq.push(born_tasks);

                // If `rq` is not empty, it performs the following actions.
                if !rq.is_empty() {
                    // The first task is popped from `rq`.
                    let mut curr = rq.pop();
                    // The task ID and the system time is printed.
                    println!(
                        "Running task id {:?} at system time {:?}",
                        curr.get_id(),
                        time
                    );
                    // The function `cpu_cycle` is called on the task.
                    curr.cpu_cycle();
                    // The task is reinserted into the `rq`.
                    rq.insert(*curr);
                }
                // The `idle` function is called on the `rq` FairAlgorithm instance.
                rq.idle();

                // If `rq` is finished and there are no more tasks in the `tasks_cp_2` vector, the loop breaks.
                if rq.is_finished() && tasks_cp_2.lock().unwrap().is_empty() {
                    break;
                }
            }

            // The `clock_recv` and `born_recv` channels are dropped.
            drop(clock_recv);
            drop(born_recv);
        });

        // The "running" thread is pushed onto the `threads` vector.
        threads.push(running);

        // The function joins all the threads in the `threads` vector.
        for thread in threads {
            thread.join().unwrap();
        }

        // A message is printed indicating that the scheduler job has completed.
        println!("Scheduler job completed!");
    }
}
