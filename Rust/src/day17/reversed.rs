// Decompiled program
// START
//   B = A &  7  (x 3?)
//   B = B ^  3
//   C = A >> B  (x 2**B)
//   B = B ^  C
//   B = B ^  3
//   A = A >> 3  (x 8)
//   print(B)
//   <jump to 0>
// END

use std::collections::VecDeque;
use faer::iter::ColIterMut;
use crate::day17::model::Computer;

/// We know that we got out of the loop: A = 0
/// B was just printed: B = last_val = 0
/// C is unknown (lets consider only the last 3 bits)
pub fn solve_part_two(computer: &Computer) -> u32 {
    let mut target_output: Vec<u8> = computer.stack.clone();
    target_output.reverse();
    let target_str: String = Computer::format_stdout(&target_output);

    // println!("Target output: {}", target_str);

    let mut queue: VecDeque<Computer> = VecDeque::with_capacity(2 << 15);
    let mut results: Vec<u32> = Vec::new();

    // Initialize queue
    for i in 0..(1 << 6) {
        queue.push_back(computer.clone_for_reverse(0, 0, i))
    }
    while let Some(mut current_computer) = queue.pop_front() {
        // println!("\n[Loop] (queue len={}) {current_computer:?}", queue.len());
        // current_computer.show_next_reverse_operation();

        let current_stdout_str: String = Computer::format_stdout(&current_computer.stdout);

        // Stop condition
        if current_stdout_str.len() > target_str.len() && current_stdout_str.starts_with(&target_str) {
            // println!(" !!! Candidate !!!");
            results.push(current_computer.a);
            continue
        }

        // Filter out invalid computers
        if !target_str.starts_with(&current_stdout_str) {
            // println!("... skipped");
            continue
        }

        match current_computer.run_reverse_operation() {
            None => {
                // println!("Pushed back in queue");
                queue.push_front(current_computer)
            },
            Some(poss_vec) => {
                // println!("Pushing {} in queue (2**{})", poss_vec.len(), poss_vec.len().ilog2());
                for poss in poss_vec {
                    queue.push_front(poss);
                }
            }
        }
    }

    // println!("Target output: {}", target_str);

    *results.iter().min().unwrap()
}
