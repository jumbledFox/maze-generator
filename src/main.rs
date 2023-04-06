use std::cell;
use std::{time::Instant};
use rand::prelude::SliceRandom;
use rand::Rng;
use raylib::prelude::*;

//let m: Vec<u8> = vec![0b1010, 0b1100, 0b1101, 0b1101, 0b1101, 0b1101, 0b0011, 0b0000, 0b0010, 0b0110, 0b0011, 0b1000, 0b0010, 0b1000, 0b1100, 0b1001, 0b0110, 0b1101, 0b0101, 0b0111, 0b0011, 0b1110, 0b0011, 0b0010, 0b1010];

fn main() {
    let now = Instant::now();

    let w: usize = 100;
    let h: usize = 100;
    let mut walls: Vec<u8> = vec![1; (((w+1)*h)+((h+1)*w)).try_into().unwrap()];
    //                             Vertical                                   Horizontal
    let mut _walls: [Vec<u8>; 2] = [vec![1; (((w+1)*h)).try_into().unwrap()], vec![1; (((h+1)*w)).try_into().unwrap()]];

    // Generating
    let mut cellstates: Vec<u8> = vec![0; (w*h).try_into().unwrap()]; // 0 nothing // 1 frontier // 2 maze //
    cellstates[0] = 2; // Set a maze cell
    let mut frontiers: Vec<usize> = vec![];

    while {

        // Condition for semi-hacky do while loop syntax
        cellstates.contains(&1)
    } { }
    println!("{}", now.elapsed().as_millis());

    //walls[0] = 0;
    //let wl = walls.len()-1;
    //walls[wl] = 0;

    
    //let walls: Vec<u8> = vec![1,1,0,1,0,1, 1,0,1,0,1,1, 1,0,0,1,0,1, 1,0,1,0,0,1, 1,0,1,0,1,1,  1,0,1,1,1,1, 1,1,0,0,0,1, 1,0,0,0,0,1, 1,0,1,1,1,1, 1,0,1,0,0,1];

    let window_scale = 1;
    let w_s_6 = window_scale * 6;

    let (mut rl, thread) = raylib::init()
        .size(((w as i32 * 6)+1)*window_scale, ((h as i32 * 6)+1)*window_scale)
        .title("Maze")
        .build();
    
    let mut stopgen: bool = false;

    while !rl.window_should_close() {
        
        if !stopgen {
            // Add frontier indexes
            for index in 0..cellstates.len() {
                // If we're not looking at a part of the maze, continue. 
                if cellstates[index] != 2 { continue; }
                // TODO optimise
                // For each adjacent cell, if it's valid and currently nothing, make it into a frontier!!
                if (index % w) > 0   { if cellstates[index-1] == 0 { cellstates[index-1] = 1; frontiers.push(index-1); }} // Left
                if index < (w*(h-1)) { if cellstates[index+w] == 0 { cellstates[index+w] = 1; frontiers.push(index+w); }} // Down
                if (index % w) < w-1 { if cellstates[index+1] == 0 { cellstates[index+1] = 1; frontiers.push(index+1); }} // Right
                if index > w-1       { if cellstates[index-w] == 0 { cellstates[index-w] = 1; frontiers.push(index-w); }} // Up
            }
            if !cellstates.contains(&1) {
                stopgen = true;
            }
            if !stopgen {
                // Select a random frontier
                /*// Generate a vector of indexes of every frontier - Credits: TheRoboMan https://discord.com/channels/976120247427948564/1004336932760866958/1092840641681166367
                let frontiers: Vec<usize> = cellstates
                .iter()
                .enumerate()
                .flat_map(|(i, &n)| (n == 1).then(|| i))
                .collect();*/
                // Now select a random one out of those frontiers
                let rand_frontier_index = rand::thread_rng().gen_range(0..frontiers.len());
                //let rand_frontier_index: usize = 0;
                let rand_frontier = frontiers[rand_frontier_index];

                // Get all of its neighbouring walls into a vector, but only if they border a maze!! (And if they're in bounds)
                let mut r_f_walls: Vec<[usize; 2]> = Vec::with_capacity(4);
                if (rand_frontier % w) > 0   { if cellstates[rand_frontier-1] == 2 { r_f_walls.push( [1, rand_frontier] ); }} // Left
                if rand_frontier < (w*(h-1)) { if cellstates[rand_frontier+w] == 2 { r_f_walls.push( [0, rand_frontier+1] ); }} // Down
                if (rand_frontier % w) < w-1 { if cellstates[rand_frontier+1] == 2 { r_f_walls.push( [1, rand_frontier+1] ); }} // Right
                if rand_frontier > w-1       { if cellstates[rand_frontier-w] == 2 { r_f_walls.push( [0, rand_frontier] ); }} // Up
                // Select a random wall and remove it!!
                let rand_wall = *r_f_walls.choose(&mut rand::thread_rng()).unwrap();
                _walls[rand_wall[0]][rand_wall[1]] = 0;
                //walls[rand_wall] = 0;
                // Make the frontier a maze cell
                cellstates[rand_frontier] = 2; 
                // Remove the frontier index from frontiers
                frontiers.remove(rand_frontier_index);
            }
        }
        

        let mut d = rl.begin_drawing(&thread);
         
        d.clear_background(Color::WHITE);

        for (index, &v) in cellstates.iter().enumerate() {
            if v == 2 {
                continue;
            }
            d.draw_rectangle((index%w) as i32 *w_s_6, (index/w) as i32 *w_s_6, window_scale*6, window_scale*6, if v == 0 {Color::GRAY} else {Color::RED});
        }
        /*
        for (index, &wa) in walls.iter().enumerate() {
            // Make it only draw if the wall is there
            if wa == 0 { continue; }
            if index < ((h+1)*w) {
                let index: i32 = index as i32;
                d.draw_rectangle((index/(h+1) as i32)*6*window_scale+window_scale, ((index%(h+1) as i32)*6)*window_scale, window_scale*5, window_scale, Color::BLACK);
            } else {
                let index: i32 = (index-((h+1)*w)) as i32;
                d.draw_rectangle((index%(w+1) as i32)*6*window_scale, (index/(w+1) as i32)*6*window_scale+window_scale,  window_scale, window_scale*5, Color::BLACK);
            }
        }*/
        // TODO - DRAWING FOR NEW ARRANGEMENT
        let mut w_type = 0;
        for wv in &_walls {
            for (index, &wa) in wv.iter().enumerate() {
                if wa == 0 { continue; }
                let index: i32 = index as i32;

                d.draw_rectangle((index/(h+1) as i32)*6*window_scale+window_scale, ((index%(h+1) as i32)*6)*window_scale, window_scale*5, window_scale, Color::BLACK);
            }
            w_type+=1;
        }
        for ih in 0..(h+1) {
            for iw in 0..(w+1) {
                d.draw_rectangle(iw as i32*w_s_6, ih as i32*w_s_6, window_scale, window_scale, Color::BLACK);
            }
        }
        //d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}

    /*
    let mut i = 0;
    for y in 0..h {
        for x in 0..w {
            print!("{:?} ", cellstates[i]);
            i+=1;
        }
        print!("\n")
    }

    println!("{:?}", cellstates);

    let wall_ascii = ["  ", "██", "░░", "▓▓"];

    for (i, &wa) in walls.iter().enumerate() {
        print!("{} ", wa as usize);
        if i % (w+1) == w {
            print!(", ");
        }
        if i+1 == walls.len()/2 {
            println!("New: \n");
        }
    }

    for i in 0..h+1 {
        let mut upwalls = String::new();
        let mut leftwalls = String::new();
        for j in 0..w+1 {
            if j != w {
                //let upwall = m[]
            }
        }
    }
    */

    //println!("frontier count: {}", );

    // Old code
    /*
    let mut maze: Vec<Coord> = vec![];
    let mut frontier: Vec<Coord> = vec![];

    maze.push([0, 0]);
    loop {
        break;
        // Add to frontier
        for coord in &maze {
            for i in get_neighbour_coords(*coord, w, h) {
                if !frontier.contains(&i) && !maze.contains(&i) {
                    frontier.push(i);
                } 
            }
        }
        if frontier.len() == 0{
            break;
        }
        // Select a random frontier
        let selected_frontier = rand::thread_rng().gen_range(0..frontier.len()) as usize;
        // Get a vector of all of the neighbours of the selected frontier
        let frontier_neighbour_coords = get_neighbour_coords(frontier[selected_frontier], w, h);
        let mut frontier_neighbours: Vec<Direction> = vec![];
        for fnc in frontier_neighbour_coords {
            if maze.contains(&fnc) {
                if fnc[0] == frontier[selected_frontier][0] {
                    if fnc[1] > frontier[selected_frontier][1] {
                        frontier_neighbours.push(Direction::Down);
                    } else {
                        frontier_neighbours.push(Direction::Up);
                    }
                } else {
                    if fnc[0] > frontier[selected_frontier][0] {
                        frontier_neighbours.push(Direction::Right);
                    } else {
                        frontier_neighbours.push(Direction::Left);
                    }
                }
                //frontier_neighbours.push(fnc);
            }
        }
        // Select a random one
        let selected_maze = rand::thread_rng().gen_range(0..frontier_neighbours.len());
        // Remove the wall between them

        maze.push(frontier[selected_frontier].clone());

        m[get_wall_index(frontier[selected_frontier], frontier_neighbours.get(selected_maze).unwrap(), w, h) as usize] = 0;

        frontier.remove(selected_frontier);
    */
    // Drawing 
    /*
    
    let wall = ["  ", "██", "░░", "▓▓"];

    for i in 0..h+1 {
        let mut upwalls = String::new();
        let mut leftwalls = String::new();
        for j in 0..w+1 {
            if j != w {
                let upwall = m[get_wall_index([j, i], &Direction::Up, w, h) as usize];
                upwalls += wall[1];
                upwalls += wall[upwall as usize];
            }
            if i < h {
                let leftwall = m[get_wall_index([j, i], &Direction::Left, w, h) as usize];
                leftwalls += wall[leftwall as usize];
                if j < w {
                    if maze.contains(&[j, i]) {
                        leftwalls += wall[0];
                    } else if frontier.contains(&[j, i]) {
                        leftwalls += wall[3];
                    } else {
                        leftwalls += wall[2];
                    }
                }
            }
            //print!(upwall)
        }
        //leftwalls += wall[m[get_wall_index(w-1, i, Direction::Right, w, h)] as usize];
        upwalls += wall[1];

        
        println!("{upwalls}");
        if i < h {
            println!("{leftwalls}");
        }
        //print!("{}", wall[upwall as usize])
    }*/
    //std::thread::sleep(time::Duration::from_millis(50));

    

    /*println!("
    ██████████████████████
            ██  ██  ██  ██
    ██████  ██  ██  ██  ██
    ██  ██              ██
    ██  ██████  ██████████
    ██                  ██
    ██████  ██████  ██  ██
    ██      ██  ██  ██  ██
    ██  ██████  ██  ██████
    ██      ██            
    ██████████████████████
            ");*/
