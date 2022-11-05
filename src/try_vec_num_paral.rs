/*
    testing multithreading in rust

*/

use rayon::prelude::*;

fn pvstuff(v: &Vec<f32>) -> Vec<f32> {
    v.par_iter()
        .map(|x| x + 1.0)
        .map(|x| x + 5.0)
        .map(|x| x / 30.0)
        .collect()
}

fn vstuff(v: &Vec<f32>) -> Vec<f32> {
    v.iter()
        .map(|x| x + 1.0)
        .map(|x| x + 5.0)
        .map(|x| x / 30.0)
        .collect()
}

// timeit should take a function closure and time it
fn timeit(f: &dyn Fn() -> Vec<f32>) -> f32 {
    let start = std::time::Instant::now();
    let _ = f();
    let end = std::time::Instant::now();
    let dur = end.duration_since(start);
    dur.as_secs_f32()
}

fn main() {
    let num: usize = 2_000_000_000;

    let mut nums = vec![];
    for i in 0..num {
        nums.push(i as f32);
    }

    // let vstuff_closure = || vstuff(num);
    // let pvstuff_closure = || pvstuff(num);
    // let duration = timeit(vstuff_closure);
    // let duration = timeit(pvstuff_closure);

    let start = std::time::Instant::now();
    let nums = vstuff(&nums);
    let end = std::time::Instant::now();
    let duration = end.duration_since(start);
    println!("Time elapsed in expensive_function() is: {:?}", duration);

    let start = std::time::Instant::now();
    let nums = pvstuff(&nums);
    let end = std::time::Instant::now();
    let duration = end.duration_since(start);
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    let _dnums = nums;
}
