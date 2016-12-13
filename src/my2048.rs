

use std::io::{self};
extern crate rand;

pub enum Dir{
	Up,
	Down,
	Left,
	Right,
	Quit,
}

pub fn print_grid(grid : &[[u32; 4];4]){
	
	for i in  0..4{
		print!("|");
		for j in  0..4{
			print!("{} |",grid[i][j] );
		}
		
		println!("|");
	}

}

pub fn do_move(dir : &Dir, grid :&mut [[u32; 4];4] ){
	
	match *dir{
		Dir::Quit => {
			// does nothing
		},

		Dir::Up => {
			for cidx in 0..4 {
				for ridx in 0..4 {
					for tidx in (ridx+1)..4 {
						if grid[tidx][cidx] != 0 {
							if grid[ridx][cidx] == 0 {
								grid[ridx][cidx] += grid[tidx][cidx];
								grid[tidx][cidx] = 0;
							}
							else if grid[ridx][cidx] == grid[tidx][cidx] {
								grid[ridx][cidx] += grid[tidx][cidx];
								grid[tidx][cidx] = 0;
								break;
							}
							else {
								// do nothing
								break;
							}
						}
					}
				}
			}
			
		},

		Dir::Down =>{
			for cidx in 0..4 {
				for ridx in (0..4).rev() {
					for tidx in (0..ridx).rev() {
						if grid[tidx][cidx] != 0 {
							if grid[ridx][cidx] == 0 {
								grid[ridx][cidx] += grid[tidx][cidx];
								grid[tidx][cidx] = 0;
							}
							else if grid[ridx][cidx] == grid[tidx][cidx] {
								grid[ridx][cidx] += grid[tidx][cidx];
								grid[tidx][cidx] = 0;
								break;
							}
							else {
								// do nothing
								break;
							}
						}
					}
				}
			}
			

		},

		Dir::Left => {
			for row in grid {
				for cidx in 0..4 {
					for tidx in (cidx+1)..4 {
						if row[tidx] != 0 {
							if row[cidx] == 0 {
								row[cidx] += row[tidx];
								row[tidx] = 0;
							}
							else if row[cidx] == row[tidx] {
								row[cidx] += row[tidx];
								row[tidx] = 0;
								break;
							}
							else {
								// do nothing
								break;
							}
						}
					}
				}
			}
			

		},

		Dir::Right => {
			for row in grid {
				for cidx in (0..4).rev() {
					for tidx in (0..cidx).rev() {
						if row[tidx] != 0 {
							if row[cidx] == 0 {
								row[cidx] += row[tidx];
								row[tidx] = 0;
							}
							else if row[cidx] == row[tidx] {
								row[cidx] += row[tidx];
								row[tidx] = 0;
								break;
							}
							else {
								// do nothing
								break;
							}
						}
					}
				}
			}
			

		},
	}	
}

pub fn spawn(grid :&mut [[u32;4];4]){
	
	let mut row = 4;
	let mut col = 4;

	// adds some contrains to ensure
	while ( (row > 3) || (col > 3) ) || (grid[row][col] != 0) {
		
		println!("({},{})", row,col);
		
		
		row = rand::random::<usize>();
        	col = rand::random::<usize>();
        	row %= 4;
        	col %= 4;
	}
	
	if rand::random::<usize>() % 10 == 0 {
		grid[row][col] = 4;
	}
	else{
		grid[row][col] = 2;
	}
}



pub fn get_user_input() -> Dir {
	let dir : Dir;
    	loop{
        	let mut input = String::new();
        	io::stdin().read_line(&mut input).unwrap();
 
        	match input.chars().nth(0){
            		Some('a') =>{dir = Dir::Left ;break },
            		Some('w') =>{dir = Dir::Up   ;break },
            		Some('s') =>{dir = Dir::Down ;break },
            		Some('d') =>{dir = Dir::Right;break },
			Some('q') =>{dir = Dir::Quit; break },
            		_   => {println!("input was {}: invalid character should be a,s,w or d ",input.chars().nth(0).unwrap());} ,
        	}
    	}
    dir
}


pub fn check_game_over(grid :&[[u32;4];4])->bool{
	let mut game_over = false;	
	let mut left_clone : [[u32;4];4]  = grid.clone();
	let mut right_clone : [[u32;4];4]  = grid.clone();
	let mut up_clone : [[u32;4];4]  = grid.clone();
	let mut down_clone : [[u32;4];4]  = grid.clone();
	
	do_move(&Dir::Up, &mut up_clone);
	do_move(&Dir::Down, &mut down_clone);
	do_move(&Dir::Left, &mut left_clone);
	do_move(&Dir::Right, &mut right_clone);
	

	if left_clone != *grid || right_clone != *grid || up_clone != *grid || down_clone != *grid {
		println!("Make your next move!");
	}
	else{
		println!("Game over Unfortunatly!");
		game_over = true;
	}
	
	game_over
}
