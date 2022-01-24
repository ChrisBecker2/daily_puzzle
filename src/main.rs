#![allow(unused)]
use lazy_static::lazy_static; 
use std::time::{Duration, Instant};
use futures::executor::block_on;
use async_std::{task};
use std::fmt;

const PIECE_SIZE : usize = 4;
type LayoutValues = [[u32; PIECE_SIZE]; PIECE_SIZE];
const EMPTY_LAYOUT_VALUES : LayoutValues = [[0; PIECE_SIZE]; PIECE_SIZE];
const WALL_VALUE : u32 = 100;
const DAY_VALUE : u32 = WALL_VALUE - 1;
const MONTH_VALUE : u32 = WALL_VALUE - 2;

#[derive(Debug)]
#[derive(PartialEq)]
struct Layout
{
    width: usize,
    height: usize,
    values: LayoutValues,
}

#[derive(Debug)]
struct Piece
{
    orientations: Vec<Layout>,
}

#[derive(Debug)]
struct Board
{
    values : [[u32; 7]; 7],
}

impl Board
{
    fn new( _month: u32, _day: u32 ) -> Self
    {
        if _month <= 0 || _month > 12 || _day <= 0 || _day > 31
        {
            panic!("Invalidate date supplied");
        }

        let mut b : [[u32; 7]; 7] = [[0; 7]; 7];

        // fill top right
        b[0][6] = WALL_VALUE;
        b[1][6] = WALL_VALUE;

        // fill in bottom right
        b[6][6] = WALL_VALUE;
        b[6][5] = WALL_VALUE;
        b[6][4] = WALL_VALUE;
        b[6][3] = WALL_VALUE;

        let month = _month - 1;
        let day = _day - 1;

        // fill in month
        b[(month / 6) as usize][(month % 6) as usize] = MONTH_VALUE;

        // fill in date
        b[(day / 7 + 2) as usize][(day % 7) as usize] = DAY_VALUE;

        Board { values : b }
    }

    fn place_layout(&mut self, layout: &Layout, at_x: usize, at_y: usize,) -> bool
    {
        // does it fit on the board?
        if at_x + layout.width > self.values[0].len() || at_y + layout.height > self.values.len()
        {
            return false;
        }

        let mut placed = true;

        // add the piece to the board
        for y in 0..layout.height
        {
            for x in 0..layout.width
            {
                let val = & mut self.values[y + at_y][x + at_x];
                let piece_val = layout.values[y][x];
                placed = placed && ( *val == 0 || piece_val == 0 );
                *val += layout.values[y][x];
            }
        } 

        // if it overlapped with another piece, remove it
        if !placed
        {
            self.remove_layout(layout, at_x, at_y );
            return false;
        }

        return true;
    }

    fn remove_layout(&mut self, layout: &Layout, at_x: usize, at_y: usize)
    {
        // subtract the piece
        for y in 0..layout.height
        {
            for x in 0..layout.width
            {
                self.values[y + at_y][x + at_x] -= layout.values[y][x];
            }
        } 
    }
}

 // Enable print! for Board
 impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.values
        {
            for value in row
            {
                match value
                {
                    0 => write!(f, "."),
                    WALL_VALUE => write!(f, "X"),
                    DAY_VALUE => write!(f, "D"),
                    MONTH_VALUE => write!(f, "M"),
                    _ => write!(f, "{}", value),
                };
                write!(f, " ");
            }

            writeln!(f);
        }

        Ok(())
    }
}

fn add_all_rotations( shape: LayoutValues, width: usize, height: usize, orientations: &mut Vec<Layout> )
{
    let mut flip_dimensions = false;
    for i in 0..4
    {
        let mut values = EMPTY_LAYOUT_VALUES;
        for y in 0..height
        {
            for x in 0..width
            {
                match i
                {
                    0 => values[y][x] = shape[y][x], // 0 degrees
                    1 => values[x][height - y - 1] = shape[y][x], // 90 degrees
                    2 => values[height - y - 1][width - x - 1] = shape[y][x], // 180 degrees
                    3 => values[width - x - 1][y] = shape[y][x], // 270 degrees
                    _ => panic!()
                }
            }
        }

        let layout = Layout{ 
            width: if flip_dimensions { height } else {width},
            height: if flip_dimensions { width } else {height},
            values 
        };

        // only add layouts that we don't have already (reduces pieces that have symmetry)
        if !orientations.contains(&layout)
        {
            orientations.push(layout);
        }

        flip_dimensions = !flip_dimensions;
    }
}

impl Piece
{
    fn new( width: usize, height: usize, multiplier: usize, shape: LayoutValues ) -> Self
    {
        let mut orientations = Vec::new();

        // apply multiplier so each piece has unique number
        let mut multiplied_shape = EMPTY_LAYOUT_VALUES;
        for y in 0..height
        {
            for x in 0..width
            {
                multiplied_shape[y][x] = shape[y][x] * multiplier as u32;   
            }
        }

        // add all rotations of the shape
        add_all_rotations( multiplied_shape, width, height, &mut orientations );  

        // flip the shape and add rotations
        let mut flipped = EMPTY_LAYOUT_VALUES;
        for y in 0..height
        {
            for x in 0..width
            {
                flipped[y][width - x - 1] = multiplied_shape[y][x];
            }
        }

        add_all_rotations( flipped, width, height, &mut orientations );

        Piece
        {
            orientations
        }
    }
}

// initalize all pieces
lazy_static! {
    #[derive(Debug)]
    static ref ALL_PIECES:  Vec<Piece> = {
       let mut v = Vec::new();

       v.push( Piece::new( 4, 2, v.len()+1, [
            [0,0,1,0],
            [1,1,1,1],
            [0,0,0,0],
            [0,0,0,0]] ));

       v.push( Piece::new( 3, 2, v.len()+1, [
            [1,1,1,0],
            [1,1,1,0],
            [0,0,0,0],
            [0,0,0,0]] ));

        v.push( Piece::new( 4, 2, v.len()+1, [
            [1,0,0,0],
            [1,1,1,1],
            [0,0,0,0],
            [0,0,0,0]] ));

        v.push( Piece::new( 3, 3, v.len()+1, [
            [1,0,0,0],
            [1,0,0,0],
            [1,1,1,0],
            [0,0,0,0]] ));

        v.push( Piece::new( 4, 2, v.len()+1, [
            [1,1,1,0],
            [0,0,1,1],
            [0,0,0,0],
            [0,0,0,0]] ));

        v.push( Piece::new( 3, 2, v.len()+1, [
            [1,1,1,0],
            [0,1,1,0],
            [0,0,0,0],
            [0,0,0,0]] ));
        
        v.push( Piece::new( 3, 2, v.len()+1, [
            [1,1,1,0],
            [1,0,1,0],
            [0,0,0,0],
            [0,0,0,0]] ));

        v.push( Piece::new( 3, 3, v.len()+1, [
            [1,1,0,0],
            [0,1,0,0],
            [0,1,1,0],
            [0,0,0,0]] ));

        v
    };
}


fn recurse( piece_index : usize, start_row : usize, board : & mut Board ) -> bool
{
    if piece_index >= ALL_PIECES.len()
    {
        return true;
    }

    let ref piece = ALL_PIECES[piece_index];

    for layout in &piece.orientations
    {
        let max_y = board.values.len() - layout.height + 1;
        let max_x = board.values[0].len() - layout.width + 1;

        // place the piece at each location
        for y in start_row..max_y
        {
            for x in 0..max_x
            {
                if board.place_layout( layout, x, y )
                {
                   /* if piece_index == 0
                    {
                        println!("{}", board);
                        //println!("{}, {}", layout.width, layout.height);
                        println!();
                    }*/

                    // start farther down if a piece can't reach the upper rows which should be filled by now
                    let new_start_row = 0; //if y > PIECE_SIZE { y - PIECE_SIZE } else {0};

                    if recurse( piece_index + 1, new_start_row, board )
                    {
                        return true;
                    }

                    board.remove_layout( layout, x, y );
                }
            }
        }
    }

    return false;
}

async fn solve( day: u32, month: u32 ) -> ( bool, Board ) 
{
    let mut board = Board::new( day, month );

    (recurse(0, 0, & mut board), board)
}

async fn solve_and_print( month : u32, day: u32 )
{
    let mut v = Vec::new();

    if month == 0 || day == 0
    {
        for m in 1..12+1
        {
            for d in 1..31+1
            {
                v.push( (m, d, task::spawn( solve( m, d ) ) ) );
            }
        }
    }
    else
    {
        v.push( (month, day, task::spawn( solve( month, day ) ) ) );
    }

    for result in v
    {
        let solution = result.2.await;
        if(solution.0)
        {
            println!("Month {}, Day {}", result.0, result.1);
            println!("{}", solution.1)
        }
        else
        {
            println!("Month {}, Day {} - No solution found!", result.0, result.1);
            panic!();
        }
        println!();
    }
}

fn main() {

    let now = Instant::now();

    // solve all
    block_on(solve_and_print(0, 0));

    // solve one
    //block_on(solve_and_print(2, 15));

    println!("Runtime took {} seconds.", now.elapsed().as_millis() as f64 / 1000.0 );
}