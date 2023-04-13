use std::{time::Instant, cell};
use rand::{prelude::SliceRandom, Rng};
use raylib::prelude::*;
use clap::Parser;

// Command line arguments // :3
/*
EDIT: just use an external library e.g. -> https://docs.rs/clap/latest/clap/

             Dimensions   w  h  Window Scale    If watching and speed
cargo run -- --dimensions 10 10 -window-scale 5 -watch 5

required dimensions
optional window-scale - needs scale after
optional watch - optional speed afterwards
*/

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Width
    #[arg(long)]
    width: usize,
    // Width
    #[arg(long)]
    height: usize,
    // Window Scale
    #[arg(long, default_value_t = 1)]
    window_scale: i32,
}


fn main() {
    let args = Args::parse();
    let w = args.width;
    let h = args.height;
    let window_scale = args.window_scale;
    let now = Instant::now();

    // Width and height of the maze
    //let w: usize = 50;
    //let h: usize = 50;

    // An array of two vectors for storing the walls of the maze, doesn't store the right/bottom-most walls for efficient indexing
    //                            Vertical                               Horizontal
    let mut walls: [Vec<u8>; 2] = [vec![1; ((h*w)).try_into().unwrap()], vec![1; ((w*h)).try_into().unwrap()]];
    walls[0][0] = 0;

    // Is it more efficient to have 3 different vectors? If I were to have just fronteirs and mazes I'd have to check 
    // for a value not existing in TWO different vectors, as opposed to a value being 0 in just 1, however fronteirs and mazes
    // are necessary, as storing these is a lot quicker than making them each time I need a vector of every maze or fronteir cell

    // Generating
    let mut cellstates: Vec<u8> = vec![0; (w*h).try_into().unwrap()]; // 0 nothing // 1 frontier // 2 maze //
    cellstates[0] = 2; // Set the maze cell
    // Vectors containing indexes of fronteirs and mazes
    let mut frontiers: Vec<usize> = vec![];
    let mut mazes: Vec<usize> = vec![0];

    let mut maze_generated: bool = false;
    let mut r_f_walls: Vec<[usize; 2]> = Vec::with_capacity(4);
    
    //let mut loops = 0;
    //while {
    //    loops+=1;
    //    generation_step(&w, &h, &mut r_f_walls, &mut maze_generated, &mut mazes, &mut frontiers, &mut cellstates, &mut walls);
    //    maze_generated == false
    //}  {}

    println!("Generated maze in {:?}ms!", now.elapsed().as_millis());

    let w_s_6 = window_scale * 6;
    
    let (mut rl, thread) = raylib::init()
        .size(((w as i32 * 6)+1)*window_scale, ((h as i32 * 6)+1)*window_scale)
        .title(&(w.to_string() + " x " + &h.to_string() + " maze"))
        .build();

    while !rl.window_should_close() {

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        
        for _ in 0..25 {
            generation_step(&w, &h, &mut r_f_walls, &mut maze_generated, &mut mazes, &mut frontiers, &mut cellstates, &mut walls);
        }


        // Draw rectangles
        for (index, &v) in cellstates.iter().enumerate() {
            if v == 2 {
                continue;
            }
            d.draw_rectangle((index%w) as i32 *w_s_6, (index/w) as i32 *w_s_6, window_scale*6, window_scale*6, if v == 0 {Color::GRAY} else {Color::RED});
        }

        // Draw walls
        let mut w_type = 0;
        for wv in &walls {
            for (index, &wa) in wv.iter().enumerate() {
                if wa == 0 { continue; }
                let index: i32 = index as i32;
                if w_type == 0 {
                    d.draw_rectangle((index%(w) as i32)*w_s_6, window_scale+(index/(w) as i32)*w_s_6, window_scale, window_scale*5, Color::BLACK);
                } else {
                    d.draw_rectangle(window_scale+(index%(w) as i32)*w_s_6, (index/(w) as i32)*w_s_6, window_scale*5, window_scale, Color::BLACK);
                }
            }
            w_type+=1;
        }
        // Draw side walls
        d.draw_rectangle(0, h as i32*w_s_6, w as i32*w_s_6, window_scale, Color::BLACK); // Bottom
        d.draw_rectangle(w as i32*w_s_6, 0, window_scale, (h-1) as i32*w_s_6, Color::BLACK); // Right

        // Draw dots
        for ih in 0..(h+1) {
            for iw in 0..(w+1) {
                d.draw_rectangle(iw as i32*w_s_6, ih as i32*w_s_6, window_scale, window_scale, Color::BLACK);
            }
        }
    }
}

fn generation_step(w: & usize, h: & usize, r_f_walls: &mut Vec<[usize; 2]>, maze_generated: &mut bool, mazes: &mut Vec<usize>, frontiers: &mut Vec<usize>, cellstates: &mut Vec<u8>, walls: &mut [Vec<u8>; 2]) {
    // Add frontier indexes
    for index in &mut *mazes {
        //// If we're not looking at a part of the maze, continue. 
        //if cellstates[index] != 2 { continue; }
        // TODO optimise
        // For each adjacent cell, if it's valid and currently nothing, make it into a frontier!!
        if (*index % *w) > 0      { if cellstates[*index-1] == 0 { cellstates[*index-1] = 1; frontiers.push(*index-1); }} // Left
        if index < &mut (w*(h-1)) { if cellstates[*index+w] == 0 { cellstates[*index+w] = 1; frontiers.push(*index+w); }} // Down
        if (*index % *w) < w-1    { if cellstates[*index+1] == 0 { cellstates[*index+1] = 1; frontiers.push(*index+1); }} // Right
        if index > &mut(*w-1)     { if cellstates[*index-w] == 0 { cellstates[*index-w] = 1; frontiers.push(*index-w); }} // Up
    } 
    if frontiers.len() == 0 {
        *maze_generated = true;
    } else {
        // Select a random frontier
        let rand_frontier_index = rand::thread_rng().gen_range(0..frontiers.len());
        //let rand_frontier_index: usize = 0;
        let rand_frontier = frontiers[rand_frontier_index];

        // Get all of its neighbouring walls into a vector, but only if they border a maze!! (And if they're in bounds)
        if (rand_frontier % w) > 0   { if cellstates[rand_frontier-1] == 2 { r_f_walls.push( [0, rand_frontier] ); }} // Left
        if rand_frontier < (w*(h-1)) { if cellstates[rand_frontier+w] == 2 { r_f_walls.push( [1, rand_frontier+w] ); }} // Down
        if (rand_frontier % w) < w-1 { if cellstates[rand_frontier+1] == 2 { r_f_walls.push( [0, rand_frontier+1] ); }} // Right
        if rand_frontier > w-1       { if cellstates[rand_frontier-w] == 2 { r_f_walls.push( [1, rand_frontier] ); }} // Up
        // Select a random wall and remove it!!
        let rand_wall = *r_f_walls.choose(&mut rand::thread_rng()).unwrap();

        walls[rand_wall[0]][rand_wall[1]] = 0;
        // Make the frontier a maze cell
        cellstates[rand_frontier] = 2; 

        mazes.push(rand_frontier);
        // Remove the frontier index from frontiers
        frontiers.remove(rand_frontier_index);

        r_f_walls.clear();
    }
}