use bevy::prelude::*;

/// Player elements that cannot be put in a SpriteBundle
#[derive(Component)]
pub struct Player {
    pub velocity: Vec2,
}

pub fn create_app(velocity: Vec2) -> App {
    let mut app = App::new();
    let add_player_fn = move |commands: Commands| {
        add_player(commands, velocity);
    };
    app.add_systems(Startup, add_player_fn);
    app.add_systems(Update, move_player);

    // Do not do update, as this will disallow to do more steps
    // app.update(); //Don't!
    app
}

fn move_player(mut query: Query<(&mut Transform, &Player)>) {
    let (mut player_sprite, player) = query.single_mut();
    player_sprite.translation.x += player.velocity.x;
    player_sprite.translation.y += player.velocity.y;
}

fn add_player(mut commands: Commands, velocity: Vec2) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                scale: Vec3::new(64.0, 32.0, 0.0),
                ..default()
            },
            ..default()
        },
        Player { velocity },
    ));
}

#[cfg(test)]
fn count_n_players(app: &mut App) -> usize {
    let mut query = app.world_mut().query::<&Player>();
    query.iter(app.world_mut()).len()
}

#[cfg(test)]
fn get_player_position(app: &mut App) -> Vec2 {
    let mut query = app.world_mut().query::<(&Transform, &Player)>();
    let (transform, _) = query.single(app.world());
    transform.translation.xy()
}

#[cfg(test)]
fn get_player_size(app: &mut App) -> Vec2 {
    let mut query = app.world_mut().query::<(&Transform, &Player)>();
    let (transform, _) = query.single(app.world());
    transform.scale.xy()
}

#[cfg(test)]
fn get_player_velocity(app: &mut App) -> Vec2 {
    let mut query = app.world_mut().query::<(&Transform, &Player)>();
    let (_, player) = query.single(app.world());
    player.velocity
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_app_has_no_players() {
        let mut app = App::new();
        app.update();
        assert_eq!(count_n_players(&mut app), 0);
    }

    #[test]
    fn test_can_set_and_get_velocity() {
        let velocity = Vec2::new(1.2, 3.4);
        let mut app = create_app(velocity);
        app.update();
        assert_eq!(get_player_velocity(&mut app), velocity);
    }

    #[test]
    fn test_create_app_has_a_player() {
        let velocity = Vec2::new(0.0, 0.0);
        let mut app = create_app(velocity);
        app.update();
        assert_eq!(count_n_players(&mut app), 1);
    }

    #[test]
    fn test_player_starts_at_the_origin() {
        let velocity = Vec2::new(0.0, 0.0);
        let mut app = create_app(velocity);
        app.update();
        assert_eq!(get_player_position(&mut app), Vec2::new(0.0, 0.0));
    }

    #[test]
    fn test_get_player_size() {
        let velocity = Vec2::new(0.0, 0.0);
        let mut app = create_app(velocity);
        app.update();
        assert_eq!(get_player_size(&mut app), Vec2::new(64.0, 32.0));
    }

    #[test]
    fn test_player_moves() {
        let velocity = Vec2::new(1.2, 3.4);
        let mut app = create_app(velocity);
        app.update(); // moves the player
        assert_ne!(get_player_position(&mut app), Vec2::new(0.0, 0.0));
    }
}
