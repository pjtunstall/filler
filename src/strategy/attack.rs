use std::collections::VecDeque;

use crate::{
    anfield::Anfield,
    cell::Cell,
    piece::{Piece, PossiblePlacement},
    strategy::Strategy,
    symbols::CellRole,
};

pub struct Attack;

impl Strategy for Attack {
    fn choose_move(&self, anfield: &Anfield, piece: &Piece) -> [i32; 2] {
        place(anfield, piece)
    }
}

pub fn place(anfield: &Anfield, piece: &Piece) -> [i32; 2] {
    let possible_placements = get_possible_placements(anfield, &piece);
    let mut chosen_possible_placement = possible_placements[0];
    for possible_placement in possible_placements.iter().skip(1) {
        if possible_placement.weight > chosen_possible_placement.weight {
            chosen_possible_placement = *possible_placement;
        }
    }
    let x = chosen_possible_placement.x as i32;
    let y = chosen_possible_placement.y as i32;
    [x, y]
}

// The "casting gymnastics" here (and in `try_fit`) are to allow pieces to be placed in such a way that their top-left corner is outside of the Anfield, as long as their shape cells are inside it. This can prevent the bot from getting stuck in situations where that's is the only way to make a piece overlap its territory.
fn get_possible_placements(anfield: &Anfield, piece: &Piece) -> Vec<PossiblePlacement> {
    let mut possible_placements = Vec::new();

    let x_min = -(piece.width as isize);
    let x_max = anfield.width as isize;
    let y_min = -(piece.height as isize);
    let y_max = anfield.height as isize;

    for x in x_min..x_max {
        for y in y_min..y_max {
            if let Some(mut possible_placement) = try_fit(anfield, piece, x, y) {
                for cell in &piece.shape {
                    let s = x + cell.x as isize;
                    let t = y + cell.y as isize;
                    assert!(
                        s >= 0 && t >= 0,
                        "Negative cell placement coordinates: ({}, {})",
                        s,
                        t
                    );
                    let s = s as usize;
                    let t = t as usize;
                    let cell_distance = get_distance_to_opponent(anfield, s, t);
                    let cell_weight = usize::MAX - cell_distance;
                    possible_placement.weight += cell_weight;
                }
                possible_placements.push(possible_placement);
            }
        }
    }

    if possible_placements.is_empty() {
        possible_placements.push(PossiblePlacement::default());
    }

    possible_placements
}

fn try_fit(anfield: &Anfield, piece: &Piece, x: isize, y: isize) -> Option<PossiblePlacement> {
    let mut overlaps_with_own_territory = 0;
    for cell in &piece.shape {
        let s = x + cell.x as isize;
        let t = y + cell.y as isize;

        if s >= anfield.width as isize || t >= anfield.height as isize || s < 0 || t < 0 {
            return None;
        }

        let cell_role = anfield.get_cell_role(s as usize, t as usize);
        match cell_role {
            Some(CellRole::OpponentSymbol | CellRole::OpponentLatestMove) => return None,
            Some(CellRole::OwnSymbol | CellRole::OwnLatestMove) => overlaps_with_own_territory += 1,
            Some(_) => (),
            None => panic!("Cell ({}, {}) has no role", s, t),
        }
    }

    if overlaps_with_own_territory != 1 {
        return None;
    }

    Some(PossiblePlacement {
        x: x as usize,
        y: y as usize,
        weight: 0,
    })
}

fn get_distance_to_opponent(anfield: &Anfield, x: usize, y: usize) -> usize {
    let mut visited = vec![false; anfield.cells.len()];
    let mut queue = VecDeque::new();
    visited[y * anfield.width + x] = true;
    queue.push_back((Cell { x, y }, 0));

    while let Some((p, distance)) = queue.pop_front() {
        match anfield.get_cell_role(p.x, p.y) {
            Some(CellRole::OpponentSymbol) | Some(CellRole::OpponentLatestMove) => {
                return distance;
            }
            _ => (),
        }

        for (nx, ny) in neighbors(p.x, p.y, anfield.width, anfield.height) {
            if !visited[ny * anfield.width + nx] {
                visited[ny * anfield.width + nx] = true;
                queue.push_back((Cell { x: nx, y: ny }, distance + 1));
            }
        }
    }

    usize::MAX
}

fn neighbors(x: usize, y: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    if x > 0 {
        result.push((x - 1, y));
    }
    if x + 1 < width {
        result.push((x + 1, y));
    }
    if y > 0 {
        result.push((x, y - 1));
    }
    if y + 1 < height {
        result.push((x, y + 1));
    }
    result
}
