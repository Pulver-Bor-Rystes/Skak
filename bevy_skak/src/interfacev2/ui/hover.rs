use super::*;


pub fn hover(
    pieces: Query<(&Name, Entity, &Transform, Option<&Hover>), With<IsHoverable>>,
    mut last_highlight: Local<HashSet<Entity>>,
    mouse: Res<MousePosition>,
    tile_size: Res<TileSize>,
    mut commands: Commands,
) { 
    if mouse.is_none() { return }
    let mouse = mouse.unwrap();
    
    
    for (name, piece_entity, transform, is_hovering) in &pieces {
        
        let translation = transform.translation.xy();

        if mouse.x >= translation.x - tile_size.0 / 2.0 && mouse.x <= translation.x + tile_size.0 / 2.0 && mouse.y >= translation.y - tile_size.0 / 2.0 && mouse.y <= translation.y + tile_size.0 / 2.0 {
            if (*last_highlight).contains(&piece_entity) { continue }
            (*last_highlight).insert(piece_entity);

            info!("Hovering: {}", name);
            
            // tilføj komponent
            commands.entity(piece_entity).try_insert(Hover);
        }
        else if is_hovering.is_some() {
            (*last_highlight).remove(&piece_entity);
            // hvis ikke musen er over brikken, og Hover er på, så skal den fjernes!
            commands.entity(piece_entity).remove::<Hover>();
        }
    }
}



pub fn spawn_hightlight_on_hovered(
    mut commands: Commands,
    chess_pieces: Query<(Entity, Option<&Hover>, Option<&HasHoverIcon>), With<IsHoverable>>,
    asset_server: Res<AssetServer>,
    window_size: Res<WindowSize>,
) {
    for (entity, is_hovering, has_icon) in &chess_pieces {
        match (is_hovering.is_some(), has_icon.is_some()) {
            (true, false) => {
                let child = commands.spawn((
                    Name::new("Hover"),
                    Sprite {
                        image: asset_server.load("hover.png"),
                        ..default()
                    },
                    ChildOf(entity),
                    Transform::default()
                        .with_scale(Vec3::splat((window_size.0 / 8.0) / 64.0)),
                    Visibility::default(),
                )).id();

                commands.entity(entity).insert(HasHoverIcon(child));
            },
            (false, true) => {
                let child_entity = has_icon.unwrap().0;
                commands.entity(child_entity).despawn();
                commands.entity(entity).remove::<HasHoverIcon>();
            },
            (_, _) => {},
        }
    }
}
