# LychrelSieve
Sieve Lychrel Candidates (Base 10)
# Scope
+ Write a multi-threaded program to find "Lychrel Candidates"
# Understanding Lychrel Numbers
Defining the mathematical concepts of Palindromic Numbers, Lychrel Numbers/Candidates, and the Iterative process as it pertains to the program itself. This will be a simplified overview of the information required to understand and implement the inner workings of this program; Following the order of:
+ "What Is a Palindromic Number?"
+ "What a Lychrel Number Actually Is?"
+ "What Is the Iterative Process?"
+ "Lychrel Number VS. Candidate" \
\
Based heavily on:\
https://en.wikipedia.org/wiki/Lychrel_number
## What Is a Palindromic Number?
Before explaining "Lychrel Numbers" or the "Iterative Process" it is crucial to understand Palindromic Numbers.\
\
A Number is a palindrome (or is Palindromic) when, similar to the linguistic context, all digits are the same when reveresed.\
\
Programatically, this state can be represented as "for all digits at position p < the center_position == the digit at the reversed position end_position - p", when represeted as an array of digits.\
\
Some examples are:
+ 11 
+ 121 
+ 353
+ 88991219988
## What a Lychrel Number Actually Is?
Simply put a Lychrel Number is a number in which a Palindromic result can not ever occur through the Iterative Process. \
\
Functionally this can be represented as the ItereativeProcess(N, i) != Palidromic Result for i = 1 -> inf Iterations. \
\
Based off my interpretation of the mathematical definition, a Lychrel Number itself may be a palindrome, but must still meet the prior conditions of the iterative process.
## What Is the Iterative Process?
Now that we understand Palindromic Numbers and Lychrel Numbers, it is imperative we understand the "Iterative Process" used to determine whether a number is NOT a Lychrel Number.\
\
The goal of the Iterative Process is not to find Lychrel Numbers but rather determine if a number is NOT a Lychrel Number/Candidate, There exists no solution or algorithm to finding Lychrel numbers and no verifiable proof to determine if any possible "canadite" is a Lychrel Number (In base 10, other Bases such as base 2 are an exception discussed later).\
\
That being said we can still find "Lychrel Candidates" through the execution of the iterative process through constant reiteration until a number of iterations (i) have passed worth accepting said number as a possible candidate (More later about Lychrel Numbers Vs Candidates). For conext the most well known and smallest Lychrel Candidate 196 was acknowledged in the 1980s as a possible candidate after 12,954 iterations by James Killman and later tested upto a billion iterations by Romain Dolbeau in 2011.\
\
With this understanding we cand define the iterative process as such:
+ Take a number N0, add it to the number formed by its reversal (example: N0 = 56, [56 + 65 = 121]).
+ If the sum is not Palindromic, Repeat the previous step with N[i] = IterativeSum[i-1], until either a resulting palindromic number occurs or an acceptable amount of iterations have passed labeling N0 as a possible "Lychrel Candidate".\
\
Example: N0 = 59, [59 + 95 = 154]; N1 = 154, [154 + 451 = 605]; N2 = 605, [605 + 506 = 1111]; 1111 is palindromic meaning 59 is not a Lychrel Number/Candidate. \
\
As stated before this will only determine if a number IS NOT a Lychrel Number, Some non-Lychrel Numbers are known only to converge to palindromic after 200+ iterations, with even accepted Lychrel Canidates having the possibility of convergence after an undeterminable amount.
## Lychrel Number VS. Candidate
To clarify there are no known Lychrel Numbers in the base 10 system as mentioned before, there are only possible Lychrel Candidates that have been discovered and tested upto an arbitrary i iterations.\
\
The main difference as mentioned briefly before is that a Lychrel Number is a Number in which provably cannot produce a palindromic number through the iterative process, while a Lychrel Candidate is a number tested to not produce a palindromic number within 0 -> i iterations but may still converge to palindromic as i -> inf iterations and in which no recognizable pattern occurs proving its Lychrelic nature (in contrast some base 2 numbers have shown to produce patterns proving their Lychrelic nature, for example 10110).