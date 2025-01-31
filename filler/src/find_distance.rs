use std::collections::VecDeque;

use crate::find_::{Finder, Compas};

impl Finder {
  pub fn find_distance(&mut self, xy1:[usize;2], xy2:[usize;2])-> f64 {
    let x1 = xy1[0] as f64;
    let y1 = xy1[1] as f64;
    let x2 = xy2[0] as f64;
    let y2 = xy2[1] as f64;
    ((x1-x2).powf(2.0) + (y1-y2).powf(2.0)).sqrt()
  }
  
  /** find the minimum distance from the any cell of the team
   * to the cell of the opposite_team_cell_xy,
   * according to symbol of anfield cell.
   * if opposite_team_cell_xy of anfield is a or @,
   * then the team cells is s or $,
   */
  pub fn find_min_distance_from_team_area_to_opposite_team_cell(&mut self,anfield: &VecDeque<VecDeque<char>>, opposite_team_cell_xy:[usize;2])-> f64 {
    let team_char =
    if anfield[opposite_team_cell_xy[1]][opposite_team_cell_xy[0]] == 'a'
    || anfield[opposite_team_cell_xy[1]][opposite_team_cell_xy[0]] == '@'
    {
      ['s','$']
    }
    else if anfield[opposite_team_cell_xy[1]][opposite_team_cell_xy[0]] == 's'
    || anfield[opposite_team_cell_xy[1]][opposite_team_cell_xy[0]] == '$'
    {
      ['a','@']
    }
    else {
      ['\0','\0']
    };

    /*
      if incoming char coordinates were wrong, then return min value
      it will prevent to have result shorter than this distance,
      so the variant will be not chosen as effective
    */
    if team_char[0] == '\0' || team_char[1] == '\0' { return f64::MIN; }

    let mut distance = f64::MAX;
    for y in 0..anfield.len() {
      for x in 0..anfield[0].len() {
        if anfield[y][x] == team_char[0] || anfield[y][x] == team_char[1] {
          let xy = [x,y];
          let current_distance = self.find_distance(xy, opposite_team_cell_xy);
          if current_distance < distance {
            distance = current_distance;
          }
        }
      }
    }
    distance

  }

  /** find the minimum distance from the any piece cell
   * to the cell of the opposite_team_cell_xy
   * no check for the character of the opposite_team_cell_xy
   * be careful to use it
   */
  pub fn find_piece_cell_min_distance_to_cell_xy(
    &mut self,
    anfield: &VecDeque<VecDeque<char>>,
    piece: &VecDeque<VecDeque<char>>,
    piece_left_top_cell_xy:[usize;2],
    opposite_team_cell_xy:[usize;2]
  )-> f64 {
    let mut distance = f64::MAX;
    /*check the opposite_team_cell_xy is not empty cell */
    if anfield[opposite_team_cell_xy[1]][opposite_team_cell_xy[0]] == '.' 
    {
      return distance
    }


    /*
      iterate peice cells inside anfield with incrementation of x and y
      and calculate the distances from the not empty cells '.'
    */
    for piece_y in 0..piece.len() {
      for piece_x in 0..piece[0].len() {
        if piece[piece_y][piece_x] != '.' {
          let xy = [piece_left_top_cell_xy[0] + piece_x, piece_left_top_cell_xy[1] + piece_y];
          let current_distance = self.find_distance(xy, opposite_team_cell_xy);
          if current_distance < distance {
            distance = current_distance;
          }
        }
      }
    }
    distance
    
  }

  /**find the most agressive distance of piece cell on compas way */
  pub fn find_most_agressive_distnace_of_piece_cell(
    &mut self,
    piece: &VecDeque<VecDeque<char>>,
    piece_left_top_cell_xy:[usize;2],
    direction:Compas,
    anfield_size:&[usize;2],
  )-> f64 {
    let mut distance = f64::MIN;
    let xy = [piece_left_top_cell_xy[0], piece_left_top_cell_xy[1]];
    let far_xy = self.find_most_far_xy_opposite_the_direction(
      anfield_size,
      direction
    );

    /* iterate each piece cell */
    for (piece_y, piece_row) in piece.iter().enumerate() {
      for (piece_x, piece_cell) in piece_row.iter().enumerate() {
        /* if the cell is not empty */
        if *piece_cell != '.' {
          let x = piece_x+xy[0];
          let y = piece_y+xy[1];
          /*from far point to piece cell */
          let piece_distance = self.find_distance(
            far_xy,
            [x,y]
          );
          
          if piece_distance > distance {
            distance = piece_distance;
          }
        }
      }
    }

    distance
  }

  /**find the most agressive distance of piece cell on on compas way */
  pub fn find_most_agressive_distnace_proportion_of_piece_cell(
    &mut self,
    piece: &VecDeque<VecDeque<char>>,
    piece_left_top_cell_xy:[usize;2],
    direction:Compas,
    anfield_size:&[usize;2],
  )-> f64 {

    let full_distance = match direction {
      Compas::N | Compas::S => anfield_size[1] as f64,
      Compas::W | Compas::E => anfield_size[0] as f64,
      Compas::NW | Compas::SE | Compas::NE | Compas::SW => (
        (
          anfield_size[0] * anfield_size[0]
          + anfield_size[1] * anfield_size[1]
        ) as f64
      ).sqrt(),
      Compas::CENTRAL => f64::MAX,
    };

    let mut distance = f64::MIN;
    let xy = piece_left_top_cell_xy.clone();
    let far_xy = self.find_most_far_xy_opposite_the_direction(
      anfield_size,
      direction
    );

    /* iterate each piece cell */
    for (piece_y, piece_row) in piece.iter().enumerate() {
      for (piece_x, piece_cell) in piece_row.iter().enumerate() {
        /* if the cell is not empty */
        if *piece_cell != '.' {
          let x = piece_x+xy[0];
          let y = piece_y+xy[1];
          /*from far point to piece cell */
          let piece_distance = self.find_distance(
            far_xy.clone(),
            [x,y].clone()
          );
          
          if piece_distance > distance {
            distance = piece_distance.clone();
          }
        }
      }
    }

    distance / full_distance
  }

}