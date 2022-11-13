# Parallel Binary Adder - Advanced Parallel Systems Course Project

## Goal 

This small project was given in my advanced parallel system course, where we had to design a divide and conquer algorithm from a leetcode exercise. I chose the binary adder problem (https://leetcode.com/problems/add-binary/) where the goal is to add 2 binary number that are formatted into strings.

## Usage

This is a cargo project, and therefor is run using the "cargo run" command. Additional arguments can be given to modify the size of the binary numbers, the depth of the parallelization, and the number of times we repeat the operation with different binary numbers (with the same previous parameters).

The program will then perform the operation and measure the time took by the sequential function and the parallel one, and display both on the terminal. The program will also save an svg file destined to show how the task was divided and how the threads ran.

### Branch "performance evaluation"

This branch contains a version of **main** that does not take user inputs, and runs the code for several input sizes and levels 100 times, then averages out the times and prints them.

WARNING : If you whish to run this version, be aware that the last few tests will take from a few minutes to a few hours to finish depending on your machine because of the sequential functions (especially with size $2^{20}$ that took me 2 or 3 hours to be done, without good results to show for it).

## Code
### Sequential version (seq_add_binary_v*x*)

The first thing I do, to not have trouble with the iterators later, is to *pad* each binary number so that they have the same length. I do it using my function **pad_binaries** that looks something like this :

```
fn pad_binaries(b1: String, b2: String) -> (String, String) {
    // Output variables, initially at 0 for the eventual last carry
    String o1 = "0";
    String o2 = "0";

    (l1, l2) = (b1.len(), b2.len());

    // Adding the padding 0s to have the 2 String be the same size
    if l1 == l2 {}
    else if l1 > l2 {o2.add("0" l1-l2 times);}
    else if l1 < l2 {o1.add("0" l2-l1 times);}

    // Adding the actual numbers
    o1.add(b1);
    o2.add(b2);

    return (o1,o2)
}
```

After that, I create an iterator **iter** which is a zip of the two numbers reversed, then turned into iterators. 

#### V1

In the first version, I go through the iterator using a for loop and add the bits together, remembering the carry for the next iteration each time. Once this is finished I just return the result.

#### V2

In the second version, instead of doing a for loop, I apply a fold operation on the iterator where I basically do the same thing I did in my for, add the bits, put the result in a string and remember the carry (both going into a (String, char) tuple in the fold accumulator). 

After that I just take the result from the accumulator, which is the first element of the tuple.

### Parallel version (par_add_binary)

I've chosen to make the parallel version use *"levels"*, where 1 is just sequential, and each additional level divide the load in two recursively (ending up with $2^{levels-1}$ division at the end).

To archive that, we do as we did before, padding the binary numbers, then call an auxillary recursive function **par_add_rec** that , if the input level is 1, does the same operation as **seq_add_binary_v2**, but returns both the output and the last carry. 

If the given level is greater that 1, then we cut both binary strings in half, then call **join** with the same function **par_add_rec**, giving them the 2 pieces of the strings and **level-1**.

After retrieving the output strings and carry, we check if the right side had a carry that needs to be propagated (meaning we turn all ones into 0s until we find a 0, that gets turned into a 1).
We also check if the propagation of the carry in the left part generated a carry, and stop the process if there was also an already existing carry for the left output, as it shouldn't be possible (if there already was a carry, there would be at least one 0 in the bits of the result, preventing another carry from being created).

To finish the recursion, we glue both sides back together and return it along with the carry for the level above to take care of it.
Once we return to our original function **par_add_binary**, we can just return the string we received, as there shouldn't be any carry (because of the padding that added an extra 0).

### Small comment on the addition of bits

The simple way I add bits in rust is by mapping the 2 bits and the carry to numbers, then adding them in a variable **s**. I can then directly return the result of the addition with the output bit **b** being **s** mod 2, and the carry being **s** divided by 2 (integer division).

I don't actually know if this method is faster that any other, I just found it elegant.

## Theoretical Runtime

We have here a recursive *divide and conquer* algorithm, where we do a specific number **level** (that we will call *l* from now on) of recursions. At each step of the recursion, we divide the problem into 2 sub-task, each having half the problem size. Moreover, the final level executes the addition in $O(n)$, and each merge are done (as far as I know) in $O(n)$ too as adding 1 to the left result is done in $O(n/2)\in O(n)$. The root is done in $O(1)$ as no big operation is done there.

As such, we are in the case where the cost and the work is $O(n)$, the same as the sequential algorithm. The depth is dependent on *l* here, and it is $l-1$. As such, our expected time $t_{p}$ ends up being :
$$t_p = \frac{O(n)}{p} + O(1) = \frac{n}{2^{l-1}} + O(1)$$

## Observed Runtime

### Machine specification

The machine I've tested this program on is a linux machine with 12 available threads. To therefore stay relevant, I'll only talk about my results up to $l=4$, giving us 8 parallel execution.

### Sequential time observation 

Testing for $n=2^{10}$ up to $n=2^{19}$ ($2^{20}$ gave erroneous results), we end up with these sequential times :

| n | Sequential Time (in ms) | $O(n)$ |
| :---: |    :---:   | :---: |
| $2^{10}$ | 1.06 | 1 |
| $2^{11}$ | 2.05 | 2 |
| $2^{12}$ | 4.28 | 4 |
| $2^{13}$ | 8.76 | 8 |
| $2^{14}$ | 17.71 | 16 |
| $2^{15}$ | 40.52 | 32 |
| $2^{16}$ | 108.52 | 64 |
| $2^{17}$ | 313.86 | 128 |
| $2^{18}$ | 1029.02 | 256 |
| $2^{19}$ | 4164.63 | 516 |

As we can see, the times from $2^{10}$ to $2^{14}$ seem to follow the $O(n)$ trend, but from there, it goes up exponentially compared to what would be expected, leading me to believe that the actual complexity is more akin to $O(n^k)$ with $k\in [1,2]$. 

When trying to fit this data to a curve of this type, I find an equation $y=2,200524\times 10^{-8}n^{1.971579}$, so I'm probably not far from $O(n^{1.971579})$ for the sequential version.

As a conclusion, some operations I do in the rust program probably have a higher complexity than I thought.

### Parallel time observation 

Testing for the same sizes of $n$ and $l$ going from 1 (same as sequential) to 4 (8 parallel execution), I end up with : 

| n | $l=1, p=1$ | $l=2, p=2$ | $l=3, p=4$ | $l=4, p=8$ |
| :---: | :---: | :---: | :---: | :---: |
| $2^{10}$ | 1.03 | 0.72 | 0.51 | 0.33 |
| $2^{11}$ | 2.00 | 1.37 | 0.90 | 0.52 |
| $2^{12}$ | 4.20 | 3.02 | 1.82 | 1.14 |
| $2^{13}$ | 8.29 | 6.16 | 3.45 | 2.24 |
| $2^{14}$ | 16.67 | 11.36 | 6.71 | 4.37 |
| $2^{15}$ | 39.23 | 24.28 | 13.89 | 9.57 |
| $2^{16}$ | 101.02 | 53.34 | 26.06 | 18.19 |
| $2^{17}$ | 333.40 | 116.06 | 52.25 | 35.83 |
| $2^{18}$ | 1120.41 | 328.27 | 121.11 | 72.73 |
| $2^{19}$ | 4326.25 | 1149.62 | 358.37 | 180.86 |

As expected, when $l=1$, the runtime is almost the same as the sequential version, although slightly higher (which is also to be expected). The surprising part, just like it was with the sequential analysis, is that those runtime do not follow the trend we expected them to. We expected to find the sequential time divided by $p$ each time, which is not the case at all. If we focus on $l=4$ for example, the time follow a curve akin to $n\times log(n)$, which might be because of the previous result showing that the sequential time. 

If we look at the speedup :

| n | SpeedUp ($p = 1$) |	SpeedUp ($p = 2$)	| SpeedUp ($p = 4$) | SpeedUp ($p = 8$) | 
| :---: | :---: | :---: | :---: | :---: |
| $2^{10}$ | 1.02 | 1.4 | 2.07	| 3.17|
| $2^{11}$ | 1.02 | 1.4 | 2.28	| 3.92|
| $2^{12}$ | 1.01 | 1.4 | 2.34	| 3.75|
| $2^{13}$ | 1.05 | 1.4 | 2.53	| 3.9 |
| $2^{14}$ | 1.06 | 1.5 | 2.63	| 4.05|
| $2^{15}$ | 1.03 | 1.6 | 2.91	| 4.23|
| $2^{16}$ | 1.07 | 2.0 | 4.16	| 5.96|
| $2^{17}$ | 0.94 | 2.7 | 6    | 8.76|
| $2^{18}$ | 0.91 | 3.1 | 8.49 | 14.14|
| $2^{19}$ | 0.96 | 3.6 | 11.62| 23.02|

We can see that for $p=1$, the speedup stays around 1, which is what we expected. The others seem to follow a non linear positive function.

## Conclusion 

I might not be very good at complexity analysis, but the program works really well.
