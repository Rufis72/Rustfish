fn main() {
    println!("Hello, world!");
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
    fn new() -> Self {
        Position {
            dark_pawns: [
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [true; 8],
                [false; 8]
            ],
            dark_rook: [
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [true, false, false, false, false, false, false, true]
            ], 
            dark_knight: [
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false, true, false, false, false, false, true, false]
            ], 
            dark_bishop: [
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false, false, true, false, false, true, false, false]
            ], 
            dark_queen: [
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false, false, false, true, false, false, false, false]
            ], 
            dark_king: [
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false, false, false, false, true, false, false, false]
            ], 
            light_pawns: [
                [false; 8],
                [true; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8]
            ], 
            light_rook: [
                [true, false, false, false, false, false, false, true],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8]
            ], 
            light_knight: [
                [false, true, false, false, false, false, true, false],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8]
            ], 
            light_bishop: [
                [false, false, true, false, false, true, false, false],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8]
            ], 
            light_queen: [
                [false, false, false, true, false, false, false, false],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8]
            ], 
            light_king: [
                [false, false, false, false, true, false, false, false],
                [false; 8],
                [false; 8],
                [false; 8],
                [false; 8],
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
}
