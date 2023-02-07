use rand::Rng;
use rusty_engine::prelude::*;

struct GameState {
    high_score: u32,
    score: u32,
    ferris_index: i32,
    spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        return Self {
            high_score: 0,
            score: 0,
            ferris_index: 0,
            spawn_timer: Timer::from_seconds(1.0, true),
        };
    }
}

fn main() {
    let mut game = Game::new();
    game.window_settings(WindowDescriptor {
        title: "Rusty Game".to_string(),
        ..WindowDescriptor::default()
    });
    game.audio_manager.play_music(MusicPreset::Classy8Bit, 0.1);

    let player = game.add_sprite("player", SpritePreset::RacingCarRed);
    player.translation = Vec2::new(0.0, 0.0);
    player.rotation = EAST;
    player.collision = true;

    let score = game.add_text("score", "Score: 0");
    score.translation = Vec2::new(520.0, 420.0);

    let high_score = game.add_text("high_score", "High Score: 0");
    high_score.translation = Vec2::new(-520.0, 420.0);

    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    let score = engine.texts.get_mut("score").unwrap();
    score.translation.x = engine.window_dimensions.x / 2.0 - 100.0;
    score.translation.y = engine.window_dimensions.y / 2.0 - 50.0;

    let high_score = engine.texts.get_mut("high_score").unwrap();
    high_score.translation.x = -engine.window_dimensions.x / 2.0 + 100.0;
    high_score.translation.y = engine.window_dimensions.y / 2.0 - 50.0;

    //handle collitsions
    for event in engine.collision_events.drain(..) {
        if event.state == CollisionState::Begin && event.pair.one_starts_with("player") {
            for label in [event.pair.0, event.pair.1] {
                if label != "player" {
                    engine.sprites.remove(&label);
                    game_state.score += 1;
                    let score = engine.texts.get_mut("score").unwrap();
                    score.value = format!("Score: {:?}", game_state.score);

                    let high_score = engine.texts.get_mut("high_score").unwrap();

                    if game_state.high_score == 0 || game_state.high_score <= game_state.score {
                        game_state.high_score = game_state.score;
                        high_score.value = format!("High Score: {:?}", game_state.high_score);
                    }
                    engine.audio_manager.play_sfx(SfxPreset::Minimize1, 0.3);
                }
            }
        }
    }

    //handle movement
    let player = engine.sprites.get_mut("player").unwrap();
    const MOVEMENT_SPEED: f32 = 200.0;

    if engine.keyboard_state.just_pressed(KeyCode::R) {
        game_state.score = 0;
        let score = engine.texts.get_mut("score").unwrap();
        score.value = format!("Score: 0");
    }

    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Up, KeyCode::W])
    {
        player.translation.y += MOVEMENT_SPEED * engine.delta_f32;
        player.rotation = NORTH;
    }
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Down, KeyCode::S])
    {
        player.translation.y -= MOVEMENT_SPEED * engine.delta_f32;
        player.rotation = SOUTH;
    }
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Left, KeyCode::D])
    {
        player.translation.x += MOVEMENT_SPEED * engine.delta_f32;
        player.rotation = EAST;
    }
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Right, KeyCode::A])
    {
        player.translation.x -= MOVEMENT_SPEED * engine.delta_f32;
        player.rotation = WEST;
    }
    let (max_x, max_y) = (
        engine.window_dimensions.x / 2.0,
        engine.window_dimensions.y / 2.0,
    );
    if game_state.spawn_timer.tick(engine.delta).just_finished() {
        let mut rng = rand::thread_rng();
        let label = format!("ferris{}", game_state.ferris_index);
        game_state.ferris_index += 1;
        let ferris = engine.add_sprite(label, "cute-ferrys.png");
        ferris.scale = 0.3;
        ferris.translation.x = rng.gen_range(-max_x..max_x);
        ferris.translation.y = rng.gen_range(-max_y..max_y - 60.0);
        ferris.collision = true;
    }
}
