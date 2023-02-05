#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TaskStatus {
    // Status of a task when it is not running and not waiting for any resources
    Idle,
    // Status of a task when it is currently running
    Running,
    // Status of a task when it has completed all its execution
    Terminated,
    // Status of a task when it is waiting for some resources to become available
    Waiting,
    // Status of a task when it is newly created and has not started yet
    New,
}

#[derive(Clone, Copy)]
pub struct TaskChar {
    // Unique identifier of the task
    id: u16,
    // Total CPU time required to complete the task
    cpu_time: u64,
    // Length of a CPU burst of the task
    cpu_burst_length: u64,
    // Length of an I/O burst of the task
    io_burst_length: u64,
    // A weight that determines the priority of the task
    weight: u32,
}

impl TaskChar {
    // Creates a new instance of TaskChar with specified attributes
    pub fn new(
        id: u16,
        cpu_time: u64,
        cpu_burst_length: u64,
        io_burst_length: u64,
        weight: u32
    ) -> Self {

        Self {
            id,
            cpu_time,
            cpu_burst_length,
            io_burst_length,
            weight
        }

    }

    // Returns the id of the task
    #[inline]
    pub fn get_id(&self) -> u16 { self.id }

    // Returns the total CPU time required to complete the task
    #[inline]
    pub fn get_cpu_time(&self) -> u64 { self.cpu_time }

    // Returns the length of a CPU burst of the task
    #[inline]
    pub fn get_cpu_burst_length(&self) -> u64 { self.cpu_burst_length }

    // Returns the length of an I/O burst of the task
    #[inline]
    pub fn get_io_burst_length(&self) -> u64 { self.io_burst_length }

    // Returns the weight that determines the priority of the task
    #[inline]
    pub fn get_weight(&self) -> u32 { self.weight }
}

#[derive(Debug)]
pub struct Task {
    id: u16, // unique identifier for the task
    cpu_time: u64, // amount of time the task will spend on the CPU
    cpu_burst_length: u64, // length of time the task will spend on the CPU in each burst
    io_burst_length: u64, // length of time the task will spend waiting for I/O in each burst
    state: TaskStatus, // current status of the task (Idle, Running, Terminated, Waiting, New)
    runtime: u64, // total amount of time the task has spent on the CPU
    vruntime: u64, // virtual runtime used by the task scheduler
    idle_time: u64, // total amount of time the task has spent waiting for I/O
    start_time: u128, // start time of the task in nanoseconds
    weight: u32, // priority weight of the task
}

impl Task {
    // Creates a new instance of Task with given parameters
    pub fn new(
        id: u16,
        cpu_time: u64,
        cpu_burst_length: u64,
        io_burst_length: u64,
        start_time: u128,
        weight: u32
    ) -> Self {

        Self {
            id,
            cpu_time,
            cpu_burst_length,
            io_burst_length,
            state: TaskStatus::New,
            runtime: 0,
            vruntime: 0,
            idle_time: 0,
            start_time,
            weight
        }

    }

    // Returns the task's id
    pub fn get_id(&self) -> u16 {
        self.id
    }

    // Returns the task's cpu time
    pub fn get_cpu_time(&self) -> u64 {
        self.cpu_time
    }

    // Returns the task's start time
    pub fn get_start_time(&self) -> u128 {
        self.start_time
    }

    // Returns the task's current state
    pub fn get_status(&self) -> TaskStatus {
        self.state
    }

    // Returns the task's current runtime
    pub fn get_runtime(&self) -> u64 {
        self.runtime
    }

    // Terminates the task
    pub fn terminate(&mut self) {
        self.state = TaskStatus::Terminated
    }

    // Returns the task's weight
    pub fn weight(&self) -> u32 {
        self.weight
    }

    // Returns the task's virtual runtime
    pub fn vruntime(&mut self, now: u128) -> u64 {
        let dt: u64 = now.overflowing_sub(self.start_time).0 as u64;
        let delta_exec_weighted: u64 = dt / (self.weight as u64);
        self.vruntime += delta_exec_weighted;

        self.vruntime
    }

    // Changes the task's state to idle
    pub fn to_idle(&mut self) {
        match self.state {
            TaskStatus::Terminated => panic!("Cannot yield a terminated task ({:?})!", self.id),
            _ => self.state = TaskStatus::Idle
        }
    }

    // Changes the task's state to waiting
    pub fn schedule(&mut self) {
        match self.state {
            TaskStatus::Terminated => panic!("Cannot schedule a terminated task: ({:?})!", self.id),
            _ => self.state = TaskStatus::Waiting
        }
    }

    // Changes the task's state to running
    pub fn run(&mut self) {
        self.state = TaskStatus::Running
    }


    pub fn restart(&mut self, time: u128) {
        // Resets the task's runtime, idle_time, status, and start_time to their default values. 
        self.runtime = 0;
        self.idle_time = 0;
        self.state = TaskStatus::New;
        self.start_time = time;
    }
    
    pub fn cpu_cycle(&mut self) {
        // Executes one CPU cycle for the task.
        // If the task is running, the runtime is incremented by 1. 
        // If the runtime is equal or greater than the task's cpu_time, the task is terminated. 
        // If the runtime is a multiple of the cpu_burst_length, the task goes idle.
        match self.state {
            TaskStatus::Running => {
                self.runtime += 1;
                if self.runtime >= self.cpu_time {
                    self.terminate();
                } else if self.runtime % self.cpu_burst_length == 0 {
                    self.to_idle();
                }
            },
            _ => println!("Task {:?} is not running", self.id)
        }
    }
    
    pub fn io_cycle(&mut self) {
        // Executes one I/O cycle for the task.
        // If the task is idle, the idle_time is incremented by 1. 
        // If the idle_time is equal or greater than the task's io_burst_length, the task is scheduled to run.
        match self.state {
            TaskStatus::Idle => {
                self.idle_time += 1;
                if self.idle_time >= self.io_burst_length {
                    self.idle_time = 0;
                    self.schedule();
                }
            },
            _ => println!("Task {:?} is currently not idle", self.id)
        }
    }
    

// Implement the PartialEq trait for the Task struct
impl PartialEq for Task {
    fn eq(&self, other: &Task) -> bool {
        // Compare if two Task objects have the same id
        self.id == other.id
    }
}

// Implement the Clone trait for the Task struct
impl Clone for Task {
    fn clone(&self) -> Self {
        // Create a new Task struct with the same values as the original
        Self {
            id:                 self.id,
            cpu_time:           self.cpu_time,
            cpu_burst_length:   self.cpu_burst_length,
            io_burst_length:    self.io_burst_length,
            state:              self.state,
            runtime:            self.runtime,
            vruntime:           self.vruntime,
            idle_time:          self.idle_time,
            start_time:         self.start_time,
            weight:             self.weight
        }
    }
}

// Implement the Copy trait for the Task struct
impl Copy for Task {}

// Implement the Send trait for the Task struct to allow it to be sent between threads
unsafe impl Send for Task {}

// Implement the Sync trait for the Task struct to allow it to be shared between threads
unsafe impl Sync for Task {}
