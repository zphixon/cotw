export enum PieceType {
    Cylinder,
    Messenger,
    StunnedMessenger,
}

export enum Player {
    Beige,
    Black,
}

export function playerOpponent(player: Player): Player {
    switch (player) {
        case Player.Beige: return Player.Black;
        case Player.Black: return Player.Beige;
    }
}

export class Piece {
    constructor(
        public player: Player,
        public piece: PieceType,
    ) { }

    isMessenger(): boolean {
        return this.piece === PieceType.Messenger || this.piece === PieceType.StunnedMessenger;
    }

    asString(): string {
        let s;
        if (this.player === Player.Beige) {
            s = 'Beige';
        } else {
            s = 'Black';
        }
        if (this.piece === PieceType.Cylinder) {
            s += 'Cylinder';
        } else if (this.piece == PieceType.Messenger) {
            s += 'Messenger';
        } else {
            s += 'StunnedMessenger';
        }
        return s;
    }
}

export class Board {
    constructor(
        public board: Array<Array<Piece | undefined>>,
    ) { }

    static default(): Board {
        return new Board([
            [undefined, undefined, undefined, new Piece(Player.Beige, PieceType.Cylinder), undefined, undefined, undefined],
            [undefined, undefined, new Piece(Player.Beige, PieceType.Messenger), new Piece(Player.Beige, PieceType.Messenger), new Piece(Player.Beige, PieceType.Messenger), undefined, undefined],
            [undefined, undefined, undefined, new Piece(Player.Beige, PieceType.Messenger), undefined, undefined, undefined],
            [undefined, undefined, undefined, undefined, undefined, undefined, undefined],
            [undefined, undefined, undefined, new Piece(Player.Black, PieceType.Messenger), undefined, undefined, undefined],
            [undefined, undefined, new Piece(Player.Black, PieceType.Messenger), new Piece(Player.Black, PieceType.Messenger), new Piece(Player.Black, PieceType.Messenger), undefined, undefined],
            [undefined, undefined, undefined, new Piece(Player.Black, PieceType.Cylinder), undefined, undefined, undefined],
        ]);
    }

    get(coord: BoardCoordinate): Piece | undefined {
        return this.board[7 - coord.y][coord.x - 1];
    }

    private set(coord: BoardCoordinate, piece: Piece | undefined) {
        this.board[7 - coord.y][coord.x - 1] = piece;
    }

    movePiece(from: BoardCoordinate, to: BoardCoordinate): Board {
        let newBoard = new Board([...this.board.map((row) => [...row])]);
        let piece = newBoard.get(from);
        newBoard.set(from, undefined);
        newBoard.set(to, piece);
        return newBoard;
    }

    stunIfOpponents(at: BoardCoordinate, player: Player): Board {
        let newBoard = new Board([...this.board.map((row) => [...row])]);
        let pieceAt = newBoard.get(at);
        if (
            pieceAt !== undefined
            && pieceAt.piece === PieceType.Messenger
            && pieceAt.player === playerOpponent(player)
        ) {
            pieceAt.piece = PieceType.StunnedMessenger;
        }
        return newBoard;
    }
}

export class BoardCoordinate {
    x: number;
    y: number;

    constructor(x: number, y: number) {
        if (1 <= x && x <= 7 && 1 <= y && y <= 7) {
            this.x = x;
            this.y = y;
        } else {
            throw new Error(`${x},${y} is an invalid coordinate`);
        }
    }

    static create(x: number, y: number): BoardCoordinate | undefined {
        try {
            let coord = new BoardCoordinate(x, y);
            return coord;
        } catch (_) {
            return undefined;
        }
    }

    plus(dir: Direction): BoardCoordinate | undefined {
        switch (dir) {
            case Direction.NW2: return BoardCoordinate.create(this.x - 2, this.y + 2);
            case Direction.N2: return BoardCoordinate.create(this.x, this.y + 2);
            case Direction.NE2: return BoardCoordinate.create(this.x + 2, this.y + 2);
            case Direction.NW: return BoardCoordinate.create(this.x - 1, this.y + 1);
            case Direction.N: return BoardCoordinate.create(this.x, this.y + 1);
            case Direction.NE: return BoardCoordinate.create(this.x + 1, this.y + 1);
            case Direction.W2: return BoardCoordinate.create(this.x - 2, this.y);
            case Direction.W: return BoardCoordinate.create(this.x - 1, this.y);
            case Direction.E: return BoardCoordinate.create(this.x + 1, this.y);
            case Direction.E2: return BoardCoordinate.create(this.x + 2, this.y);
            case Direction.SW: return BoardCoordinate.create(this.x - 1, this.y - 1);
            case Direction.S: return BoardCoordinate.create(this.x, this.y - 1);
            case Direction.SE: return BoardCoordinate.create(this.x + 1, this.y - 1);
            case Direction.SW2: return BoardCoordinate.create(this.x - 2, this.y - 2);
            case Direction.S2: return BoardCoordinate.create(this.x, this.y - 2);
            case Direction.SE2: return BoardCoordinate.create(this.x + 2, this.y - 2);
        }
    }
}

export enum Direction {
    NW2, N2, NE2,
    NW, N, NE,
    W2, W, E, E2,
    SW, S, SE,
    SW2, S2, SE2
}

export const ONE_SQUARE = [
    Direction.NW,
    Direction.N,
    Direction.NE,
    Direction.W,
    Direction.E,
    Direction.SW,
    Direction.S,
    Direction.SE
];

export function unitDirection(dir: Direction): Direction {
    switch (dir) {
        case Direction.NW2: return Direction.NW;
        case Direction.N2: return Direction.N;
        case Direction.NE2: return Direction.NE;
        case Direction.W2: return Direction.W;
        case Direction.E2: return Direction.E;
        case Direction.SW2: return Direction.SW;
        case Direction.S2: return Direction.S;
        case Direction.SE2: return Direction.SE;
        default: return dir;
    }
}

export function negateDirection(dir: Direction): Direction {
    switch (dir) {
        case Direction.NW2: return Direction.SE2;
        case Direction.N2: return Direction.S2;
        case Direction.NE2: return Direction.SW2;
        case Direction.NW: return Direction.SE;
        case Direction.N: return Direction.S;
        case Direction.NE: return Direction.SW;
        case Direction.W2: return Direction.E2;
        case Direction.W: return Direction.E;
        case Direction.E: return Direction.W;
        case Direction.E2: return Direction.W2;
        case Direction.SW: return Direction.NE;
        case Direction.S: return Direction.N;
        case Direction.SE: return Direction.NW;
        case Direction.SW2: return Direction.NE2;
        case Direction.S2: return Direction.N2;
        case Direction.SE2: return Direction.NW2;
    }
}

export class Move {
    constructor(
        public player: Player,
        public messenger: BoardCoordinate,
        public direction: Direction,
        public firstThrow: Direction,
        public extraThrows: Array<Direction>,
    ) { }
}

export class LegalMove extends Move {
    static fromMove(game: Game, move: Move): LegalMove | undefined {
        if (game.attemptMove(move) !== undefined) {
            return move;
        }
        return undefined;
    }
}

export class Game {
    toMove: Player;
    board: Board;

    constructor() {
        this.toMove = Player.Beige;
        this.board = Board.default();
    }

    attemptMove(move: Move): Board | undefined {
        if (this.toMove !== move.player) {
            return undefined;
        }

        if (this.board.get(move.messenger)?.player !== move.player) {
            return undefined;
        }

        if (this.board.get(move.messenger)?.piece !== PieceType.Messenger) {
            return undefined;
        }

        const moveTo = move.messenger.plus(move.direction);
        const moveThrough = move.messenger.plus(unitDirection(move.direction));
        if (moveTo === undefined || moveThrough === undefined) {
            return undefined;
        }

        if (this.board.get(moveTo) !== undefined) {
            return undefined;
        }

        if (this.board.get(moveThrough) !== undefined) {
            return undefined;
        }

        const afterMove = this.board.movePiece(move.messenger, moveTo);

        const checkThrow = (
            board: Board,
            messenger: BoardCoordinate,
            throw_: Direction
        ): { afterThrow?: Board, throwTo?: BoardCoordinate } => {
            if (!board.get(messenger)?.isMessenger()) {
                return {};
            }
            if (board.get(messenger)?.player !== move.player) {
                return {};
            }
            if (unitDirection(throw_) !== throw_) {
                return {};
            }
            let throwFrom = messenger.plus(negateDirection(throw_));
            let throwTo = messenger.plus(throw_);
            if (!throwFrom || !throwTo) {
                return {};
            }
            if (!(
                (board.get(throwFrom)?.piece === PieceType.Cylinder
                    && board.get(throwFrom)?.player == move.player)
                || board.get(throwFrom)?.isMessenger()
            )) {
                return {};
            }
            if (board.get(throwTo) !== undefined) {
                return {};
            }
            return {
                afterThrow: board.movePiece(
                    throwFrom,
                    throwTo
                ).stunIfOpponents(
                    throwTo,
                    move.player
                ),
                throwTo
            };
        };

        let { afterThrow: afterThrow1, throwTo: throwTo1 }
            = checkThrow(afterMove, moveTo, move.firstThrow);
        if (afterThrow1 === undefined || throwTo1 === undefined) {
            return undefined;
        }

        if (move.extraThrows.length > 4) {
            return undefined;
        }
        if (move.extraThrows.length >= 1) {
            let [throw2, ...throw34] = move.extraThrows;
            let numSurrounding = (board: Board, player: Player, coord: BoardCoordinate) => {
                return ONE_SQUARE.map((dir) => {
                    return coord.plus(dir);
                }).filter((coord): coord is BoardCoordinate => {
                    return coord !== undefined;
                }).filter((coord) => {
                    return (board.get(coord)?.player === player
                        && board.get(coord)?.isMessenger());
                }).length;
            };

            let mySurrounding = numSurrounding(afterThrow1, move.player, throwTo1);
            let theirSurrounding = numSurrounding(afterThrow1, playerOpponent(move.player), throwTo1);
            if (mySurrounding <= theirSurrounding) {
                return undefined;
            }
            let messenger2 = throwTo1.plus(throw2);
            if (messenger2 === undefined) {
                return undefined;
            }

            let { afterThrow: afterThrow2, throwTo: throwTo2 }
                = checkThrow(afterThrow1, messenger2, throw2);
            if (afterThrow2 === undefined || throwTo2 === undefined) {
                return undefined;
            }

            let [throw3, throw4] = throw34;
            if (throw3 !== undefined) {
                let mySurrounding = numSurrounding(afterThrow2, move.player, throwTo2);
                let theirSurrounding = numSurrounding(afterThrow2, playerOpponent(move.player), throwTo2);
                if (mySurrounding <= theirSurrounding) {
                    return undefined;
                }
                let messenger3 = throwTo2.plus(throw3);
                if (messenger3 === undefined) {
                    return undefined;
                }

                let { afterThrow: afterThrow3, throwTo: throwTo3 }
                    = checkThrow(afterThrow2, messenger3, throw3);
                if (afterThrow3 === undefined || throwTo3 === undefined) {
                    return undefined;
                }

                if (throw4 !== undefined) {
                    let mySurrounding = numSurrounding(afterThrow3, move.player, throwTo3);
                    let theirSurrounding = numSurrounding(afterThrow3, playerOpponent(move.player), throwTo3);
                    if (mySurrounding <= theirSurrounding) {
                        return undefined;
                    }
                    let messenger4 = throwTo3.plus(throw4);
                    if (messenger4 === undefined) {
                        return undefined;
                    }

                    let { afterThrow: afterThrow4, throwTo: throwTo4 }
                        = checkThrow(afterThrow3, messenger4, throw4);
                    if (afterThrow4 === undefined || throwTo4 === undefined) {
                        return undefined;
                    }

                    return afterThrow4;
                }

                return afterThrow3;
            }

            return afterThrow2;
        }

        return afterThrow1;
    }
}
