use planetensimulation::Board;
use std::io::Write;

fn main() {
    env_logger::builder()
        .format(|buf, record| writeln!(buf, "{}: {}", record.level(), record.args()))
        .init();

    let mut board = Board::new(200, 100, 40, 40);
    board.generate_random_animals();

    loop {
        if let Err(_) = board.step() {
            break;
        }
        println!("{}", board);
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
