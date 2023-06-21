use clearscreen;
use ttschnz::algorythms::game_of_life::{board::Board, patterns::Patterns};

fn main() {
    let mut board = Board::new((0, 0), (64, 64));
    board.fill();
    board.create_life(Patterns::GPT);

    while board.has_live() {
        let output = board.fmt();
        let _ = clearscreen::clear();
        println!("{}", output);
        std::thread::sleep(std::time::Duration::from_millis(100));
        if !board.tick() {
            break;
        }
    }
}
