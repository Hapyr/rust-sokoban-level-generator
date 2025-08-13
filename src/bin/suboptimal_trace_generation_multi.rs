use sokoban_level_generator::{SokobanGame, generate_level};
use pathfinding::prelude::astar;
use std::io::Write;
use rayon::prelude::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of threads to use for parallel processing
    #[arg(short, long, default_value_t = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(8))]
    threads: usize,

    /// Number of levels to generate
    #[arg(short, long, default_value_t = 10)]
    levels: usize,

    /// Number of A* search iterations per level
    #[arg(short, long, default_value_t = 3)]
    iterations: usize,
}

fn process_level(level_id: usize, iterations: usize) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let level = generate_level(2, 2, 1);
    let game = SokobanGame::new(level, 30);

    // Create directory if it doesn't exist (this is thread-safe)
    std::fs::create_dir_all("game_states")?;
    
    // Create file with game state (unique filename per level)
    let filename = format!("game_states/game_{}.txt", level_id);
    let mut file = std::fs::File::create(&filename)?;

    // Print to console (this might be interleaved between threads, but that's okay for debug)
    println!("Processing level {} on thread {:?}", level_id, std::thread::current().id());
    
    for q in 0..iterations {
        // Solve with A* search
        let path = astar(
            &game,
            |p| p.get_successor(true, 0.5, true, true, 0.2),
            |p| p.distance_to_goal(),
            |p| p.is_solved(),
        );
        
        
        if let Some((solution_path, _cost)) = path {
            if solution_path.len() == 1 {
                writeln!(file, "--{}--", q)?;
            }else{
                writeln!(file, "__{}__", q)?;
            }
            
            for state in solution_path.iter() {
                if let Some(action) = &state.previous_action {
                    writeln!(file, "{:?}", action)?;
                }
                writeln!(file, "{}", state.board.get_one_line_string())?;
            }
        } else {
            writeln!(file, "++{}++ - No solution found", q)?;
        }
    }
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    // For each level, 1 or more suboptimal traces get generated.
    // All traces for one level are stored in the same file.
    // The file is named game_{level_id}.txt
    // A state is encoded in a one line string.
    // Each trace get a heading: 
    // - If the heading is formatted like --{}--, it is a already solved instance. 
    // - If the heading is formatted like __{}__, it is a suboptimal trace.
    // - If the heading is formatted like ++{}++, it is a failed attempt.
    // The number is the iteration number.
    
    let args = Args::parse();
    
    println!("Starting Sokoban level generator with {} threads", args.threads);
    println!("Generating {} levels with {} iterations each", args.levels, args.iterations);
    
    // Configure rayon thread pool
    rayon::ThreadPoolBuilder::new()
        .num_threads(args.threads)
        .build_global()?;
    
    // Process levels in parallel
    let start_time = std::time::Instant::now();
    
    let results: Vec<_> = (0..args.levels)
        .into_par_iter()
        .map(|i| process_level(i, args.iterations))
        .collect();
    
    // Check for any errors
    let mut error_count = 0;
    for (i, result) in results.iter().enumerate() {
        if let Err(e) = result {
            eprintln!("Error processing level {}: {}", i, e);
            error_count += 1;
        }
    }
    
    let elapsed = start_time.elapsed();
    println!("Completed processing {} levels in {:?}", args.levels, elapsed);
    
    if error_count > 0 {
        println!("Warning: {} levels failed to process", error_count);
    } else {
        println!("All levels processed successfully!");
    }
    
    Ok(())
}
