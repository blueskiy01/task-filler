use std::collections::VecDeque;

use crate::{find_::{Finder, Compas, MajorStrategy}, parse_::Parser, debug::{DEBUG_FILE, append_to_file}};

impl Finder {
  /** todo: find the optimal position for the piece
  * iterate all possible positions for the piece and in time of iteration:
  * 
  * find the most agressively placed enemy cell, opposite the major direction
  * (placed on the player territory, or aimed closer to the player territory)
  *
  * find the most agressively placed player cell, on the major direction
  * (placed on the enemy territory, or aimed closer to the enemy territory)
  *
  * //todo: refactor bottom comments, to possible one iteration for all, with save, probably two possible answers, before choose the best one
  * find all possible variants to place the piece on the field correctly
  *
  * iterate the correct variants
  * check if there is variant to place the piece on the field that some cell of that piece
  * will be placed not the less agressively than the enemy cell, so the same or more agressively
  * from the enemy position. if there is such variant - place the piece there(set the self.answer)
  * to try prevent or restrict the enemy cell to move in the major direction
  *
  * otherwise iterate the correct variants again
  * and place the piece on the field with the most agressively placed player cell, as possible deep
  * in the major direction, to try cover more enemy cells in the major direction
  *
  * otherwise return the surrender answer (the most far enemy position)
  */
  pub fn find_position(&mut self, parser: &mut Parser) -> [usize;2] {
    let anfield = &parser.anfield;
    let piece = &parser.piece;
    let player_char = &parser.player_char;
    let enemy_char = &parser.enemy_char;
    
    /* for more agressive piece cell check */
    let anfield_size_xy = [anfield[0].len(), anfield.len()];
    
    /* the piece top left corner position */
    let mut answer_xy = [usize::MIN, usize::MIN];
    /* the most argessive enemy cell position */
    
    
    //todo: implement.
    /* to default values for correct piece position */
    let mut fresh_calculation:bool = true;
    
    let piece_height = piece.len();
    let piece_width = piece[0].len();
    
    /* prepare iterators depend on self.major direction */
    let y_iterator = match self.major {
      Compas::NE | Compas::N | Compas::NW | Compas::W | Compas::CENTRAL =>
      (0..anfield.len() - piece_height + 1).rev().collect::<Vec<_>>().into_iter(),
      
      Compas::SW | Compas::S | Compas::SE | Compas::E =>
      (0..anfield.len() - piece_height + 1).collect::<Vec<_>>().into_iter(),
    };
    
    let x_iterator = match self.major {
      Compas::NW | Compas::W | Compas::SW | Compas::S =>
      (0..anfield[0].len() - piece_width + 1).rev().collect::<Vec<_>>().into_iter(),
      
      Compas::SE | Compas::E | Compas::NE | Compas::N | Compas::CENTRAL =>
      (0..anfield[0].len() - piece_width + 1).collect::<Vec<_>>().into_iter(),
    };
    
    if self.major_strategy == MajorStrategy::SPEAR {/*agressively invade face direction */
      let mut spear_strategy_still_effective = false; /*false - no more ways to decrease distance from piece to most enemy agressive cell*/
      for y in y_iterator.clone() {
        for x in x_iterator.clone() {
          if self.position_is_correct(anfield, piece, x, y, player_char){
            
            if fresh_calculation {/*use the first found correct position for the piece as default */
              fresh_calculation = false;
              answer_xy = [x, y].clone();
            }
            
            /* the most argessive enemy cell position */
            let most_agressive_enemy_xy = self.find_most_agressive_xy(&parser, enemy_char, self.minor);
            
            let min_distance_from_piece_to_most_agressive_enemy_xy =
            self.find_piece_cell_min_distance_to_cell_xy(
              anfield, piece,
              [x,y].clone(),
              most_agressive_enemy_xy.clone()
            );
            
            // let piece_more_agressive_xy = self.find_more_agressive(
            //   piece,
            //   &[x,y].clone(),
            //   &most_agressive_enemy_xy.clone(),
            //   &self.major.clone(),
            //   &anfield_size_xy.clone()
            // );
            
            // let still_present_distance_before_contact = self.first_more_agressive_than_second(
            //   &most_agressive_enemy_xy,
            //   &piece_more_agressive_xy,
            //   &self.major.clone(),
            //   &anfield_size_xy
            // );
            
            if min_distance_from_piece_to_most_agressive_enemy_xy <
            self.global_min_distance_between_most_agressive_cells
            // && still_present_distance_before_contact //todo this craps all
            { /*still possible decrease the distance to most agressive enemy cell */
              self.global_min_distance_between_most_agressive_cells = min_distance_from_piece_to_most_agressive_enemy_xy.clone();
              spear_strategy_still_effective = true;
              answer_xy = [x, y].clone();
              
            }
            
          }
        }
      }
      
      if spear_strategy_still_effective {
        return answer_xy;
      } else {
        // self.major_strategy = MajorStrategy::FORK;
        fresh_calculation = true;
      }
      
    }
    
    // todo: in some reasons it always worse than without fork strategy
    if self.major_strategy == MajorStrategy::FORK {/*agressively invade to fork sides */
      let mut fork_strategy_still_effective = false; /*false - no more ways to increase the progress to fork directions*/
      
      /* use buffers to rollback finally not used variant
      * after choose which one to use,
      * left or right fork
      */
      let mut buffer_global_max_distance_left_fork =
      self.global_max_distance_proportion_left_fork.clone();
      
      let mut buffer_global_max_distance_right_fork =
      self.global_max_distance_proportion_right_fork.clone();
      
      let mut answer_xy_left_fork = answer_xy.clone();
      let mut answer_xy_right_fork = answer_xy.clone();
      
      for y in y_iterator.clone() {
        for x in x_iterator.clone() {
          if self.position_is_correct(anfield, piece, x, y, player_char){
            
            if fresh_calculation {/*use the first found correct position for the piece as default */
              fresh_calculation = false;
              answer_xy = [x, y].clone();
              answer_xy_left_fork = [x, y].clone();
              answer_xy_right_fork = [x, y].clone();
            }
            
            let max_distance_proportion_left_fork =
            self.find_most_agressive_distnace_proportion_of_piece_cell(
              piece,
              [x,y].clone(),
              self.major_fork_left.clone(),
              &anfield_size_xy.clone()
            );
            
            let max_distance_proportion_right_fork =
            self.find_most_agressive_distnace_proportion_of_piece_cell(
              piece,
              [x,y].clone(),
              self.major_fork_right.clone(),
              &anfield_size_xy.clone()
            );
            
            if max_distance_proportion_left_fork > buffer_global_max_distance_left_fork{
              buffer_global_max_distance_left_fork = max_distance_proportion_left_fork.clone();
              fork_strategy_still_effective = true;
              answer_xy_left_fork = [x, y].clone();
            }
            
            if max_distance_proportion_right_fork > buffer_global_max_distance_right_fork{
              buffer_global_max_distance_right_fork = max_distance_proportion_right_fork.clone();
              fork_strategy_still_effective = true;
              answer_xy_right_fork = [x, y].clone();
            }
            
          }
        }
      }
      
      if fork_strategy_still_effective
      {
        if buffer_global_max_distance_left_fork > buffer_global_max_distance_right_fork
        {
          self.global_max_distance_proportion_left_fork = buffer_global_max_distance_left_fork.clone();
          return answer_xy_left_fork;
        } else {
          self.global_max_distance_proportion_right_fork = buffer_global_max_distance_right_fork.clone();
          return answer_xy_right_fork;
        }
      }
      
      self.major_strategy = MajorStrategy::SPEAR;
    }
    
    
    // now if strategies did not work, then try to find the default answer
    
    
    append_to_file(DEBUG_FILE, &format!("\n====\nanswer_xy: {} {}", answer_xy[0], answer_xy[1])).expect("cannot write to debug file");
    answer_xy
  }
  
  /** 
  the piece position is correct if all(except one) non-empty cells of the piece
  are placed on the empty cells of the field, and only one non-empty cell
  of the piece is placed on the player cell(any cell covered by the
    player char by the player piece placement previously)
    */
    fn position_is_correct(&self, anfield: &VecDeque<VecDeque<char>>, piece: &VecDeque<VecDeque<char>>, x: usize, y: usize, player:&[char;2]) -> bool {
      
      append_to_file(DEBUG_FILE, &format!("inside ===\nanfield {:?}" ,anfield)).expect("cannot write to debug file");
      append_to_file(DEBUG_FILE, &format!("piece {:?}" ,piece)).expect("cannot write to debug file");
      append_to_file(DEBUG_FILE, &format!("x {} y {}" ,x,y)).expect("cannot write to debug file");
      append_to_file(DEBUG_FILE, &format!("player {:?}" ,player)).expect("cannot write to debug file");
      
      /*
      only one cell from the piece must be placed on the player cell, so
      when the player_cells_hovered_by_piece is 1, for all the piece cells,
      the position is correct, otherwise it is not
      */
      let mut player_cells_hovered_by_piece:usize = 0;
      
      /*iterate the piece and compare the cells with the field cells using the x and y incrementation*/
      for (piece_y, field_y) in (0..piece.len()).zip(y..y + piece.len()) { /*vertical row step */
        for (piece_x, field_x) in (0..piece[0].len()).zip(x..x+piece[0].len()) {/*column */
          if piece[piece_y][piece_x] != '.' {/*if the piece cell is not empty*/
            if anfield[field_y][field_x] != '.' {/*if the field cell is not empty*/
              /* both cells (anfield, piece) are not empty, so need extra check*/
              
              if player_cells_hovered_by_piece > 0{/*if at least one player cell is already hovered by piece*/
                return false;/*the piece position is not correct*/
              }
              
              if anfield[field_y][field_x] == player[0] || anfield[field_y][field_x] == player[1] {/*if the field cell is player cell*/
                player_cells_hovered_by_piece += 1;/*increment the player cells hovered by piece*/
              } else {/*if the field cell is enemy cell*/
                return false;/*the piece position is not correct*/
              }
              
            }
          }
        }
      }
      
      if player_cells_hovered_by_piece == 0 {/*if(finally) no player cell is hovered by piece*/
        return false;/*the piece position is not correct*/
      }
      
      true /*the piece position is correct*/
    }
    
  }