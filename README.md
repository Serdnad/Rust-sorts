This repository hosts a growing collection of sorting algorithm implementations programmed in Rust, which I've programmed as an exercise for becoming more familiar with Rust.

While this is meant primarily as a means for learning the language, I've attempted to optimize the algorithm implementations when reasonably convenient.

### Benchmarking
There currently isn't a built in way to test the performance of each algorithm. However, it's easy enough to conduct rough tests by running
```asm
time ./sorts <array_size> <algorithm>

// For example, to run quicksort on a list of 10000 numbers
time ./sorts 10000 quick
```

I may eventually integrate the Clap crate to provide some built in help and remove some guesswork. For now, the options for <algorithm> are:
```
// built in std sort => "built_in"
// quicksort => "quick"
// mergesort => "merge"
// bubblesort => "bubble"
// selection sort => "selection"
// insertion sort => "insertion"
```

### Future Goals
(That may or may not ever be achieved).
- Additional Sorting Algorithms (heap sort, shell sort, timsort, other fun sorts)
- Integrated Clap crate
- Optimization and tuning
- Code Cleanup and proper documentation
- Visualizations in terminal? Maybe?