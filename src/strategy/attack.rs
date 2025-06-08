use std::collections::VecDeque;

use crate::anfield::Anfield;
use crate::cell::Cell;
use crate::piece::{Piece, PossiblePlacement};
use crate::strategy::Strategy;
use crate::symbols::CellRole;

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
        if possible_placement.distance_to_opponent < chosen_possible_placement.distance_to_opponent
        {
            chosen_possible_placement = *possible_placement;
        }
    }
    let x = chosen_possible_placement.x as i32;
    let y = chosen_possible_placement.y as i32;
    [x, y]
}

fn get_possible_placements(anfield: &Anfield, piece: &Piece) -> Vec<PossiblePlacement> {
    let mut possible_placements = Vec::new();
    for x in 0..(anfield.width - piece.width) {
        for y in 0..(anfield.height - piece.height) {
            if let Some(mut possible_placement) = try_fit(anfield, &piece, x, y) {
                possible_placement.distance_to_opponent =
                    get_distance_to_opponent(anfield, possible_placement.x, possible_placement.y);
                possible_placements.push(possible_placement);
            }
        }
    }
    if possible_placements.len() == 0 {
        possible_placements.push(PossiblePlacement::default());
    }
    possible_placements
}

fn try_fit(anfield: &Anfield, piece: &Piece, x: usize, y: usize) -> Option<PossiblePlacement> {
    let mut overlaps_with_own_territory = 0;
    for cell in &piece.shape {
        let s = x + cell.x;
        let t = y + cell.y;

        if s >= anfield.width || t >= anfield.height {
            return None;
        }

        let cell_role = anfield.get_cell_role(s, t);
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
        x,
        y,
        distance_to_opponent: 0,
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
