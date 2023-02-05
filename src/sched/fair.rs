// Importing external crates and use statements
extern crate raw_pointer as rptr;
extern crate rbtree;
use super::clock::Clock;
use crate::proc::task::{Task, TaskStatus};
use rbtree::RBTree;
use rptr::Pointer;
use std::collections::VecDeque;

// Struct definition for FairAlgorithm
pub struct FairAlgorithm {
    // Red-black tree for tasks sorted by key (u64)
    tree: RBTree<u64, Task>,
    // Vector deque for idle tasks
    idle: VecDeque<Task>,
    // Pointer to the Clock object
    clock: Pointer<Clock>,
}

// Implementation block for FairAlgorithm
impl FairAlgorithm {
    // Constructor for FairAlgorithm
    pub fn new(clock: &mut Clock) -> Self {
        Self {
            // Initialize the red-black tree
            tree: RBTree::new(),
            // Initialize the vector deque for idle tasks
            idle: VecDeque::new(),
            // Store the clock object in a raw pointer
            clock: Pointer::new(clock),
        }
    }

    // Function to insert tasks into the FairAlgorithm object
    #[inline]
    pub fn push(&mut self, tasks: Vec<Task>) {
        // Iterate through the tasks in the input vector
        for task in tasks {
            // Call the insert function for each task
            self.insert(task);
        }
    }

    // Function to insert a task into the FairAlgorithm object
    #[inline]
    pub fn insert(&mut self, mut task: Task) {
        // Get the task's status
        let state = task.get_status();
        // If the task is terminated, return immediately
        if state == TaskStatus::Terminated {
            return;
        }
        // If the task is idle, add it to the idle queue
        else if state == TaskStatus::Idle {
            self.idle.push_back(task);
            return;
        }
        // Calculate the task's key (vruntime) using the clock's time
        let key: u64 = task.vruntime(self.clock.time());
        // Schedule the task
        task.schedule();
        // Insert the task into the red-black tree using the key as the key
        self.tree.insert(key, task);
    }

    // Function to pop a task from the FairAlgorithm object
    #[inline]
    pub fn pop(&mut self) -> Box<Task> {
        // If the tree is empty, panic
        if self.is_empty() {
            panic!("Attempted to pop from an empty tree");
        }
        // Pop the first task from the red-black tree
        let mut task = Box::new(self.tree.pop_first().unwrap().1);
        // Run the task
        task.run();

        task
    }

    // Function to check if the red-black tree in the FairAlgorithm object is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        // Return whether the red-black tree is empty
        self.tree.is_empty()
    }

    // Function to check if the FairAlgorithm object is finished running all tasks
    #[inline]
    pub fn is_finished(&self) -> bool {
        // Return whether the red-black tree is empty and the idle queue has no tasks
        self.is_empty() && self.idle.len() == 0
    }

    // Function to run a task
    pub fn run(&mut self) {
        // If the red-black tree is empty, return
        if self.is_empty() {
            return;
        }
        // Pop the first task from the red-black tree
        let mut task = *self.pop();
        // Run a CPU cycle for the task
        task.cpu_cycle();
        // Insert the task back into the FairAlgorithm object
        self.insert(task);
    }

    // Function to perform an IO cycle for an idle task
    pub fn idle(&mut self) {
        // If the idle queue has no tasks, return
        if self.idle.len() == 0 {
            return;
        }
        // Pop the first task from the idle queue
        let mut curr = self.idle.pop_front().unwrap();
        // Run an IO cycle for the task
        curr.io_cycle();
        // Insert the task back into the FairAlgorithm object
        self.insert(curr);
    }
}
// Implement the Sync trait for FairAlgorithm to make it thread-safe
unsafe impl Sync for FairAlgorithm {}
