# remove the existing "tasks.txt" file
import os
os.remove("tasks.txt")

# create a new "tasks.txt" file
task_file = open("tasks.txt", "x")

# import the random module
import random

# generate a random number of tasks
num_tasks = random.randint(1, 2 ** 4)

# initialize a list to store the task parameters
tasks = []

# loop to generate task parameters
for i in range(num_tasks):
    # generate random values for max_cpu_time, cpu_burst_len, io_burst_len, and weight
    max_cpu_time = random.randint(1, 2 ** 20)
    tasks.append(
        (
            max_cpu_time,
            random.randint(1, min(2 ** 16, max_cpu_time)),
            random.randint(0, 2 ** 11),
            random.randint(1, 2 ** 5)
        )
    )

# write the task parameters to the file
for cpu_time, cpu_burst_len, io_burst_len, weight in tasks:
    task_file.write(f'{cpu_time} {cpu_burst_len} {io_burst_len} {weight}\n')

# close the file
task_file.close()
