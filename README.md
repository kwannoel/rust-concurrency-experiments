# Rust concurrency experiments

- Performance of concurrent / parallel Rust programs

- Problem:
  How fast can we run 3 distinct and expensive jobs, which are completely independent of each other.

- Solution candidates:

## Run these 3 tasks in sequence

  ```
  // sequence.rs
  $ time cargo run --bin sequence > /dev/null
  cargo run --bin sequence > /dev/null  0.48s user 0.24s system 4% cpu 16.101 total
  ```

## Run these 3 tasks in the same program, with `futures + tokio`
  
  ```
  // task.rs
  $ time cargo run --bin futures > /dev/null
  cargo run --bin futures > /dev/null  0.15s user 0.07s system 2% cpu 10.211 total
  cargo run --bin futures  0.15s user 0.07s system 4% cpu 5.208 total
  ```
  
  For some reason sometimes you have to run this multiple times to get this perfomance? 
  My guess is that OS gives us more threads only then...
  In production we might need some way to use majority of core threads on our program.
  The upside is that the upper bound is synchronous execution of tasks.

## Run these 3 tasks in 3 different programs

``` sh
$ time cargo run --bin single-task & cargo run --bin single-task & cargo run --bin single-task

cargo run --bin single-task  0.15s user 0.08s system 4% cpu 5.352 total
```


## Task
  Use `sleep` for `5s` to simulate compute.
  Hypothesis:
  - Sequence should take 15s
  - Futures + tokio should take 5s
  - 3 task parallel should take 5s
