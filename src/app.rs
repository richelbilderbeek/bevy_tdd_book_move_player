use crate::game_parameters::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn create_app(game_parameters: GameParameters) -> App {
    let mut app = App::new();
    let add_player_fn = move |/* no mut? */ commands: Commands| {
        add_player_with_sprite_at_pos_with_scale(
            commands,
            game_parameters.initial_player_position,
            game_parameters.initial_player_scale,
        );
    };
    app.add_systems(Startup, add_player_fn);
    let move_player_fn = move |/* no mut? */ query: Query<&mut Transform, With<Player>>| {
        move_player(query, game_parameters.player_velocity);
    };
    app.add_systems(Update, move_player_fn);

    // Do not do update, as this will disallow to do more steps
    // app.update(); //Don't!
    return app;
}

#[cfg(test)]
fn add_player(mut commands: Commands) {
    commands.spawn(Player);
}

fn move_player(mut query: Query<&mut Transform, With<Player>>, velocity: Vec2) {
    let mut player_sprite = query.single_mut();
    player_sprite.translation.x += velocity.x;
    player_sprite.translation.y += velocity.y;
}

fn add_player_with_sprite_at_pos_with_scale(
    mut commands: Commands,
    initial_player_position: Vec3,
    initial_player_scale: Vec3,
) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: initial_player_position,
                scale: initial_player_scale,
                ..default()
            },
            ..default()
        },
        Player,
    ));
}

#[cfg(test)]
fn count_n_players(app: &App) -> usize {
    let mut n = 0;
    for c in app.world.components().iter() {
        // The complete name will be '[crate_name]::Player'
        if c.name().contains("Player") {
            n += 1;
        }
    }
    return n;
}

#[cfg(test)]
fn get_player_coordinat(app: &mut App) -> Vec3 {
    assert_eq!(
        count_n_players(&app),
        1,
        "Do 'app.update()' before calling this function"
    );
    let mut query = app.world.query::<(&Transform, &Player)>();
    let (transform, _) = query.single(&app.world);
    return transform.translation;
}

#[cfg(test)]
fn get_player_scale(app: &mut App) -> Vec3 {
    let mut query = app.world.query::<(&Transform, &Player)>();
    let (transform, _) = query.single(&app.world);
    return transform.scale;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_testing() {
        assert_eq!(1 + 1, 2)
    }

    #[test]
    fn test_can_create_app() {
        create_app(create_default_game_parameters());
    }

    #[test]
    fn test_empty_app_has_no_players() {
        let app = App::new();
        assert_eq!(count_n_players(&app), 0);
    }

    #[test]
    fn test_setup_player_adds_a_player() {
        let mut app = App::new();
        assert_eq!(count_n_players(&app), 0);
        app.add_systems(Startup, add_player);
        app.update();
        assert_eq!(count_n_players(&app), 1);
    }

    #[test]
    fn test_create_app_has_a_player() {
        let mut app = create_app(create_default_game_parameters());
        app.update();
        assert_eq!(count_n_players(&app), 1);
    }

    #[test]
    fn test_player_is_at_origin() {
        let mut app = create_app(create_default_game_parameters());
        app.update();
        assert_eq!(get_player_coordinat(&mut app), Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_player_is_at_custom_place() {
        let initial_coordinat = Vec3::new(1.2, 3.4, 5.6);
        let mut game_parameters = create_default_game_parameters();
        game_parameters.initial_player_position = initial_coordinat;
        let mut app = create_app(game_parameters);
        app.update();
        assert_eq!(get_player_coordinat(&mut app), initial_coordinat);
    }

    #[test]
    fn test_player_has_a_custom_scale() {
        let player_scale = Vec3::new(1.1, 2.2, 3.3);
        let mut game_parameters = create_default_game_parameters();
        game_parameters.initial_player_scale = player_scale;
        let mut app = create_app(game_parameters);
        app.update();
        assert_eq!(get_player_scale(&mut app), player_scale);
    }

    #[test]
    fn test_player_moves() {
        use create_default_game_parameters as create_params;
        let mut params = create_params();
        let velocity = Vec2::new(1.1, 2.2);
        params.player_velocity = velocity.clone();
        let mut app = create_app(params);
        app.update(); // Already moves the player
        assert_ne!(
            create_params().initial_player_position,
            get_player_coordinat(&mut app)
        );
        // This more precise test will probably be removed in the future
        let expected_pos =
            create_params().initial_player_position + Vec3::new(velocity.x, velocity.y, 0.0);
        assert_eq!(expected_pos, get_player_coordinat(&mut app));
    }
}
