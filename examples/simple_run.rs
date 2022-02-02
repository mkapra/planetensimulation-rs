use planetensimulation::Board;

fn main() {
    let mut board = Board::new(10, 5, 5, 5);
    board.generate_random_animals();

    loop {
        if let Err(_) = board.step() {
            break;
        }
        println!("{}", board);
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
