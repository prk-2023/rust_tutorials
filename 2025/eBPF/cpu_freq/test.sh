#!/bin/bash 

# possible cpu governers
# cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_available_governors
# conservative ondemand userspace powersave performance schedutil
#
# Testing possible governers with this eBPF program 
GOV_LIST="conservative
userspace
powersave
performance
"
condition=1
while [ condition ]
do
  # Commands to be executed
  while IFS= read -r word; do 
      echo "$word" | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor
      sleep 5
  done <<< "$GOV_LIST"
done

