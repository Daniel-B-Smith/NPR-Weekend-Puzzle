Puzzle definition
------

The problem is a variation on the classic word ladder problem. Word paths are created by adding or deleting one
letter from the preceding word. As an example, a path from red to rose:

red ->
rued ->
rue ->
ruse ->
rouse ->
rose

Additionally, all words must have at least three letters and no plurals or verbs added by adding the letter 's'.
What is the shortest path from 'whole' to 'heart'?

Solution
-------

My first solution is in letter_insert_delete.cc. The basic algorithm is a hybrid of Djikstra and bread-first search that
runs in O(#Edges) by taking advantage of the fact that all the edges have the same weight, which negates the need for the
priority queue. On my machine, the code was finding the path in ~24 seconds.

In a bout of premature optimization, I'm used `const string*` everywhere to avoid string copies without profiling/any
evidence I was getting value from that. I changed to use value semantics everywhere in letter_insert_delete_copy.cc. The
program ran maybe ~0.5 secs slower, but it seems that string copies didn't really matter.

For whatever reason, I decided to iterate on the `const string*` version. Profiling showed that checking for the length
of the two strings inside `is_single_edit()` was by far the most expensive line, which leads to the obvious optimization of
grouping the strings by string length. Unlike my string copy change, that optimization actually matter. The code in
letter_insert_delete_len_map.cc dropped the execution time from ~24 secs all the way down to ~17 secs.

I finally took profiling seriously and was able to find some small optimizations here and there that got the execution
all the way down to ~13 secs. At that point, I didn't see any more low-hanging fruit from the profiler output. However,
the profile really smelled like I had a data locality problem. Either that, or I just knew I was iterating over a vector
of pointers and know that dereferencing a pointer at every step in your loop is likely to cause data locality problems.
After starting to look into gcc's prefetching intrinsics, I realized that my implementation just copying everything cost
very little and that converting to value semantics would be easier and more portable than dealing with gcc prefetching.
Lo and behond, letter_insert_delete_len_map_copy.cc runs in just over 9 secs. The big win comes from both a 7% drop in
number of instructions along with a nearly 10% increase in instructions per cycle. The branch prediction rate stayed
effectively constant, so data locality is probably the best explanation.

Notes
-------

All timing results were compiled with gcc 4.8.4 on Ubuntu 14.04 using this command:

    g++ -O3 --std=c++11 <filename>

All profiling was done with Valgrind's callgraph and visualized with kcachegrind. The code also compiles and runs with
Clang 3.6.0.
