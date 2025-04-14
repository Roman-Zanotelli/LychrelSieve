#Scope
Create a multithreaded sieve for possible lychrel canadites
#Prerequisites
+ Docker
+ Docker-Compose
#CFG
- START_NUMBER: Starting Number
- START_ITER: Starting Iteration Count (only used for single number)
- MAX_ITER: Max Amount of iterations before stopping number (0 is infinite)
- MAX_NUMBER: Largest Number to test (0 is infinite)
- MAX_CONCURRENT_SEEDS: MAX amount of numbers that can be concurrently tested (default 1)
- SINGLE_NUMBER_ONLY: Switches Modes, True only tests 1 number up until MAX_ITER (default true)