#![allow(unused)]
use lazy_static::lazy_static; 

const PIECE_SIZE : usize = 4;
type Layout = [[u8; PIECE_SIZE]; PIECE_SIZE];
const EMPTY_LAYOUT : Layout = [[0; PIECE_SIZE]; PIECE_SIZE];
const WALL_VALUE : u8 = 100;
const DAY_VALUE : u8 = WALL_VALUE - 1;
const MONTH_VALUE : u8 = WALL_VALUE - 2;

#[derive(Debug)]
struct Piece
{
    width: usize,
    height: usize,
    orientations: Vec<Layout>,
}

struct Board
{
    values : [[u8; 7]; 7],
}

impl Board
{
    fn new( _month: u8, _day: u8 ) -> Self
    {
        if _month <= 0 || _month > 12 || _day <= 0 || _day > 31
        {
            panic!("Invalidate date supplied");
        }

        let mut b : [[u8; 7]; 7] = [[0; 7]; 7];

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

    fn print(&self)
    {
        for row in self.values
        {
            for value in row
            {
                match value
                {
                    0 => print!("."),
                    WALL_VALUE => print!("X"),
                    DAY_VALUE => print!("D"),
                    MONTH_VALUE => print!("M"),
                    _ => print!("{}", value),
                }
                print!(" ");
            }

            println!();
        }
    }

    fn place_layout(&mut self, ref piece: Piece, at_x: usize, at_y: usize,) -> bool
    {
        // does it fit?
        if at_x + piece.width > self.values[0].len() || at_y + piece.width > self.values.len()
        {
            return false;
        }

        let mut sum : u32 = 0;

        for y in 0..piece.height
        {
            for x in 0..piece.width
            {
                self.values[y + at_y][x + at_x] += piece.orientations[y][x];
            }
        } 

        return true;
    }
}

fn add_rotations( shape: Layout, width: usize, height: usize, orientations: &mut Vec<Layout> )
{
     // 0 degrees
     orientations.push( shape );

       /*
        0010
        1111

        10
        10
        11
        10

        1111
        0100

        01
        11
        01
        01*/


    for i in 0..3
    {
        let mut layout = EMPTY_LAYOUT;
        for h in 0..height
        {
            for w in 0..width
            {
                match i
                {
                    0 => layout[w][height - h - 1] = shape[h][w], // 90 degrees
                    1 => layout[height - h - 1][width - w - 1] = shape[h][w], // 180 degrees
                    2 => layout[width - w - 1][h] = shape[h][w], // 270 degrees
                    _ => panic!()
                }
            }
        }
        orientations.push(layout);
    }
}

impl Piece
{
    fn new( width: usize, height: usize, flippable: bool, multiplier: usize, shape: Layout ) -> Self
    {
        let mut orientations = Vec::new();

        let mut multiplied_shape = EMPTY_LAYOUT;
        for h in 0..height
        {
            for w in 0..width
            {
                multiplied_shape[h][w] = shape[h][w] * multiplier as u8;   
            }
        }

        add_rotations( multiplied_shape, width, height, &mut orientations );  

        // 1 flip
        if( flippable )
        {
            let mut flipped = EMPTY_LAYOUT;

            for h in 0..height
            {
                for w in 0..width
                {
                    flipped[h][width - w - 1] = multiplied_shape[h][w];
                }
            }
 
            add_rotations( flipped, width, height, &mut orientations );
        }

        Piece
        {
            width,
            height,
            flippable,
            orientations
        }
    }
}

// initalize all pieces
lazy_static! {
    #[derive(Debug)]
    static ref ALL_PIECES:  Vec<Piece> = {
       let mut v = Vec::new();

       v.push( Piece::new( 4, 2, true, v.len()+1, [
            [0,0,1,0],
            [1,1,1,1],
            [0,0,0,0],
            [0,0,0,0]] ));

       v.push( Piece::new( 3, 2, false, v.len()+1, [
            [1,1,1,0],
            [1,1,1,0],
            [0,0,0,0],
            [0,0,0,0]] ));

        v.push( Piece::new( 4, 2, true, v.len()+1, [
            [1,0,0,0],
            [1,1,1,1],
            [0,0,0,0],
            [0,0,0,0]] ));

        v.push( Piece::new( 3, 3, false, v.len()+1, [
            [1,0,0,0],
            [1,0,0,0],
            [1,1,1,0],
            [0,0,0,0]] ));

        v.push( Piece::new( 4, 2, true, v.len()+1, [
            [1,1,1,0],
            [0,0,1,1],
            [0,0,0,0],
            [0,0,0,0]] ));

        v.push( Piece::new( 3, 2, true, v.len()+1, [
            [1,1,1,0],
            [0,1,1,0],
            [0,0,0,0],
            [0,0,0,0]] ));
        
        v.push( Piece::new( 3, 2, false, v.len()+1, [
            [1,1,1,0],
            [1,0,1,0],
            [0,0,0,0],
            [0,0,0,0]] ));

        v.push( Piece::new( 3, 3, false, v.len()+1, [
            [1,1,0,0],
            [0,1,0,0],
            [0,1,1,0],
            [0,0,0,0]] ));

        v
    };
}


fn recurse( piece_index : usize, board : & mut Board ) -> bool
{
    if ALL_PIECES.len() >= piece_index
    {
        return true;
    }

    let ref piece = ALL_PIECES[piece_index];

    let height = board.values.len() - piece.height;
    let width = board.values[0].len() - piece.width;

    // place the piece at each location
    for y in 0..height
    {
        for x in 0..width
        {
            if board.place_piece( *piece, x, y )
            {
                if recurse( piece_index + 1, board )
                {
                    return true;
                }

                board.remove_piece( piece, x, y );
            }
        }
    }

    // recurse( piece_index + 1, board );
    

    return false;
}

fn solve( day: u8, month: u8 ) -> ( bool, Board )
{
    let mut b = Board::new( day, month );

    let result = recurse(0, & mut b);

    (result, b)
}

fn main() {

    let result = solve( 1, 1 );

    println!("solved: {}", result.0);
    result.1.print();

}