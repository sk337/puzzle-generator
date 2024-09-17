use ctrlc;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use puzzle_generator::{get_similarity_score, PuzzleGen, Solution, SIZE};
use speedy::{Readable, Writable};
use std::fs;
use std::io::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

static TRESHOLD: f32 = 0.25;
static EDGE_COUNT: u8 = 6;
static SUFFIX_COMPRESSED: &str = ".puzz.gz";
static SUFFIX: &str = ".puzz";
static COMPRESS: bool = true;

fn main() {
    if fs::read_dir("puzzles").is_err() {
        fs::create_dir("puzzles").unwrap();
    }

    let puzzles = get_puzzles_from_dir();

    println!("Puzzles: {}", puzzles.len());

    let solution = get_puzzle_with_score(TRESHOLD);

    if COMPRESS {
        write_to_file_compressed(&solution);
    } else {
        write_to_file(&solution);
    }
}

fn write_to_file_compressed(solution: &Solution) {
    let data = solution.write_to_vec().unwrap();
    let mut file = fs::File::create(format!(
        "puzzles/puzzle_{}_{}_{}_{}_{SIZE}x{SIZE}{SUFFIX_COMPRESSED}",
        solution.score, solution.tries, solution.treshold, solution.edge_count
    ))
    .unwrap();
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&data).unwrap();
    let compressed_data = encoder.finish().unwrap();
    file.write_all(&compressed_data).unwrap();
}

fn write_to_file(solution: &Solution) {
    let data = solution.write_to_vec().unwrap();
    let mut file = fs::File::create(format!(
        "puzzles/puzzle_{}_{}_{}_{}_{SIZE}x{SIZE}{SUFFIX}",
        solution.score, solution.tries, solution.treshold, solution.edge_count
    ))
    .unwrap();
    file.write_all(&data).unwrap();
}

fn load_puzzle_from_file(file: String, compressed: bool) -> Solution {
    let data = fs::read(file).unwrap();
    if compressed {
        let mut decoder = ZlibDecoder::new(&data[..]);
        let mut buffer: Vec<u8> = Vec::new();
        decoder.read_to_end(&mut buffer).unwrap();
        Solution::read_from_buffer(&buffer).unwrap()
    } else {
        Solution::read_from_buffer(&data).unwrap()
    }
}

fn get_puzzles_from_dir() -> Vec<Solution> {
    let mut puzzles: Vec<Solution> = Vec::new();
    for entry in fs::read_dir("puzzles").unwrap() {
        if entry.is_err() {
            println!("Error reading file: {:#?}", entry);
            continue;
        }
        let entry = entry.unwrap();
        let path = entry.path();
        let file = path.to_str().unwrap().to_string();
        if !file.contains(format!("{SIZE}x{SIZE}").as_str()) {
            continue;
        }
        if file.ends_with(SUFFIX) {
            puzzles.push(load_puzzle_from_file(file, false));
        } else if file.ends_with(SUFFIX_COMPRESSED) {
            puzzles.push(load_puzzle_from_file(file, true));
        }
    }
    puzzles
}

fn get_puzzle_with_score(threshold: f32) -> Solution {
    let terminated = Arc::new(AtomicBool::new(false));
    let terminated_clone = terminated.clone();

    ctrlc::set_handler(move || {
        terminated_clone.store(true, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl+C handler");

    let mut pg = PuzzleGen::new(EDGE_COUNT);
    let mut cont = true;
    let mut tries: u128 = 0;
    let mut closest = 1.0;

    while cont {
        pg.new_puzzle();
        pg.solve();

        if pg.solutions.len() <= 1 {
            tries += 1;
            if tries % 10_000 == 0 {
                println!("Tries: {}. Closest: {}", tries, closest);
            }
            if terminated.load(Ordering::SeqCst) {
                let best_solution = Solution {
                    score: closest,
                    tries,
                    edge_count: EDGE_COUNT,
                    treshold: threshold,
                    values: pg.solutions.clone(),
                };
                println!("Safely Terminated at Ctrl+C");
                return best_solution;
            }
            continue;
        }
        let score = get_similarity_score(&pg.solutions[0], &pg.solutions[1]);
        cont = pg.solutions.len() <= 1 || !pg.check_solutions() || score > threshold;
        if score < closest {
            closest = score;
        }
        tries += 1;
    }

    Solution {
        score: get_similarity_score(&pg.solutions[0], &pg.solutions[1]),
        tries,
        edge_count: EDGE_COUNT,
        treshold: threshold,
        values: pg.solutions.clone(),
    }
}
