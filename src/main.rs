use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 1280;
const SCREEN_HEIGHT: i32 = 800;
const GREEN: Color = Color {
    r: 38,
    g: 185,
    b: 154,
    a: 255,
};
const DARK_GREEN: Color = Color {
    r: 20,
    g: 160,
    b: 133,
    a: 255,
};
const LIGHT_GREEN: Color = Color {
    r: 129,
    g: 204,
    b: 184,
    a: 255,
};
const YELLOW: Color = Color {
    r: 243,
    g: 213,
    b: 91,
    a: 255,
};

struct Ball {
    x: i32,
    y: i32,
    speed_x: i32,
    speed_y: i32,
    radius: f32,
}

impl Default for Ball {
    fn default() -> Self {
        Self {
            x: SCREEN_WIDTH / 2,
            y: SCREEN_HEIGHT / 2,
            radius: 20f32,
            speed_x: 7,
            speed_y: 7,
        }
    }
}

impl Ball {
    fn draw(self: &mut Self, brush: &mut RaylibDrawHandle) {
        brush.draw_circle(self.x, self.y, self.radius, YELLOW);
    }

    fn tick(self: &mut Self, player_score: &mut i32, cpu_score: &mut i32, rl: &mut RaylibHandle) {
        if self.y + self.radius as i32 >= SCREEN_HEIGHT || self.y - self.radius as i32 <= 0 {
            self.speed_y *= -1;
        }

        if self.x + self.radius as i32 >= SCREEN_WIDTH {
            // CPU wins
            *cpu_score += 1;
            self.reset_ball(rl);
        }

        if self.x - self.radius as i32 <= 0 {
            //Player wins
            *player_score += 1;
            self.reset_ball(rl);
        }

        self.x += self.speed_x;
        self.y += self.speed_y;
    }

    fn as_vec2(self: &Self) -> Vector2 {
        Vector2 {
            x: self.x as f32,
            y: self.y as f32,
        }
    }

    fn reset_ball(self: &mut Self, rl: &mut RaylibHandle) {
        self.x = SCREEN_WIDTH / 2;
        self.y = SCREEN_HEIGHT / 2;

        let speeds = [-7, 7];
        self.speed_x = speeds[rl.get_random_value::<i32>(0..1) as usize];
        self.speed_y = speeds[rl.get_random_value::<i32>(0..1) as usize];
    }
}

trait Paddle {
    fn tick(self: &mut Self, _rl: &mut RaylibHandle) -> () {}
    fn draw(self: &mut Self, _brush: &mut RaylibDrawHandle) -> () {}
    fn limit_movement(self: &mut Self) -> () {}
    fn as_rect(self: &Self) -> Rectangle {
        Default::default()
    }
}

struct PlayerPaddle {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    speed: i32,
}

impl Default for PlayerPaddle {
    fn default() -> Self {
        Self {
            x: 10.0,
            y: 10.0,
            w: 25.0,
            h: 120.,
            speed: 6,
        }
    }
}

impl Paddle for PlayerPaddle {
    fn draw(self: &mut Self, brush: &mut RaylibDrawHandle) {
        brush.draw_rectangle_rounded(
            Rectangle {
                x: self.x,
                y: self.y,
                width: self.w,
                height: self.h,
            },
            0.8,
            0,
            Color::WHITE,
        );
    }

    fn tick(self: &mut Self, rl: &mut RaylibHandle) {
        if rl.is_key_down(KeyboardKey::KEY_UP) {
            self.y = self.y - self.speed as f32;
        } else if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            self.y = self.y + self.speed as f32;
        }

        self.limit_movement();
    }

    fn limit_movement(self: &mut Self) -> () {
        self.y = f32::min(f32::max(0., self.y), SCREEN_HEIGHT as f32 - self.h);
    }

    fn as_rect(self: &Self) -> Rectangle {
        Rectangle {
            x: self.x,
            y: self.y,
            width: self.w,
            height: self.h,
        }
    }
}

struct CPUPaddle {
    base: PlayerPaddle, // Rust have no class inheritance, so use base to mimic one.
}

impl Default for CPUPaddle {
    fn default() -> Self {
        let master_obj: PlayerPaddle = Default::default();
        Self {
            base: PlayerPaddle {
                x: 10.,
                y: SCREEN_HEIGHT as f32 / 2.,
                h: master_obj.h,
                w: master_obj.w,
                speed: master_obj.speed,
            },
        }
    }
}

impl CPUPaddle {
    fn tick(self: &mut Self, ball_y: f32, _rl: &mut RaylibHandle) {
        if self.base.y + self.base.h / 2. > ball_y {
            self.base.y = self.base.y - self.base.speed as f32;
        } else if self.base.y + self.base.h / 2. <= ball_y {
            self.base.y = self.base.y + self.base.speed as f32;
        }

        self.base.limit_movement();
    }
}

impl Paddle for CPUPaddle {
    fn draw(self: &mut Self, brush: &mut RaylibDrawHandle) {
        self.base.draw(brush);
    }

    fn as_rect(self: &Self) -> Rectangle {
        self.base.as_rect()
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Is Pong!")
        .build();
    rl.set_target_fps(60);

    let mut ball: Ball = Default::default();
    let mut player: PlayerPaddle = Default::default();
    let mut cpu_paddle: CPUPaddle = Default::default();

    let mut player_score: i32 = 0;
    let mut cpu_score: i32 = 0;

    player.x = SCREEN_WIDTH as f32 - player.w - 10.;
    player.y = SCREEN_HEIGHT as f32 / 2. - player.h / 2.;

    while !rl.window_should_close() {
        ball.tick(&mut player_score, &mut cpu_score, &mut rl);
        player.tick(&mut rl);
        cpu_paddle.tick(ball.y as f32, &mut rl);

        let player_paddel_rect = player.as_rect();
        let cpu_paddel_rect = cpu_paddle.as_rect();
        let ball_pos = ball.as_vec2();

        if player_paddel_rect.check_collision_circle_rec(ball_pos, ball.radius) {
            ball.speed_x *= -1;
        }

        if cpu_paddel_rect.check_collision_circle_rec(ball_pos, ball.radius) {
            ball.speed_x *= -1;
        }

        let mut brush = rl.begin_drawing(&thread);
        brush.clear_background(DARK_GREEN);
        brush.draw_rectangle(SCREEN_WIDTH / 2, 0, SCREEN_WIDTH, SCREEN_HEIGHT, GREEN);
        brush.draw_circle(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, 150., LIGHT_GREEN);

        brush.draw_text(
            format!("{}", cpu_score).as_str(),
            SCREEN_WIDTH / 4 - 20,
            20,
            80,
            Color::WHITE,
        );
        brush.draw_text(
            format!("{}", player_score).as_str(),
            3 * SCREEN_WIDTH / 4 - 20,
            20,
            80,
            Color::WHITE,
        );

        brush.draw_line(
            SCREEN_WIDTH / 2,
            0,
            SCREEN_WIDTH / 2,
            SCREEN_HEIGHT,
            Color::WHITE,
        );
        ball.draw(&mut brush);
        player.draw(&mut brush);
        cpu_paddle.draw(&mut brush);
    }
}
