extern crate completely_fair_scheduler as cfs;

use cfs::sched::scheduler::Scheduler;
use cfs::proc::task::TaskChar;

use std::io::Read;
use std::fs::File;

fn main() {
    // Open the tasks file and check if it was successful
    let mut file = match File::open("tasks.txt") {
        Ok(file) => file,
        Err(e) => {
            println!("Could not find tasks file: {}", e);
            return;
        },
    };

    // Read contents of the file into a string
    let mut file_str = String::new();
    match file.read_to_string(&mut file_str) {
        Ok(_) => (),
        Err(e) => {
            println!("Unable to write contents of tasks file to string: {}", e);
            return;
        },
    };

    // Split the contents of the file into lines
    let task_lines = file_str.split("\n");

    // Create a vector of split lines
    let task_lines_vec: Vec<&str> = task_lines.collect();

    // Create a vector to store properties of each task
    let mut task_props = vec![];
    for line in task_lines_vec {
        // Split each line into task properties
        let task_props_split = line.split_whitespace().take(4);

        // Collect the task properties into a vector
        task_props.push(task_props_split.collect::<Vec<&str>>());
    }

    // Create a vector to store tasks
    let mut tasks = vec![];

    // Index for assigning task IDs
    let mut idx = 1;

    // Create tasks from the properties
    for task_prop in task_props {
        match &task_prop[..] {
            [cpu_time, cpu_burst_length, io_burst_length, weight] => {
                tasks.push(TaskChar::new(
                    idx,
                    cpu_time.parse::<u64>().unwrap(),
                    cpu_burst_length.parse::<u64>().unwrap(),
                    io_burst_length.parse::<u64>().unwrap(),
                    weight.parse::<u32>().unwrap(),
                ));
            }
            _ => continue,
        }
        idx += 1;
    }

    // Create a scheduler instance
    let mut scheduler = Scheduler::new();

    // Run the scheduler
    scheduler.run(tasks);
}
