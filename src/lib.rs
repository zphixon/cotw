#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Square {
    Empty,
    Beige(Piece),
    Black(Piece),
}

impl Square {
    pub fn is_players(&self, player: Player) -> bool {
        match player {
            Player::Beige => matches!(self, Square::Beige(_)),
            Player::Black => matches!(self, Square::Black(_)),
        }
    }

    pub fn player(&self) -> Option<Player> {
        match self {
            Square::Beige(_) => Some(Player::Beige),
            Square::Black(_) => Some(Player::Black),
            _ => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, Square::Empty)
    }

    pub fn is_cylinder(&self) -> bool {
        match *self {
            Square::Beige(piece) | Square::Black(piece) => piece == Piece::Cylinder,
            _ => false,
        }
    }

    pub fn is_messenger(&self) -> bool {
        self.is_unstunned_messenger() || self.is_stunned_messenger()
    }

    pub fn is_unstunned_messenger(&self) -> bool {
        match *self {
            Square::Beige(piece) | Square::Black(piece) => piece == Piece::Messenger,
            _ => false,
        }
    }

    pub fn is_stunned_messenger(&self) -> bool {
        match *self {
            Square::Beige(piece) | Square::Black(piece) => piece == Piece::StunnedMessenger,
            _ => false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Piece {
    Cylinder,
    Messenger,
    StunnedMessenger,
}

#[derive(Clone, Copy)]
pub struct Board {
    board: [[Square; 7]; 7],
}

#[rustfmt::skip]
pub const STARTING_BOARD: Board = {
    use Piece::*;
    use Square::*;

    Board {
        board: [
            [Empty, Empty, Empty,            Beige(Cylinder),  Empty,            Empty, Empty],
            [Empty, Empty, Beige(Messenger), Beige(Messenger), Beige(Messenger), Empty, Empty],
            [Empty, Empty, Empty,            Beige(Messenger), Empty,            Empty, Empty],
            [Empty, Empty, Empty,            Empty,            Empty,            Empty, Empty],
            [Empty, Empty, Empty,            Black(Messenger), Empty,            Empty, Empty],
            [Empty, Empty, Black(Messenger), Black(Messenger), Black(Messenger), Empty, Empty],
            [Empty, Empty, Empty,            Black(Cylinder),  Empty,            Empty, Empty],
        ],
    }
};

#[rustfmt::skip]
pub const BAD_THROW_2: Board = {
    use Piece::*;
    use Square::*;

    Board {
        board: [
            [Empty, Empty,           Empty,            Empty,            Empty,            Empty, Empty],
            [Empty, Black(Cylinder), Black(Messenger), Empty,            Empty,            Empty, Empty],
            [Empty, Empty,           Empty,            Black(Messenger), Beige(Messenger), Empty, Empty],
            [Empty, Empty,           Beige(Messenger), Empty,            Beige(Messenger), Empty, Empty],
            [Empty, Empty,           Beige(Messenger), Beige(Messenger), Beige(Messenger), Empty, Empty],
            [Empty, Empty,           Empty,            Empty,            Empty,            Empty, Empty],
            [Empty, Empty,           Empty,            Empty,            Empty,            Empty, Empty],
        ],
    }
};

#[rustfmt::skip]
pub const BAD_THROW_3: Board = {
    use Piece::*;
    use Square::*;

    Board {
        board: [
            [Empty, Empty,           Beige(Messenger), Beige(Messenger), Beige(Messenger), Empty, Empty],
            [Empty, Black(Cylinder), Black(Messenger), Empty,            Black(Messenger), Empty, Empty],
            [Empty, Empty,           Empty,            Black(Messenger), Beige(Messenger), Empty, Empty],
            [Empty, Empty,           Empty,            Empty,            Empty,            Empty, Empty],
            [Empty, Empty,           Empty,            Empty,            Empty,            Empty, Empty],
            [Empty, Empty,           Empty,            Empty,            Empty,            Empty, Empty],
            [Empty, Empty,           Empty,            Empty,            Empty,            Empty, Empty],
        ],
    }
};

impl std::ops::Index<BoardCoordinate> for Board {
    type Output = Square;
    fn index(&self, index: BoardCoordinate) -> &Self::Output {
        &self.board[(7 - index.y) as usize][(index.x - 1) as usize]
    }
}

impl std::ops::IndexMut<BoardCoordinate> for Board {
    fn index_mut(&mut self, index: BoardCoordinate) -> &mut Self::Output {
        &mut self.board[(7 - index.y) as usize][(index.x - 1) as usize]
    }
}

impl Board {
    fn move_piece(&self, from: BoardCoordinate, to: BoardCoordinate) -> Board {
        let mut new_board = *self;
        let piece = new_board[from];
        new_board[from] = Square::Empty;
        new_board[to] = piece;
        new_board
    }

    fn stun_if_opponents(&self, at: BoardCoordinate, player: Player) -> Board {
        match self[at] {
            Square::Beige(Piece::Messenger) if player.opponent() == Player::Beige => {
                let mut new_board = *self;
                new_board[at] = Square::Beige(Piece::StunnedMessenger);
                new_board
            }

            Square::Black(Piece::Messenger) if player.opponent() == Player::Black => {
                let mut new_board = *self;
                new_board[at] = Square::Black(Piece::StunnedMessenger);
                new_board
            }

            _ => *self,
        }
    }

    fn un_stun(&self, player: Player) -> Board {
        let mut new_board = *self;
        for row in new_board.board.iter_mut() {
            for square in row.iter_mut() {
                *square = match square {
                    Square::Beige(Piece::StunnedMessenger) if player == Player::Beige => {
                        Square::Beige(Piece::Messenger)
                    }
                    Square::Black(Piece::StunnedMessenger) if player == Player::Black => {
                        Square::Black(Piece::Messenger)
                    }
                    _ => *square,
                };
            }
        }
        new_board
    }
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            writeln!(f, "\n 1234567")?;
            let mut y = 7;
            for row in self.board {
                write!(f, "{}", y)?;
                for square in row {
                    match square {
                        Square::Empty => write!(f, " ")?,
                        Square::Beige(p) => match p {
                            Piece::Cylinder => write!(f, "C")?,
                            Piece::Messenger => write!(f, "M")?,
                            Piece::StunnedMessenger => write!(f, "S")?,
                        },
                        Square::Black(p) => match p {
                            Piece::Cylinder => write!(f, "c")?,
                            Piece::Messenger => write!(f, "m")?,
                            Piece::StunnedMessenger => write!(f, "s")?,
                        },
                    }
                }
                writeln!(f, "")?;
                y -= 1;
            }
            Ok(())
        } else {
            f.debug_list().entries(self.board.iter()).finish()
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoardCoordinate {
    x: i8,
    y: i8,
}

impl BoardCoordinate {
    pub fn new(x: i8, y: i8) -> Option<BoardCoordinate> {
        match (x, y) {
            (1..=7, 1..=7) => Some(BoardCoordinate { x, y }),
            _ => None,
        }
    }

    pub fn one_away(&self, other: BoardCoordinate) -> bool {
        ONE_SQUARE.iter().any(|dir| *self + *dir == Some(other))
    }
}

impl std::fmt::Debug for BoardCoordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({},{})", self, self.x + 1, self.y + 1 - 7)
    }
}

impl std::fmt::Display for BoardCoordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl std::ops::Add<Direction> for BoardCoordinate {
    type Output = Option<BoardCoordinate>;
    fn add(self, rhs: Direction) -> Self::Output {
        use Direction::*;
        match rhs {
            NW2 => BoardCoordinate::new(self.x - 2, self.y + 2),
            N2 => BoardCoordinate::new(self.x, self.y + 2),
            NE2 => BoardCoordinate::new(self.x + 2, self.y + 2),
            NW => BoardCoordinate::new(self.x - 1, self.y + 1),
            N => BoardCoordinate::new(self.x, self.y + 1),
            NE => BoardCoordinate::new(self.x + 1, self.y + 1),
            W2 => BoardCoordinate::new(self.x - 2, self.y),
            W => BoardCoordinate::new(self.x - 1, self.y),
            E => BoardCoordinate::new(self.x + 1, self.y),
            E2 => BoardCoordinate::new(self.x + 2, self.y),
            SW => BoardCoordinate::new(self.x - 1, self.y - 1),
            S => BoardCoordinate::new(self.x, self.y - 1),
            SE => BoardCoordinate::new(self.x + 1, self.y - 1),
            SW2 => BoardCoordinate::new(self.x - 2, self.y - 2),
            S2 => BoardCoordinate::new(self.x, self.y - 2),
            SE2 => BoardCoordinate::new(self.x + 2, self.y - 2),
        }
    }
}

impl std::ops::Neg for Direction {
    type Output = Self;
    fn neg(self) -> Self::Output {
        use Direction::*;
        match self {
            NW2 => SE2,
            N2 => S2,
            NE2 => SW2,
            NW => SE,
            N => S,
            NE => SW,
            W2 => E2,
            W => E,
            E => W,
            E2 => W2,
            SW => NE,
            S => N,
            SE => NW,
            SW2 => NE2,
            S2 => N2,
            SE2 => NW2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[rustfmt::skip]
pub enum Direction {
    NW2,     N2,     NE2,
         NW, N,  NE,
    W2,  W,      E,  E2,
         SW, S,  SE,
    SW2,     S2,     SE2,
}

impl Direction {
    pub fn unit(&self) -> Direction {
        use Direction::*;
        match self {
            NW2 => NW,
            N2 => N,
            NE2 => NE,
            W2 => W,
            E2 => E,
            SW2 => SW,
            S2 => S,
            SE2 => SE,
            _ => *self,
        }
    }
}

pub const ONE_SQUARE: [Direction; 8] = {
    use Direction::*;
    [NW, N, NE, W, E, SW, S, SE]
};

pub const TWO_SQUARES: [Direction; 8] = {
    use Direction::*;
    [NW2, N2, NE2, W2, E2, SW2, S2, SE2]
};

pub type ExtraThrows = Option<(Direction, Option<(Direction, Option<Direction>)>)>;

#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub player: Player,
    pub messenger: BoardCoordinate,
    pub direction: Direction,
    pub first_throw: Direction,
    pub extra_throws: ExtraThrows,
}

#[derive(Debug, Clone, Copy)]
pub struct LegalMove(Move);

impl LegalMove {
    pub fn from_move(game: &Game, move_: Move) -> Option<LegalMove> {
        let _new_board = game.attempt_move(&move_)?;
        Some(LegalMove(move_))
    }

    pub fn to_move(self) -> Move {
        self.0
    }
}

impl std::ops::Deref for LegalMove {
    type Target = Move;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Beige,
    Black,
}

impl Player {
    pub fn opponent(&self) -> Self {
        match self {
            Player::Beige => Player::Black,
            Player::Black => Player::Beige,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Game {
    to_move: Player,
    board: Board,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            to_move: Player::Beige,
            board: STARTING_BOARD,
        }
    }
}

impl Game {
    pub fn from_position(position: Board, to_move: Player) -> Game {
        Game {
            to_move,
            board: position,
        }
    }

    pub fn legal_moves(&self) -> Vec<LegalMove> {
        (1..=7)
            .flat_map(|y| (1..=7).map(move |x| (x, y)))
            .flat_map(|(x, y)| BoardCoordinate::new(x, y))
            .filter(|&coord| {
                self.board[coord].is_players(self.to_move)
                    && self.board[coord].is_unstunned_messenger()
            })
            .flat_map(|messenger| self.legal_moves_for(messenger))
            .collect()
    }

    pub fn legal_moves_for(&self, messenger: BoardCoordinate) -> Vec<LegalMove> {
        if !(self.board[messenger].is_unstunned_messenger()
            && self.board[messenger].is_players(self.to_move))
        {
            return vec![];
        }

        #[derive(PartialEq, Eq, Hash)]
        struct SeenThrow {
            messenger: BoardCoordinate,
            direction: Direction,
            throw_from: BoardCoordinate,
            throw_to: BoardCoordinate,
        }

        impl SeenThrow {
            fn from_legal_move(legal_move: &LegalMove) -> Self {
                let move_to = (legal_move.messenger + legal_move.direction).unwrap();
                let mut throw_to = (move_to + legal_move.first_throw).unwrap();
                if let Some((second_throw, extra_throws)) = legal_move.extra_throws {
                    throw_to = ((throw_to + second_throw).unwrap() + second_throw).unwrap();
                    if let Some((third_throw, extra_throws)) = extra_throws {
                        throw_to = ((throw_to + third_throw).unwrap() + third_throw).unwrap();
                        if let Some(fourth_throw) = extra_throws {
                            throw_to = ((throw_to + fourth_throw).unwrap() + fourth_throw).unwrap();
                        }
                    }
                }

                SeenThrow {
                    messenger: legal_move.messenger,
                    direction: legal_move.direction,
                    throw_from: (move_to + -legal_move.first_throw).unwrap(),
                    throw_to,
                }
            }
        }

        let mut seen_throws = std::collections::HashSet::new();

        let one_throw = ONE_SQUARE
            .iter()
            .chain(TWO_SQUARES.iter())
            .flat_map(|&direction| {
                ONE_SQUARE
                    .iter()
                    .map(move |&first_throw| (direction, messenger, first_throw))
            })
            .flat_map(|(direction, messenger, first_throw)| {
                LegalMove::from_move(
                    self,
                    Move {
                        direction,
                        messenger,
                        first_throw,
                        player: self.to_move,
                        extra_throws: None,
                    },
                )
            })
            .collect::<Vec<LegalMove>>();
        for legal_move in one_throw.iter() {
            seen_throws.insert(SeenThrow::from_legal_move(legal_move));
        }

        let two_throws = one_throw
            .iter()
            .flat_map(|&legal_move| {
                ONE_SQUARE
                    .iter()
                    .map(move |&second_throw| (legal_move, second_throw))
            })
            .flat_map(|(legal_move, second_throw)| {
                LegalMove::from_move(
                    self,
                    Move {
                        extra_throws: Some((second_throw, None)),
                        ..legal_move.to_move()
                    },
                )
            })
            .filter(|legal_move| !seen_throws.contains(&SeenThrow::from_legal_move(legal_move)))
            .collect::<Vec<LegalMove>>();
        for legal_move in one_throw.iter() {
            seen_throws.insert(SeenThrow::from_legal_move(legal_move));
        }

        let three_throws = two_throws
            .iter()
            .flat_map(|&legal_move| {
                ONE_SQUARE
                    .iter()
                    .map(move |&third_throw| (legal_move, third_throw))
            })
            .flat_map(|(legal_move, third_throw)| {
                let move_ = legal_move.to_move();
                let Some((second_throw, None)) = move_.extra_throws else {
                    // unreachable?
                    return None;
                };
                LegalMove::from_move(
                    self,
                    Move {
                        extra_throws: Some((second_throw, Some((third_throw, None)))),
                        ..move_
                    },
                )
            })
            .filter(|legal_move| !seen_throws.contains(&SeenThrow::from_legal_move(legal_move)))
            .collect::<Vec<LegalMove>>();
        for legal_move in one_throw.iter() {
            seen_throws.insert(SeenThrow::from_legal_move(legal_move));
        }

        let four_throws = three_throws
            .iter()
            .flat_map(|&legal_move| {
                ONE_SQUARE
                    .iter()
                    .map(move |&fourth_throw| (legal_move, fourth_throw))
            })
            .flat_map(|(legal_move, fourth_throw)| {
                let move_ = legal_move.to_move();
                let Some((second_throw, Some((third_throw, None)))) = move_.extra_throws else {
                    // unreachable?
                    return None;
                };
                LegalMove::from_move(
                    self,
                    Move {
                        extra_throws: Some((second_throw, Some((third_throw, Some(fourth_throw))))),
                        ..move_
                    },
                )
            })
            .filter(|legal_move| !seen_throws.contains(&SeenThrow::from_legal_move(legal_move)))
            .collect::<Vec<LegalMove>>();
        for legal_move in one_throw.iter() {
            seen_throws.insert(SeenThrow::from_legal_move(legal_move));
        }

        let mut moves = one_throw;
        moves.extend(two_throws);
        moves.extend(three_throws);
        moves.extend(four_throws);
        moves
    }

    pub fn make_move(&mut self, move_: &LegalMove) {
        self.board = self.attempt_move(&move_.0).unwrap().un_stun(self.to_move);
        self.to_move = self.to_move.opponent();
    }

    pub fn attempt_move(&self, move_: &Move) -> Option<Board> {
        macro_rules! my_debug {
            ($s:expr $(, $fmt:expr)*) => {
                #[cfg(feature = "debug")] {
                    tracing::debug!($s $(, $fmt)?);
                }
            }
        }

        macro_rules! my_error {
            ($s:expr $(, $fmt:expr)*) => {
                #[cfg(feature = "debug")] {
                    tracing::error!($s $(, $fmt)?);
                }
            }
        }

        macro_rules! rule {
            ($name:expr, $cond:expr $(, $dbg:expr $(, $($fmt:expr),*)?)?) => {
                $(my_debug!("{}: {}", $name, format!($dbg, $($($fmt),*)?));)?
                if !$cond {
                    my_error!("rejected move on {:?}: {:#?}\n{:#?}", $name, move_, self.board);
                    return None;
                }
            };

            ($name:expr => $some:expr $(, $dbg:expr $(, $($fmt:expr),*)?)?) => {
                {
                    $(my_debug!("{}: {}", $name, format!($dbg, $($($fmt),*)?));)?
                    match $some {
                        Some(some) => some,
                        _ => {
                            my_error!("rejected move on {:?}: ({:#?})\n{:#?}", $name, move_, self.board);
                            return None;
                        }
                    }
                }
            };
        }

        rule!(
            "moving player is current player",
            { self.to_move == move_.player },
            "to_move={:?} player={:?}",
            self.to_move,
            move_.player
        );

        rule!(
            "messenger to be moved is current player's",
            { self.board[move_.messenger].is_players(self.to_move) },
            "messenger={} to_move={:?} player={:?}",
            move_.messenger,
            self.to_move,
            self.board[move_.messenger].player()
        );

        rule!(
            "messenger to be moved is not stunned",
            { self.board[move_.messenger].is_unstunned_messenger() },
            "messenger={}",
            move_.messenger
        );

        let move_to = rule!(
            "messenger is not moving off the board"
            => { move_.messenger + move_.direction },
            "direction={:?}",
            move_.direction
        );

        rule!(
            "messenger is moving to an unoccupied square",
            { self.board[move_to].is_empty() },
            "move_to={}",
            move_to
        );

        rule!(
            "messenger does not hop over occupied squares",
            { self.board[(move_.messenger + move_.direction.unit()).unwrap()].is_empty() },
            "unit={}",
            (move_.messenger + move_.direction.unit()).unwrap()
        );

        let after_move = self.board.move_piece(move_.messenger, move_to);

        let check_throw = |n: usize, board: Board, messenger: BoardCoordinate, throw: Direction| {
            let _ = n;
            rule!(
                "player's messenger is throwing",
                { board[messenger].is_messenger() && board[messenger].is_players(move_.player) },
                "messenger={} board[messenger]={:?}",
                messenger,
                board[messenger]
            );
            rule!(
                "throw is unit length",
                { throw.unit() == throw },
                "n={} throw={:?} unit={:?}",
                n,
                throw,
                throw.unit()
            );
            let throw_from = rule!(
                "messenger is throwing something on the board"
                => { messenger + -throw },
                "n={} messenger={} throw={:?} -throw={:?}",
                n,
                messenger,
                throw,
                -throw
            );
            let throw_to = rule!(
                "messenger is throwing onto the board"
                => { messenger + throw },
                "n={} throw={:?}",
                n,
                throw
            );
            rule!(
                "messenger is throwing their own cylinder or a messenger",
                {
                    (board[throw_from].is_cylinder() && board[throw_from].is_players(move_.player))
                        || board[throw_from].is_messenger()
                },
                "n={} throw_from={} board[throw_from]={:?}",
                n,
                throw_from,
                board[throw_from]
            );
            rule!(
                "throw destination is unoccupied",
                { board[throw_to].is_empty() },
                "n={} throw_to={} board[throw_to]={:?}",
                n,
                throw_to,
                board[throw_to]
            );
            Some((
                board
                    .move_piece(throw_from, throw_to)
                    .stun_if_opponents(throw_to, move_.player),
                throw_to,
            ))
        };

        let (after_throw1, throw_to1) = check_throw(1, after_move, move_to, move_.first_throw)?;

        if let Some((throw2, throw34)) = move_.extra_throws {
            let num_surrounding = |board: Board, player: Player, coord: BoardCoordinate| {
                ONE_SQUARE
                    .iter()
                    .flat_map(|&dir| coord + dir)
                    .filter(|&coord| board[coord].is_players(player) && board[coord].is_messenger())
                    .count()
            };

            let my_surrounding = num_surrounding(after_throw1, move_.player, throw_to1);
            let their_surrounding =
                num_surrounding(after_throw1, move_.player.opponent(), throw_to1);
            rule!(
                "throw 2 requires a majority of the player's messengers surrounding the destination of throw 1",
                { my_surrounding > their_surrounding },
                "my_surrounding={} their_surrounding={}",
                my_surrounding,
                their_surrounding
            );

            let messenger2 = rule!(
                "messenger 2 is on the board"
                => { throw_to1 + throw2 },
                "throw_to1={} throw2={:?}",
                throw_to1,
                throw2
            );

            let (after_throw2, throw_to2) = check_throw(2, after_throw1, messenger2, throw2)?;

            match throw34 {
                None => Some(after_throw2),

                Some((throw3, None)) => {
                    let my_surrounding = num_surrounding(after_throw2, move_.player, throw_to2);
                    let their_surrounding =
                        num_surrounding(after_throw2, move_.player.opponent(), throw_to2);
                    rule!(
                        "throw 3 requires a majority of the player's messengers surrounding the destination of throw 2",
                        { my_surrounding > their_surrounding },
                        "my_surrounding={} their_surrounding={}",
                        my_surrounding,
                        their_surrounding
                    );

                    let messenger3 = rule!(
                        "messenger 3 is on the board"
                        => { throw_to2 + throw3 },
                        "throw_to2={} throw3={:?}",
                        throw_to2,
                        throw3
                    );

                    let (after_throw3, _) = check_throw(3, after_throw2, messenger3, throw3)?;

                    Some(after_throw3)
                }

                Some((throw3, Some(throw4))) => {
                    let my_surrounding = num_surrounding(after_throw2, move_.player, throw_to2);
                    let their_surrounding =
                        num_surrounding(after_throw2, move_.player.opponent(), throw_to2);
                    rule!(
                        "throw 3 requires a majority of the player's messengers surrounding the destination of throw 2",
                        { my_surrounding > their_surrounding },
                        "my_surrounding={} their_surrounding={}",
                        my_surrounding,
                        their_surrounding
                    );

                    let messenger3 = rule!(
                        "messenger 3 is on the board"
                        => { throw_to2 + throw3 },
                        "throw_to2={} throw3={:?}",
                        throw_to2,
                        throw3
                    );

                    let (after_throw3, throw_to3) =
                        check_throw(3, after_throw2, messenger3, throw3)?;

                    let my_surrounding = num_surrounding(after_throw3, move_.player, throw_to3);
                    let their_surrounding =
                        num_surrounding(after_throw3, move_.player.opponent(), throw_to3);
                    rule!(
                        "throw 4 requires a majority of the player's messengers surrounding the destination of throw 3",
                        { my_surrounding > their_surrounding },
                        "my_surrounding={} their_surrounding={}",
                        my_surrounding,
                        their_surrounding
                    );

                    let messenger4 = rule!(
                        "messenger 4 is on the board"
                        => { throw_to3 + throw3 },
                        "throw_to3={} throw3={:?}",
                        throw_to3,
                        throw3
                    );

                    let (after_throw4, _) = check_throw(4, after_throw3, messenger4, throw4)?;

                    Some(after_throw4)
                }
            }
        } else {
            Some(after_throw1)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn legal_moves() {
        let game = Game::default();
        for move_ in game.legal_moves() {
            game.attempt_move(&*move_).unwrap();
        }
    }
}
