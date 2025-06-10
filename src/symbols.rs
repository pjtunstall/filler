use crate::bimap::BiMap;

pub const P1_CHAR: char = '@'; // 42 School: 'O'.
pub const P2_CHAR: char = '$'; // 42 School: 'X'.
pub const P1_LATEST_CHAR: char = 'a'; // 42 School: 'o'.
pub const P2_LATEST_CHAR: char = 's'; // 42 School: 'x'.
pub const EMPTY_CHAR: char = '.'; // 42 School: '.'.
pub const NEW_PIECE_CHAR: char = 'O'; // 42 School: '*'.

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum CellRole {
    Empty,
    OwnSymbol,
    OpponentSymbol,
    OwnLatestMove,
    OpponentLatestMove,
}

pub struct Chars {
    pub own_char: char,
    pub own_latest_char: char,
    pub opponent_char: char,
    pub opponent_latest_char: char,
}

pub fn populate_char_to_role(char_to_role: &mut BiMap<char, CellRole>, own_id: u8) -> Chars {
    let [own_char, opponent_char] = assign_chars(own_id);
    let [own_latest_char, opponent_latest_char] = assign_latest_move_chars(own_id);

    char_to_role.insert(EMPTY_CHAR, CellRole::Empty);
    char_to_role.insert(own_char, CellRole::OwnSymbol);
    char_to_role.insert(opponent_char, CellRole::OpponentSymbol);
    char_to_role.insert(own_latest_char, CellRole::OwnLatestMove);
    char_to_role.insert(opponent_latest_char, CellRole::OpponentLatestMove);

    Chars {
        own_char,
        own_latest_char,
        opponent_char,
        opponent_latest_char,
    }
}

fn assign_chars(own_id: u8) -> [char; 2] {
    if own_id == 1 {
        [P1_CHAR, P2_CHAR]
    } else {
        [P2_CHAR, P1_CHAR]
    }
}

fn assign_latest_move_chars(own_id: u8) -> [char; 2] {
    if own_id == 1 {
        [P1_LATEST_CHAR, P2_LATEST_CHAR]
    } else {
        [P2_LATEST_CHAR, P1_LATEST_CHAR]
    }
}
