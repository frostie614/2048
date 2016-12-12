
mod  my2048;

fn main(){

	println!("This Hello World!");
	let mut board : [[u32;4];4]  = [[0;4];4];
	
	my2048::print_grid(&board);
	my2048::spawn(&mut board);
	my2048::print_grid(&board);
	
	println!("Starting game loop");
	
	'gameloop: loop{
			println!("loop");
			my2048::spawn(&mut board);
			if my2048::check_game_over(&board) {
				break 'gameloop;
			}

			my2048::print_grid(&board);

			
			let copy = board.clone();
			
			while board == copy {
				let input: my2048::Dir = my2048::get_user_input();
				match input {
						 my2048::Dir::Quit => {
							println!("You quit.");
							break 'gameloop;
						}
						_ => { my2048::do_move(&input, &mut board) }
				}
			}




	}
}
