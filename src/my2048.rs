

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
		Dir::Quit => {// does nothing
						},
		// code is worng on part of it, d/s sometimes dont work.. and when they do they go in wrong direction i think..
		Dir::Up => {
			// ignore row 0, can't do move anything
			for row in  1..4{
				for col in 0..4{
					if grid[row][col] != 0 {
						for space in 0..row{
							if grid[space][col] == 0 || grid[space][col] == grid[row][col]  {
								grid[space][col] += grid[row][col];
								grid[row][col] = 0;
							}
						}
					}
				
				}
			}
				
		},

		Dir::Down => {
			for row in  (0..3).rev() {
                                for col in 0..4{
                                        if grid[row][col] != 0 { 
                                                for space in (0..row).rev() {
                                                        if grid[space][col] == 0 || grid[space][col] == grid[row][col] {
                                                                grid[row][col] += grid[space][col];
                                                                grid[space][col] = 0;
                                                        }
                                                }
                                        }

                                }
                        }
		},

		Dir::Left => {
			for row in  0..4{
                                for col in 0..4{
                                        if grid[row][col] != 0 { 
                                                for space in 0..col{
                                                        if grid[row][space] == 0 || grid[row][space] == grid[row][col] {
                                                                grid[row][space] += grid[row][col];
                                                                grid[row][col] = 0;
                                                        }
                                                }
                                        }

                                }
                        }
		},

		Dir::Right => {
			for row in  0..4{
                                for col in (0..4).rev(){
                                        if grid[row][col] != 0 {
                                                for space in (0..col).rev(){
                                                        if grid[row][space] == 0  ||  grid[row][space] == grid[row][col] {
                                                                grid[row][col] += grid[row][space];
                                                                grid[row][space] = 0;
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
