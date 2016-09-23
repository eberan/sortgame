
extern crate rayon;

mod logic;
mod prng;

use logic::*;

use rayon::prelude::*;
fn solve_all_stacks_par(ss : &mut Vec<SliceStack>) {
    ss.par_iter_mut()
        .for_each(|s| solve_stack(s));
}

fn solve_all_stacks(ss : &mut Vec<SliceStack>) {
    for s in &mut ss[..] {
        solve_stack(s);
    }
}

fn solve_stack(s : &mut SliceStack) {
    let mut direction : i32;
    let mut search_dir : i32 = 0;
    while !ss_iscomplete_unsafe(s) {
        direction = 0;
        let mut index = ss_find_single_joining_move(s, &mut direction);
        if index == -1 { index = ss_find_first_double_move(s, search_dir, &mut direction); }
        //assert!(index != -1);
        index += direction;
        //ss_flip_rsslice(s, index, direction);
        ss_flip(s, index, direction);
        //search_dir = !search_dir;
    }
}

fn main() {
    const SLICE_COUNT : i32 = MAX_SLICES as i32;
    const COLOR_COUNT : i32 = 8;
    let mut search_dir : i32 = -1;
    let mut direction : i32;

    const SS_COUNT : usize = 1024*1024;
    let mut ss = vec![SliceStack::new(); SS_COUNT];


    for s in ss.iter_mut() {
        ss_init_unsafe(s, SLICE_COUNT, COLOR_COUNT);
    }

    //solve_all_stacks_par(&mut ss);
    solve_all_stacks(&mut ss);
}

/* vim: set ts=4 sts=4 sw=4 et : */
