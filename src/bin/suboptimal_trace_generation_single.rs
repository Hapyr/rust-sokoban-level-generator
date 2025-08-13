use sokoban_level_generator::{SokobanGame, generate_level};
use pathfinding::prelude::astar;

fn main() {
    let level = generate_level(2, 2, 1);
    let game = SokobanGame::new(level, 100);

    for i in 0..3 {
        println!("Suboptimal trace {} of 3", i + 1);
        // use astar for solving the level
        let path = astar(
            &game,
            |p| p.get_successor(true, 0.5, true, true, 0.2),
            |p| p.distance_to_goal(),
            |p| p.is_solved(),
        );
        
        if let Some((solution_path, _cost)) = path {
            for state in solution_path.iter() {
                println!("\nAction: {:?}", state.previous_action);
                state.print_pretty();
            }
        }

    }


}
