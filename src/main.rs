
// Disable warning on unused function/method; we have duplicates with different
// implementations for performance investigations
#![allow(dead_code)]

extern crate rayon;

mod logic;
mod prng;

use logic::*;
use rayon::prelude::*;
use std::env;

#[derive(Debug, PartialEq)]
enum RunSafety {
    AllSafe,
    SomeSafe,
    NoSafe,
}

fn solve_all_stacks_par(ss : &mut Vec<SliceStack>, run_safety : RunSafety) {
    match run_safety {
        RunSafety::AllSafe => ss.par_iter_mut() .for_each(|s| solve_stack_all_safe(s)),
        RunSafety::SomeSafe => ss.par_iter_mut() .for_each(|s| solve_stack_some_safe(s)),
        RunSafety::NoSafe => ss.par_iter_mut() .for_each(|s| solve_stack_no_safe(s))
    }
}

fn solve_all_stacks(ss : &mut Vec<SliceStack>, run_safety : RunSafety) {
    match run_safety {
        RunSafety::AllSafe => for s in &mut ss[..] { solve_stack_all_safe(s) },
        RunSafety::SomeSafe => for s in &mut ss[..] { solve_stack_some_safe(s) },
        RunSafety::NoSafe => for s in &mut ss[..] { solve_stack_no_safe(s) }
    }
}

fn solve_stack_all_safe(s : &mut SliceStack) {
    let mut direction : i32;
    let search_dir : i32 = 0;
    while !s.iscomplete() {
        direction = 0;
        let mut index = s.find_single_joining_move(&mut direction);
        if index == -1 { index = s.find_first_double_move(search_dir, &mut direction); }
        //assert!(index != -1);
        index += direction;
        s.flip(index, direction);
        //search_dir = !search_dir;
    }
}

fn solve_stack_some_safe(s : &mut SliceStack) {
    let mut direction : i32;
    let search_dir : i32 = 0;
    while !s.iscomplete_unsafe() {
        direction = 0;
        let mut index = s.find_single_joining_move_unchecked(&mut direction);
        if index == -1 { index = s.find_first_double_move(search_dir, &mut direction); }
        //assert!(index != -1);
        index += direction;
        s.flip_rsslice(index, direction);
        //search_dir = !search_dir;
    }
}

fn solve_stack_no_safe(s : &mut SliceStack) {
    let mut direction : i32;
    let search_dir : i32 = 0;
    while !s.iscomplete_unsafe() {
        direction = 0;
        let mut index = s.find_single_joining_move_rawptr(&mut direction);
        if index == -1 { index = s.find_first_double_move_unsafe(search_dir, &mut direction); }
        //assert!(index != -1);
        index += direction;
        //s.flip_rsslice(index, direction);
        s.flip_unsafe(index, direction);
        //search_dir = !search_dir;
    }
}

const SLICE_COUNT : i32 = MAX_SLICES as i32;
const COLOR_COUNT : i32 = 8;
const SS_COUNT : usize = 1024*1024;

fn big_run(run_safety : RunSafety) {
    let mut ss = vec![SliceStack::new(); SS_COUNT];
    for s in ss.iter_mut() {
        s.init_unsafe(SLICE_COUNT, COLOR_COUNT);
    }
    solve_all_stacks(&mut ss, run_safety);
}

fn main() {
    let run_safety : RunSafety =
        match env::args().nth(1) {
            Some(arg1) =>
                match arg1.parse() {
                    Ok(0) => RunSafety::NoSafe,
                    Ok(1) => RunSafety::SomeSafe,
                    Ok(2) => RunSafety::AllSafe,
                    _ => RunSafety::NoSafe },
            _ => RunSafety::NoSafe
        };

    big_run(run_safety);
}

/* vim: set ts=4 sts=4 sw=4 et : */
