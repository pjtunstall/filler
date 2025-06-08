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
        if possible_placement.distance_to_opponent > chosen_possible_placement.distance_to_opponent
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
        // Dedending on the rules, we may need to replaces these `usize` with signed integers, in which case we'll need a check that they're nonnegative too.
        if s >= anfield.width || t >= anfield.height {
            return None;
        }
        let cell_role = anfield.get_cell_role(s, t);
        match cell_role {
            Some(cell_role) => match cell_role {
                CellRole::OpponentSymbol | CellRole::OpponentLatestMove => return None,
                CellRole::OwnSymbol | CellRole::OwnLatestMove => overlaps_with_own_territory += 1,
                _ => continue,
            },
            None => panic!("Cell ({}, {}) has no role", s, t),
        }
        if overlaps_with_own_territory == 2 {
            return None;
        }
        overlaps_with_own_territory = 0;
    }
    Some(PossiblePlacement {
        x,
        y,
        distance_to_opponent: 0,
    })
}

fn get_distance_to_opponent(anfield: &Anfield, x: usize, y: usize) -> usize {
    let mut distance = 0;
    let mut q = Vec::new();
    let mut visited = vec![false; anfield.cells.len()];
    visited[y * anfield.width + x] = true;
    q.push(Cell { x, y });

    while !q.is_empty() {
        let p = q.pop().unwrap();
        match anfield.get_cell_role(p.x, p.y) {
            Some(CellRole::OpponentSymbol) | Some(CellRole::OpponentLatestMove) => break,
            _ => (),
        }
        distance += 1;

        if p.x > 0 {
            try_visit(anfield, &mut q, &mut visited, p.x - 1, p.y);
        }
        if p.x < anfield.width - 1 {
            try_visit(anfield, &mut q, &mut visited, p.x + 1, p.y);
        }
        if p.y > 0 {
            try_visit(anfield, &mut q, &mut visited, p.x, p.y - 1);
        }
        if p.y < anfield.height - 1 {
            try_visit(anfield, &mut q, &mut visited, p.x, p.y + 1);
        }
    }

    distance
}

fn try_visit(anfield: &Anfield, q: &mut Vec<Cell>, visited: &mut Vec<bool>, s: usize, t: usize) {
    if visited[t * anfield.width + s] {
        return;
    }
    match anfield.get_cell_role(s, t) {
        Some(CellRole::OwnSymbol) | Some(CellRole::OwnLatestMove) => return,
        _ => (),
    }
    visited[t * anfield.width + s] = true;
    q.push(Cell { x: s, y: t });
}
