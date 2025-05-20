use crate::chess::chess_types::Index144;


pub fn index_to_pixel_coords(index: Index144, window_size: f32, ui_orientation: bool) -> (f32, f32, f32) {
    if !ui_orientation {
        index_to_pixel_coords_inverse(index, window_size)
    }
    else {
        let tile_size = window_size / 8.0;
        let x = (index.u8() % 8) as f32 * tile_size;
        let y = (index.u8() / 8) as f32 * tile_size;
    
        let centered_x = x - window_size / 2.0 + tile_size / 2.0;
        let centered_y = window_size / 2.0 - y - tile_size / 2.0;
        
        (centered_x, centered_y, 0.0)
    }
}

pub fn index_to_pixel_coords_inverse(index: Index144, window_size: f32) -> (f32, f32, f32) {
    let tile_size = window_size / 8.0;
    let x = (7.0 - (index.u8() % 8) as f32) * tile_size;
    let y = (7 - (index.u8() / 8)) as f32 * tile_size;

    let centered_x = x - window_size / 2.0 + tile_size / 2.0;
    let centered_y = window_size / 2.0 - y - tile_size / 2.0;
    
    (centered_x, centered_y, 0.0)
}


pub fn index_64_to_144(index_64: i32) -> i32 {
    let (x, y) = (index_64 % 8, index_64 / 8);
    let new_x = x + 2;
    let new_y = y + 2;
    new_y * 12 + new_x
}


pub fn index_144_to_64(index_144: i32) -> Option<i32> {
    if index_144 == 0 {
        return Some(0);
    }
    
    let x = index_144 % 12;
    let y = index_144 / 12;

    if (2..10).contains(&x) && (0..10).contains(&y) {
        let local_x = x - 2;
        let local_y = y - 2;
        Some(local_y * 8 + local_x)
    } else {
        None // index_144 is outside the central 8×8 block
    }
}


pub fn index_64_to_algebraic(index: Index144) -> String {
    let index = index.i8();
    
    if index >= 64 {
        panic!("aaaaaaaaah");
    }
    let file = (index % 8) as u8;
    let rank = 7-(index / 8) as u8;

    let file_char = (b'a' + file) as char;
    let rank_char = (b'1' + rank) as char;

    format!("{}{}", file_char, rank_char)
}


// pub fn algebraic_to_index_144(square: &str) -> usize {
//     if square.len() != 2 {
//         panic!("shiiiit");
//     }

//     let bytes = square.as_bytes();
//     let file = bytes[0];
//     let rank = bytes[1];

//     if !(b'a'..=b'h').contains(&file) || !(b'1'..=b'8').contains(&rank) {
//         panic!("fuuuck");
//     }

//     let file_index = (file - b'a') as i32;
//     let rank_index = 7-(rank - b'1') as i32;

//     let index_64 = rank_index * 8 + file_index;
//     index_64_to_144(index_64) as usize
// }


pub fn iter_len(something: impl Iterator) -> i32 {
    let mut len = 0;
    something.for_each(|_| len += 1);
    return len;
}