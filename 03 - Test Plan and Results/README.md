# Test Plan and Results

The main deliverable we produced in this project was a minimal RTOS kernel that prioritizes security.
The majority of our security was gained by figuring out how to use the Rust programming language on
an embedded chip, and employing it as the source language for our kernel.

Once the kernel was working, the majority of our testing took the form of creating tasks for our kernel.
The heart of our small kernel is the task switching functionality. Our initial plan was to:

 - Create several tasks for the kernel to run
 - Ensure that the tasks work interoperably with each other by communicating with OS resource (global semaphores for now)
 - Act as a user attempting to produce a malicious binary that attempts to reach prohibited memory (buffer-overflow attack)
 - Ensure ensure that the compiler will never let us generate and thus run a malicious binary like the one mentioned
 - Produce malicious binaries outside of our kernel, load them into memory and attempt to run it

We were able to test verify our kernel's security for all points above except for the last. The last of the above
points would be something for future exploration given the appropriate amount of time.

For now, we've ensured that only safe "users tasks" will run in the kernel and that they work with the kernel resources
appropriately. The `os_switch` task-switching function iterates through all of the available tasks and runs them until
they finish. The tasks run in order of priority and gaurantee completion.
