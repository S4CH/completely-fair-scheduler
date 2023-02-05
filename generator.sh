#!/bin/bash

rm tasks.txt
touch tasks.txt

num_tasks=$((RANDOM % 16 + 1))

tasks=()
for ((i=0; i<num_tasks; i++)); do
  max_cpu_time=$((RANDOM % 1048576 + 1))
  limit=$((min 65536 $max_cpu_time))
  if [[ $limit -eq 0 ]]; then
    limit=1
  fi
  tasks+=($max_cpu_time $((RANDOM % limit + 1)) $((RANDOM % 2048 + 1)) $((RANDOM % 32 + 1)))
done

for task in "${tasks[@]}"; do
  echo $task >> tasks.txt
done
