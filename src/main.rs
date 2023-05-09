use cotw::{BoardCoordinate, Direction, Game, LegalMove, Move, Player};

fn main() {
    tracing_subscriber::fmt::init();

    let mut game = Game::default();
    dbg!(&game);

    let move_ = LegalMove::from_move(
        &game,
        Move {
            player: Player::Beige,
            messenger: BoardCoordinate::new(3, 6).unwrap(),
            direction: Direction::S,
            first_throw: Direction::SW,
            extra_throws: None,
        },
    )
    .unwrap();
    game.make_move(&move_);
    dbg!(&game);

    let move_ = LegalMove::from_move(
        &game,
        Move {
            player: Player::Black,
            messenger: BoardCoordinate::new(4, 3).unwrap(),
            direction: Direction::NW2,
            first_throw: Direction::N,
            extra_throws: None,
        },
    )
    .unwrap();
    game.make_move(&move_);
    dbg!(&game);

    let move_ = LegalMove::from_move(
        &game,
        Move {
            player: Player::Beige,
            messenger: BoardCoordinate::new(5, 6).unwrap(),
            direction: Direction::W2,
            first_throw: Direction::E,
            extra_throws: Some((Direction::SW, None)),
        },
    )
    .unwrap();
    game.make_move(&move_);
    dbg!(&game);

    let move_ = LegalMove::from_move(
        &game,
        Move {
            player: Player::Black,
            messenger: BoardCoordinate::new(3, 2).unwrap(),
            direction: Direction::N2,
            first_throw: Direction::SE,
            extra_throws: None,
        },
    )
    .unwrap();
    game.make_move(&move_);
    dbg!(&game);

    let mut game = Game::from_position(cotw::BAD_THROW_3, Player::Black);
    dbg!(game.legal_moves());

    let move_ = LegalMove::from_move(
        &game,
        Move {
            player: Player::Black,
            messenger: BoardCoordinate::new(3, 6).unwrap(),
            direction: Direction::S,
            first_throw: Direction::SE,
            extra_throws: Some((Direction::N, Some((Direction::E, None)))),
        },
    )
    .unwrap();
    game.make_move(&move_);
    dbg!(&game);
}
