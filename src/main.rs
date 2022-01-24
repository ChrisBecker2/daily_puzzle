#![allow(unused)]
use lazy_static::lazy_static; 

const PIECE_SIZE : usize = 4;
type Layout = [[u8; PIECE_SIZE]; PIECE_SIZE];
const EMPTY_LAYOUT : Layout = [[0; PIECE_SIZE]; PIECE_SIZE];
const FILLER_VALUE : u8 = 255;

#[derive(Debug)]
struct Piece
{
    width: usize,
    height: usize,
    flippable: bool,
    orientations: Vec<Layout>,
}

struct Board
{
    val : [[u8; 7]; 7],
}


impl Board
{
    fn new( month: u8, day: u8 )
    {
        let mut b : [[u8; 7]; 7] = [[0; 7]; 7];

        // fill top right
        b[0][6] = FILLER_VALUE;
        b[1][6] = FILLER_VALUE;

        // fill in bottom right
        b[6][6] = FILLER_VALUE;
        b[6][5] = FILLER_VALUE;
        b[6][4] = FILLER_VALUE;
        b[6][3] = FILLER_VALUE;

        // fill in month
        b[month / 6 as usize][month % 6 as usize] = FILLER_VALUE;

        // fill in date
        b[day / 7 + 2 as usize][day % 7 as usize] = FILLER_VALUE;

        Board { val : b }
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


fn main() {
    println!("{}", ALL_PIECES.len());
    println!("{:?}", ALL_PIECES[3]);
}