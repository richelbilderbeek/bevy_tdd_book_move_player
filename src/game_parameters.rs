use bevy::prelude::*;

pub struct GameParameters {
    pub initial_player_position: Vec3,
    pub initial_player_scale: Vec3,
    pub initial_player_velocity: Vec2,
}

#[cfg(test)]
pub fn create_default_game_parameters() -> GameParameters {
    return GameParameters {
        initial_player_position: Vec3::new(0.0, 0.0, 0.0),
        initial_player_scale: Vec3::new(100.0, 20.0, 1.0),
        initial_player_velocity: Vec2::new(0.0, 0.0),
    };
}

pub fn create_default_game_parameters_with_player_velocity(
    initial_player_velocity: Vec2,
) -> GameParameters {
    return GameParameters {
        initial_player_position: Vec3::new(0.0, 0.0, 0.0),
        initial_player_scale: Vec3::new(100.0, 20.0, 1.0),
        initial_player_velocity: initial_player_velocity,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default_game_parameters() {
        create_default_game_parameters();
    }
    #[test]
    fn test_initial_player_position() {
        assert_eq!(
            create_default_game_parameters().initial_player_position,
            Vec3::new(0.0, 0.0, 0.0)
        );
    }

    #[test]
    fn test_initial_player_scale() {
        assert_eq!(
            create_default_game_parameters().initial_player_scale,
            Vec3::new(100.0, 20.0, 1.0)
        );
    }

    #[test]
    fn test_initial_player_velocity() {
        assert_eq!(
            create_default_game_parameters().initial_player_velocity,
            Vec2::new(0.0, 0.0)
        );
    }
}
