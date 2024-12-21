use crate::day21::instructions::{
    instructions_to_code, instructions_to_debug_code, parse_instructions_str, Instruction,
};
use crate::day21::keypads::{DirectionalKeypad, KeyPad, NumericKeypad};
use crate::day21::model::code_to_value;
use crate::toolbox::Direction;

pub fn solve_part_one(codes: &[String]) -> i64 {
    // Solve
    println!("Codes: {codes:?}");

    let numeric_keypad: NumericKeypad = NumericKeypad::new();
    let directional_keypad: DirectionalKeypad = DirectionalKeypad::new();

    let result: Vec<i64> = codes
        .iter()
        .map(|code| {
            println!("\nCode '{code}'");
            let final_instructions: Vec<Instruction> = build_code_instructions_part_one(
                code,
                &numeric_keypad,
                &directional_keypad,
                &directional_keypad,
            );
            println!(
                "=> {} ({})",
                instructions_to_code(&final_instructions),
                final_instructions.len()
            );
            println!(
                "{} * {} = {}",
                final_instructions.len(),
                code_to_value(code),
                final_instructions.len() as i64 * code_to_value(code)
            );
            final_instructions.len() as i64 * code_to_value(code)
        })
        .collect();

    result.into_iter().sum()
}

fn build_code_instructions_part_one(
    code: &str,
    depressurized_numeric_keypad: &NumericKeypad,
    radiated_directional_keypad: &DirectionalKeypad,
    frozen_directional_keypad: &DirectionalKeypad,
) -> Vec<Instruction> {
    // Step 1 (depressurized)
    let instructions_on_depressurized: Vec<Instruction> =
        depressurized_numeric_keypad.build_instructions_for_code(code);
    // println!(
    //     "(1) {code} => {}",
    //     instructions_to_debug_code(&instructions_on_depressurized)
    // );
    debug_assert_eq!(
        depressurized_numeric_keypad.build_code_from_instructions(&instructions_on_depressurized),
        code,
        "Depressurized sanity check failed",
    );

    // Step 2 (radiated)
    let step2_code: String = instructions_to_code(&instructions_on_depressurized);
    let instructions_on_radiated: Vec<Instruction> =
        radiated_directional_keypad.build_instructions_for_code(&step2_code);
    // println!(
    //     "(2) {step2_code} => {}",
    //     instructions_to_debug_code(&instructions_on_radiated)
    // );
    debug_assert_eq!(
        radiated_directional_keypad.build_code_from_instructions(&instructions_on_radiated),
        step2_code,
        "Radiated sanity check failed",
    );

    // Step 3 (frozen)
    let step3_code: String = instructions_to_code(&instructions_on_radiated);
    let instructions_on_frozen: Vec<Instruction> =
        frozen_directional_keypad.build_instructions_for_code(&step3_code);
    // println!(
    //     "(3) {step3_code} => {}",
    //     instructions_to_debug_code(&instructions_on_frozen)
    // );
    debug_assert_eq!(
        frozen_directional_keypad.build_code_from_instructions(&instructions_on_frozen),
        step3_code,
        "Frozen sanity check failed",
    );

    instructions_on_frozen
}

#[allow(dead_code)]
fn execute_part_one_instructions(instructions: &str) {
    let depressurized_numeric_keypad: NumericKeypad = NumericKeypad::new();
    let radiated_directional_keypad: DirectionalKeypad = DirectionalKeypad::new();
    let frozen_directional_keypad: DirectionalKeypad = DirectionalKeypad::new();

    println!("Input instructions: '{instructions}'");
    let frozen_instructions: String = frozen_directional_keypad
        .build_code_from_instructions(&parse_instructions_str(&instructions));
    println!("Frozen instructions: '{frozen_instructions}'");
    let radiated_instructions: String = radiated_directional_keypad
        .build_code_from_instructions(&parse_instructions_str(&frozen_instructions));
    println!("Radiated instructions: '{radiated_instructions}'");
    let depressurized_instructions: String = depressurized_numeric_keypad
        .build_code_from_instructions(&parse_instructions_str(&radiated_instructions));

    println!("Final: '{depressurized_instructions}'");
}
