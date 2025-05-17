use super::*;


pub fn update_placement(
    mut pieces: Query<(&mut Transform, &Index), (Changed<Index>, With<ChessPiece>)>,
    window_size: Res<WindowSize>,
) {
    for (mut transform, index) in &mut pieces {
        transform.translation = index_to_pixel_coords(index.0, window_size.0).into();
    }
}