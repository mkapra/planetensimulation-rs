use planetensimulation::Board;
use std::io::Write;

const ITERATIONS: u32 = 200;
const MATLAB_FILENAME: &str = "simulation.m";

fn main() {
    env_logger::builder()
        .format(|buf, record| writeln!(buf, "{}: {}", record.level(), record.args()))
        .init();

    let mut board = Board::new(200, 100, 40, 40);
    board.generate_random_animals();

    let mut history_fishes = Vec::with_capacity(1500);
    let mut history_sharks = Vec::with_capacity(1500);

    let mut i = 0;
    while i < ITERATIONS {
        let (fishes, sharks) = board.count_animals();
        history_fishes.push(fishes);
        history_sharks.push(sharks);

        if let Err(_) = board.step() {
            break;
        }

        i += 1;
    }

    let fishes_string = history_fishes
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    let sharks_string = history_sharks
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    // Write matlab code to file
    let mut file = std::fs::File::create(MATLAB_FILENAME).unwrap();
    writeln!(file, "y_fishes = [{}];", fishes_string).unwrap();
    writeln!(file, "y_sharks = [{}];", sharks_string).unwrap();
    writeln!(file, "figure").unwrap();
    writeln!(file, "hold on").unwrap();
    writeln!(file, "plot(1:{}, y_fishes, 'r')", ITERATIONS).unwrap();
    writeln!(file, "plot(1:{}, y_sharks, 'b')", ITERATIONS).unwrap();
    writeln!(file, "hold off").unwrap();
    writeln!(file, "legend('Fishes', 'Sharks')").unwrap();

    // Close file
    file.flush().unwrap();

    // Print file path
    println!("{MATLAB_FILENAME} written to current directory");
}
