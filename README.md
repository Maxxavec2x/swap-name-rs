# An attempt to make a rust port of exch:

I'm currently learning rust, and I wanted to try to make a command that swapped two filename without changing their content. I didn't know the ```exch``` command, that does exactly that, but I DID know a kernel syscall called ```renameat2```` that does exactly what I needed.  
So I tried to make a rust wrapper of this syscall, to see if it would be more efficient that doing ```mv toto toto.bak && mv tata toto && mv toto.bak tata```, and it is :

Benchmark swap-name-rs against 3-mv:
```
❯ hyperfine 'mv toto toto.bak && mv tata toto && mv toto.bak tata' './swap-name-rs toto tata'
Benchmark 1: mv toto toto.bak && mv tata toto && mv toto.bak tata
  Time (mean ± σ):       1.7 ms ±   0.2 ms    [User: 1.3 ms, System: 0.5 ms]
  Range (min … max):     1.2 ms …   2.3 ms    1053 runs
 
  Warning: Command took less than 5 ms to complete. Note that the results might be inaccurate because hyperfine can not calibrate the shell startup time much more precise than this limit. You can try to use the `-N`/`--shell=none` option to disable the shell completely.
 
Benchmark 2: ./swap-name-rs toto tata
  Time (mean ± σ):     642.6 µs ± 137.9 µs    [User: 524.8 µs, System: 418.8 µs]
  Range (min … max):   334.5 µs … 1064.9 µs    1980 runs
 
  Warning: Command took less than 5 ms to complete. Note that the results might be inaccurate because hyperfine can not calibrate the shell startup time much more precise than this limit. You can try to use the `-N`/`--shell=none` option to disable the shell completely.
 
Summary
  ./swap-name-rs toto tata ran
    2.63 ± 0.63 times faster than mv toto toto.bak && mv tata toto && mv toto.bak tata
```  

But then, I learned about the fking exch command, that does LITTERALLY what I did, but better, and I tried to run a benchmark against it:
```
❯ hyperfine './swap-name-rs toto tata' 'exch toto tata'                                      
Benchmark 1: ./swap-name-rs toto tata
  Time (mean ± σ):     695.0 µs ± 141.3 µs    [User: 591.9 µs, System: 444.6 µs]
  Range (min … max):   363.0 µs … 1120.6 µs    1681 runs
 
  Warning: Command took less than 5 ms to complete. Note that the results might be inaccurate because hyperfine can not calibrate the shell startup time much more precise than this limit. You can try to use the `-N`/`--shell=none` option to disable the shell completely.
 
Benchmark 2: exch toto tata
  Time (mean ± σ):     368.3 µs ± 129.8 µs    [User: 413.5 µs, System: 275.4 µs]
  Range (min … max):    86.4 µs … 875.6 µs    2211 runs
 
  Warning: Command took less than 5 ms to complete. Note that the results might be inaccurate because hyperfine can not calibrate the shell startup time much more precise than this limit. You can try to use the `-N`/`--shell=none` option to disable the shell completely.
 
Summary
  exch toto tata ran
    1.89 ± 0.77 times faster than ./swap-name-rs toto tata

swap-name-rs/target/debug on  main [!?] took 5s 
❯ 
```  
They are literally running two time faster than me :(

So I tried to optimize my code using what is the closest to the exch implementation : [https://github.com/util-linux/util-linux/blob/master/misc-utils/exch.c] (https://github.com/util-linux/util-linux/blob/master/misc-utils/exch.c), which is the actual main.rs, (the old version is main.old lmao) but it didn't help, and they are still faster.  
I guess that trying to make systemcall in rust was dumb anyway, because rust is supposed to be safer than C, not faster.  
