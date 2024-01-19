use std::cmp::{max, min};
use crate::chessboard::chessboard::Chessboard;
use crate::chessboard::player::PlayerColor;
use crate::evaluator::evaluate;
use crate::min_max::next_move_generator::generate_next_moves;
use ocl::ProQue;
use ocl::SpatialDims;
const DEPTH: usize = 4;

fn min_max_with_alpha_beta_pruning(
    state: &Chessboard,
    depth: usize,
    mut alpha: i64,
    mut beta: i64,
    player_color: PlayerColor
) -> i64 {

    if depth == 0 || state.is_finished() {
        return evaluate(&state, player_color);
    }

    let possible_moves = generate_next_moves(&state, player_color);

    match player_color {
        PlayerColor::White => {
            let mut value = i64::MIN;
            for next_move in possible_moves {
                let mut next_state = state.clone();

                let from = Chessboard::convert_index_to_square(next_move.0);
                let to = Chessboard::convert_index_to_square(next_move.1);
                next_state.perform_move(&from, &to, PlayerColor::White).unwrap();

                value = max(
                    value,
                    min_max_with_alpha_beta_pruning(&next_state, depth - 1, alpha, beta, PlayerColor::Black)
                );

                if value > beta {
                    break;
                }

                alpha = max(alpha, value)
            }
            value
        },
        PlayerColor::Black => {
            let mut value = i64::MAX;
            for next_move in possible_moves {
                let mut next_state = state.clone();

                let from = Chessboard::convert_index_to_square(next_move.0);
                let to = Chessboard::convert_index_to_square(next_move.1);
                next_state.perform_move(&from, &to, PlayerColor::Black).unwrap();

                value = min(
                    value,
                    min_max_with_alpha_beta_pruning(&next_state, depth - 1, alpha, beta, PlayerColor::White)
                );

                if value < alpha {
                    break;
                }

                beta = min(beta, value)
            }
            value
        }
    }
}

pub fn get_best_move(state: &Chessboard) -> (u64, u64) {
    let mut result = (0, 0);
    let possible_moves = generate_next_moves(&state, PlayerColor::White);
    let possible_moves_copy = possible_moves.clone();
    // let mut value = i64::MIN;
    let mut values: Vec<i64> = Vec::new(); // Vector to store min-max values

    for next_move in possible_moves {
        let mut next_state = state.clone();

        let from = Chessboard::convert_index_to_square(next_move.0);
        let to = Chessboard::convert_index_to_square(next_move.1);
        next_state.perform_move(&from, &to, PlayerColor::White).unwrap();

        let min_max_value = min_max_with_alpha_beta_pruning(&next_state, DEPTH, i64::MIN, i64::MAX, PlayerColor::Black);
        values.push(min_max_value);
        // if min_max_value > value {
        //     value = min_max_value;
        //     result = next_move;
        // }
    }
    let values_as_slice: &[f32] = &values.iter().map(|&x| x as f32).collect::<Vec<f32>>();
    // println!("Original values: {:?}", values);
    // println!("{:?}", values_as_slice);
    // println!("Result: {:?}", value);

    if let Ok(result) = find_minimum_with_array(values_as_slice) {
        // Print the minimum value
        println!("Minimum value: {:?}", result);

        for next_move in possible_moves_copy {
            let mut next_state = state.clone();
    
            let from = Chessboard::convert_index_to_square(next_move.0);
            let to = Chessboard::convert_index_to_square(next_move.1);
            next_state.perform_move(&from, &to, PlayerColor::White).unwrap();
    
            let min_max_value = min_max_with_alpha_beta_pruning(&next_state, DEPTH, i64::MIN, i64::MAX, PlayerColor::Black);
            values.push(min_max_value);
            if min_max_value as f32 == result {
                return next_move;
            }
        }
    } else {
        println!("Error occurred during computation.");
    }

    result
}
  
fn trivial() -> ocl::Result<()> {
    let src = r#"
        __kernel void add(__global float* buffer, float scalar) {
            buffer[get_global_id(0)] += scalar;
        }
    "#;

    let pro_que = ProQue::builder()
        .src(src)
        .dims(1 << 20)
        .build()?;

    let buffer = pro_que.create_buffer::<f32>()?;

    let kernel = pro_que.kernel_builder("add")
        .arg(&buffer)
        .arg(10.0f32)
        .build()?;

    unsafe { kernel.enq()?; }

    let mut vec = vec![0.0f32; buffer.len()];
    buffer.read(&mut vec).enq()?;

    println!("The value at index [{}] is now '{}'!", 200007, vec[200007]);
    Ok(())
}

fn find_minimum_with_array(input_array: &[f32]) -> ocl::Result<f32> {
    let src = r#"
        __kernel void find_minimum(__global float* buffer, __global const float* input_array, uint array_size) {
            int gid = get_global_id(0);
            float current_val = buffer[gid];
            float array_val = (gid < array_size) ? input_array[gid] : current_val; // Access input_array safely
            buffer[gid] = (current_val < array_val) ? current_val : array_val;
        }
    "#;

    // Create an OpenCL context, program, and queue
    let pro_que = ProQue::builder()
        .src(src)
        .dims(1 << 20)  // Set the global work size (number of work-items)
        .build()?;

    // Create a buffer on the GPU to store floating-point values
    let buffer = pro_que.create_buffer::<f32>()?;

    // Create a buffer on the GPU to store the input array
    let input_buffer = pro_que.create_buffer::<f32>()?;
    input_buffer.write(input_array).enq()?; // Write input array to buffer

    // Build the OpenCL kernel with the specified source and arguments
    let kernel = pro_que.kernel_builder("find_minimum")
        .arg(&buffer)            // Argument: GPU buffer
        .arg(&input_buffer)      // Argument: GPU buffer for input array
        .arg(&(input_array.len() as u32)) // Argument: Size of the input array
        .build()?;

    // Enqueue the kernel for execution on the GPU
    unsafe { kernel.enq()?; }

    let mut vec = vec![0.0f32; buffer.len()];
    buffer.read(&mut vec).enq()?;

    let min_val = vec.iter().cloned().fold(std::f32::INFINITY, f32::min);

    println!("The minimum value in the buffer is '{}'", min_val);

    Ok(min_val)
}
