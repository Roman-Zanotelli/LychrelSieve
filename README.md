# WIP
Currently functional but not complete, compose file has some basic usage configurations but I plan to add more debug/print options as ENV vars for mor descriptive details.
# Scope
Create a multithreaded sieve for possible lychrel canadites using rust\n
Uses Google's Distroless Image\n
Uses Docker Compose for easy testing
# Lab Prerequisites
- Docker
- Docker-Compose
# CFG
- START_NUMBER: Starting Number
- START_ITER: Starting Iteration Count (only used for single number)
- MAX_ITER: Max Amount of iterations before stopping number (0 is infinite)
- MAX_NUMBER: Largest Number to test (0 is infinite)
- MAX_CONCURRENT_SEEDS: MAX amount of numbers that can be concurrently tested (default 1)
- SINGLE_NUMBER_ONLY: Switches Modes, True only tests 1 number up until MAX_ITER (default true)
# Todo
- Clean up unused code
- Memory Optimizations
- Refractor Code Quality/Conciseness
- Fix Single Number Multi-Threading
- Commenting
- Add additional print options
# About
Lychrel numbers are numbers that do not become palindromic after any amount of iterations through a specific process of adding it to the reverse of itself.\n
For example the number 51 in the first iteration would be 51 + 15 = 66, meaning 51 is not a lychrel number.\n
Another example of a non-lychrel number is 59, 59 + 95 = 154, 154 + 451 = 605, 605 + 506 = 1111\n
This program by default considers all numbers after 300 iterations as possible Lychrel numbers, this value can be configured (with 0 being infite iterations) but is chosen because the largest known delayed palindrome occurs after 293 iterations.\n
There is no known way to "prove" any number in Base 10 is a Lychrel number, but lychrel numbers have been proved to exist in other bases such as Base 2