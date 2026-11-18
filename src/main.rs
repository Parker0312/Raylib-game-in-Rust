use raylib::prelude::*;

fn main() {

    let screen_width: i32 = 960;
    let screen_height: i32 = 720;

    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Window")
        .build();

    rl.set_target_fps(60);

    let mut player_pos = Vector2::new(100.0, screen_height as f32 / 2.0);
    let player_radius = 30.0;

    let mut enemy_pos = Vector2::new(screen_width as f32, rl.get_random_value::<i32>(0..=720) as f32);
    let enemy_radius: f32 = 30.0;

    let mut player_velocity: f32 = 0.0;

    let mut score: i32 = 0;

    let mut speed: f32 = 500.0;

    while !rl.window_should_close(){
        let dt = rl.get_frame_time();
        
        player_pos.y += player_velocity;
        player_velocity += 20.0 * dt;

        enemy_pos.x -= speed * dt;

        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            player_velocity = 0.0;
            player_velocity -= 10.0;
        }

        // long ass if statement bruh
        if check_collision_circles(player_pos, player_radius, enemy_pos, enemy_radius) || player_pos.y <= -50.0 || player_pos.y >= screen_height as f32 {
            player_velocity = 0.0;
            player_pos.y = screen_height as f32 / 2.0;
            enemy_pos.x = screen_width as f32;
            enemy_pos.y = rl.get_random_value::<i32>(0..=720) as f32;
            score = 0;
            speed = 500.0;
        }

        if enemy_pos.x <= 0.0 {
            enemy_pos.x = screen_width as f32;
            enemy_pos.y = rl.get_random_value::<i32>(0..=720) as f32;
            score += 1;
            speed += 50.0;
        }
        
        if speed >= 1650.0 {
            speed = 1650.0;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_circle_v(player_pos, player_radius, Color::WHITE);
        d.draw_circle_v(enemy_pos, enemy_radius, Color::WHITE);
        d.draw_text(&format!("Score: {}", score), 10, 10, 32, Color::WHITE);
        d.draw_text(&format!("Speed: {}", speed), 10, 50, 32, Color::WHITE);
    }
}

