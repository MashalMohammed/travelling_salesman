use core::f32;
use rand::Rng;
use std::time::Instant;

const CITY_COUNT: usize = 6;
const MAP_WIDTH: u16 = 100;

const SHOW_PLOT: bool = true;
const MEASURE_TIMING: bool = true;
const GRAPH_PIXELS: usize = 50;

const IS_DEBUG: bool = false; // this flag has performance cost when true
const SHOW_ALL_TRAVERSALS: bool = false;

// https://tspvis.com/
// https://www.routific.com/
// https://www.math.uwaterloo.ca/tsp/app/diy.html?
// https://getcircuit.com/
// https://www.optaplanner.org/
// https://dspace.mit.edu/handle/1721.1/131235 - Technical Proceedings of the Amazon Last Mile Routing Research Challenge
// https://arxiv.org/abs/2112.15192 - Paper - Constrained Local Search for Last-Mile Routing
// https://www.math.uwaterloo.ca/tsp/amz/index.html
fn main() {
    let points = generate_points(CITY_COUNT, MAP_WIDTH);
    if IS_DEBUG || SHOW_PLOT {
        display_plot(&points);
    }

    let grid = calculate_edge_grid(points);
    if IS_DEBUG {
        display_grid(&grid);
    }

    let min_total_dist = brute_force(grid);

    println!("Optimal path length: {min_total_dist}");
}

// (n-2)!
fn brute_force(grid: Vec<Vec<f32>>) -> f32 {
    let n = grid.len();

    let start: Instant = Instant::now();
    let min = traverse(vec![0], (1..n).collect(), f32::MAX, &grid);
    if MEASURE_TIMING {
        let duration = start.elapsed();
        println!("Took {duration:?}");
    }

    min
}

fn traverse(visited: Vec<usize>, pending: Vec<usize>, mut min: f32, grid: &Vec<Vec<f32>>) -> f32 {
    if !pending.is_empty() {
        // permutation
        for i in 0..pending.len() {
            let mut future_visit = visited.clone();
            future_visit.push(pending[i]);

            let future_available: Vec<usize> = pending
                .iter()
                .filter(|x| **x != pending[i])
                .map(|x| x.to_owned())
                .collect();
            let local_min = traverse(future_visit, future_available, min, &grid);
            if local_min < min {
                min = local_min
            }
        }

        min
    } else {
        let n = visited.len();
        let first_ix = visited[0];
        let last_ix = visited[n - 1];
        let mut total_distance = grid[first_ix][last_ix];
        for i in 0..n - 1 {
            let a_city = visited[i];
            let b_city = visited[i + 1];
            total_distance += grid[a_city][b_city];
        }

        if IS_DEBUG && SHOW_ALL_TRAVERSALS {
            display_path(&visited);
        }
        if total_distance < min {
            min = total_distance;

            if IS_DEBUG {
                if !SHOW_ALL_TRAVERSALS {
                    display_path(&visited);
                }
                println!("\t\t\tNew min: {min}");
            }
        }

        min
    }
}

/// Calculate distances between points, as a grid, where grid[i][j] is the ditsance from city i to city j
fn calculate_edge_grid(points: Vec<Point>) -> Vec<Vec<f32>> {
    let n = points.len();
    let mut grid = vec![vec![0 as f32; n]; n];

    for i in 0..n {
        for j in i + 1..n {
            let dx = points[i].x as i16 - points[j].x as i16;
            let dy = points[i].y as i16 - points[j].y as i16;
            grid[i][j] = ((dx * dx + dy * dy) as f32).sqrt();
            grid[j][i] = grid[i][j];
        }
    }

    grid
}

/// Generates dataset: (x, y) co-ordinates for n cities, in a space of area = width * width
fn generate_points(n: usize, width: u16) -> Vec<Point> {
    let mut points = vec![Point { x: 0, y: 0 }; n];

    for i in 0..n {
        let xi: u16 = rand::thread_rng().gen_range(0..width);
        let yi: u16 = rand::thread_rng().gen_range(0..width);
        points[i].x = xi;
        points[i].y = yi;
    }

    points
}

fn display_grid(grid: &Vec<Vec<f32>>) {
    let n = grid.len();
    println!("\nGrid:");

    print!("\n");
    print!("    ");
    for j in 0..n {
        print!(" {:>5}", j);
    }
    print!("\n");
    print!("    ");
    for _ in 0..n {
        print!("______");
    }
    print!("\n");

    for i in 0..n {
        print!("{:^3} |", i);

        for j in 0..n {
            print!("{0:>5.1} ", grid[i][j]);
        }
        print!("\n");
    }
    print!("\n");
}

fn display_plot(points: &Vec<Point>) {
    let n = points.len();
    let mut plot = vec![vec!["  ".to_owned(); GRAPH_PIXELS]; GRAPH_PIXELS];
    let scale_factor = (MAP_WIDTH / GRAPH_PIXELS as u16) as usize;

    // points
    for i in 0..n {
        let ix = (points[i].x / scale_factor as u16) as usize;
        let iy = (points[i].y / scale_factor as u16) as usize;
        println!(
            "City {i}: ({}, {})        ({}, {}) ",
            points[i].x, points[i].y, ix, iy
        );
        plot[ix][iy] = format!("{i:>2}");
    }

    // plot
    println!("\nPlot:");
    print!("x");
    for _ in 0..GRAPH_PIXELS {
        print!("--");
    }
    print!("x\n");
    for j in 0..GRAPH_PIXELS {
        print!("|");
        for i in 0..GRAPH_PIXELS {
            print!("{}", plot[i][GRAPH_PIXELS - 1 - j]);
        }
        print!("|\n");
    }
    print!("x");
    for _ in 0..GRAPH_PIXELS {
        print!("--");
    }
    print!("x\n");
}

fn display_path(path: &Vec<usize>) {
    let pattern: Vec<String> = path.iter().map(|x| x.to_string()).collect();
    let pattern = pattern.join(" > ");

    println!("path: {pattern} > {}", path[0]);
}

#[derive(Clone)]
struct Point {
    x: u16,
    y: u16,
}
