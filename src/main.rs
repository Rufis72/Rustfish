use std::io::empty;

fn main() {
    let pos = Position::new();
    let fen = pos.to_fen();
    println!("{fen}")
}

enum Piece {
    DarkPawn,
    DarkRook,
    DarkKnight,
    DarkBishop,
    DarkQueen,
    DarkKing,
    LightPawn,
    LightRook,
    LightKnight,
    LightBishop,
    LightQueen,
    LightKing,
}

struct Position {
    // dark pieces
    dark_pawns: [[bool; 8]; 8],
    dark_rook: [[bool; 8]; 8],
    dark_knight: [[bool; 8]; 8],
    dark_bishop: [[bool; 8]; 8],
    dark_queen: [[bool; 8]; 8],
    // not sure if it's optimal to have a whole bitmap for the dark king
    // but optimization later, get it working first
    // TODO make this (possibly) more efficient
    dark_king: [[bool; 8]; 8],

    // light pieces
    light_pawns: [[bool; 8]; 8],
    light_rook: [[bool; 8]; 8],
    light_knight: [[bool; 8]; 8],
    light_bishop: [[bool; 8]; 8],
    light_queen: [[bool; 8]; 8],
    light_king: [[bool; 8]; 8],

    // castling info
    can_dark_castle_king_side: bool,
    can_dark_castle_queen_side: bool,
    can_light_castle_king_side: bool,
    can_light_castle_queen_side: bool,

    // other misc variables
    turn_count: u8,
}

impl Position {
    /// Initialises a Position struct with the starting position for chess
    fn new() -> Self {
        Position {
            dark_pawns: [
                [false, false, false, false, false, false, true, false]; 8
            ],
            dark_rook: [
                [false, false, false, false, false, false, false, true],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false, false, false, false, false, false, false, true],
            ], 
            dark_knight: [
                [false; 8],
                [false, false, false, false, false, false, false, true],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false, false, false, false, false, false, false, true],
                [false; 8]
            ], 
            dark_bishop: [
                [false; 8],
                [false; 8],
                [false, false, false, false, false, false, false, true],
                [false; 8],
                [false; 8],
                [false, false, false, false, false, false, false, true],
                [false; 8],
                [false; 8]
            ], 
            dark_queen: [
                [false; 8],
                [false; 8],
                [false; 8],
                [false, false, false, false, false, false, false, true],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8]
            ], 
            dark_king: [
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false, false, false, false, false, false, false, true],
                [false; 8],
                [false; 8],
                [false; 8]
            ], 
            light_pawns: [
                [false, true, false, false, false, false, false, false]; 8
            ], 
            light_rook: [
                [true, false, false, false, false, false, false, false],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [true, false, false, false, false, false, false, false],
            ], 
            light_knight: [
                [false; 8],
                [true, false, false, false, false, false, false, false],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [true, false, false, false, false, false, false, false],
                [false; 8]
            ], 
            light_bishop: [
                [false; 8],
                [false; 8],
                [true, false, false, false, false, false, false, false],
                [false; 8],
                [false; 8],
                [true, false, false, false, false, false, false, false],
                [false; 8],
                [false; 8]
            ], 
            light_queen: [
                [false; 8],
                [false; 8],
                [false; 8],
                [true, false, false, false, false, false, false, false],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8]
            ], 
            light_king: [
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [true, false, false, false, false, false, false, false],
                [false; 8],
                [false; 8],
                [false; 8]
            ], 
            can_dark_castle_king_side: true, 
            can_dark_castle_queen_side: true, 
            can_light_castle_king_side: true, 
            can_light_castle_queen_side: true, 
            turn_count: 0 }
    }

    /// Returns the piece at a square
    /// 
    /// This is done in a slightly optimized way where it checks the pieces that are most common first
    /// So pawns, then rooks, bishops, knights, kings, queens
    fn get_piece_at_square(&self, x: usize, y: usize) -> Option<Piece> {
        if self.dark_pawns[x][y] {
            Some(Piece::DarkPawn)
        } else if self.light_pawns[x][y] {
            Some(Piece::LightPawn)
        } else if self.dark_rook[x][y] {
            Some(Piece::DarkRook)
        } else if self.light_rook[x][y] {
            Some(Piece::LightRook)
        } else if self.dark_bishop[x][y] {
            Some(Piece::DarkBishop)
        } else if self.light_bishop[x][y] {
            Some(Piece::LightBishop)
        } else if self.dark_knight[x][y] {
            Some(Piece::DarkKnight)
        } else if self.light_knight[x][y] {
            Some(Piece::LightKnight)
        } else if self.dark_king[x][y] {
            Some(Piece::DarkKing)
        } else if self.light_king[x][y] {
            Some(Piece::LightKing)
        } else if self.dark_queen[x][y] {
            Some(Piece::DarkQueen)
        } else if self.light_queen[x][y] {
            Some(Piece::LightQueen)
        } else {
            None
        }
    }

    /// Turns the current position into a FEN (Forsyth–Edwards Notation)
    /// 
    /// This is not optimized (yet), so it's not meant to be used continuously
    fn to_fen(&self) -> String {
        // what we do is go through every row, then every piece on that row
        // if that square is empty, we add one to the count of empty squares
        // otherwise, we add the count of empty squares first (if it's not 0), then the letter for the piece
        // then at the end of the row, we add the count if it's not 0
        let mut fen: String = String::new();
        for y in 0..8 {
            let mut empty_square_count: usize = 0;
            for x in 0..8 {
                match self.get_piece_at_square(x, y) {
                    None => {
                        empty_square_count += 1;
                    }
                    Some(piece) => {
                        // if there's a gap from the last piece, adding a number to represent the gap
                        if empty_square_count != 0 {
                            fen += &empty_square_count.to_string();
                        }

                        // resetting the empty square count
                        empty_square_count = 0;

                        // adding the letter for the piece
                        fen += match piece {
                            Piece::DarkBishop => "b",
                            Piece::DarkKing => "k",
                            Piece::DarkKnight => "n",
                            Piece::DarkPawn => "p",
                            Piece::DarkQueen => "q",
                            Piece::DarkRook => "r",
                            Piece::LightBishop => "B",
                            Piece::LightKing => "K",
                            Piece::LightKnight => "N",
                            Piece::LightPawn => "P",
                            Piece::LightQueen => "Q",
                            Piece::LightRook => "R"
                        }
                    }
                }
            };
            
            // if there's still an empty count not yet added, we add it
            if empty_square_count != 0 {
                fen += &empty_square_count.to_string()
            }

            // adding a / to the end if it's not the last time
            if y != 7 {
                fen += "/";
            }
        }

        // returning the finished fen
        fen.to_string()

    }
}
