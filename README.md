```process_signal_lookup``` is a small utility that reads the ```status``` file of the process in the ```/proc``` directory and prints down the list of signals that are caught by the process, ignored by the process, blocked by the process, and signals that are pending for the thread or the whole process. This program requires the ```/proc``` directory to be mounted.

# Build

```process_signal_lookup``` uses the ```cargo``` build system and can be built using the command: ```cargo build```

# Usage

```./process_signal_lookup <PID>```
