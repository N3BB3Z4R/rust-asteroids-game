use ggez::event::{self, EventHandler, KeyCode, KeyMods};
// use ggez::graphics::{self, Color, DrawMode, Mesh};
use ggez::graphics::{self, Color, DrawMode, Mesh, Text, Font};
use ggez::{Context, ContextBuilder, GameResult};
use rand::Rng;
// use std::f32::consts::PI;
use std::time::Instant;

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;
const ASTEROID_SIZE: f32 = 40.0;
const PLAYER_SIZE: f32 = 12.0;
const BULLET_SIZE: f32 = 4.0;
const BULLET_SPEED: f32 = 400.0;
const ASTEROID_COUNT: usize = 20;
// let mut destroyed_count: f32 = 0;

struct Bullet {
    pos: (f32, f32),
    vel: (f32, f32),
}

struct Asteroid {
    pos: (f32, f32),
    vel: (f32, f32),
    size: f32,
    is_destroyed: bool,
}

struct Particle {
    pos: (f32, f32),
    vel: (f32, f32),
    life: f32, // tiempo de vida de la partícula
}

struct AsteroidsGame {
    player_pos: (f32, f32),
    player_vel: (f32, f32),
    player_angle: f32,
    bullets: Vec<Bullet>,
    asteroids: Vec<Asteroid>,
    last_update: Instant,
    is_game_over: bool,
    destroyed_count: f32,
    particles: Vec<Particle>,
}

impl AsteroidsGame {
    fn new(_ctx: &mut Context) -> GameResult<AsteroidsGame> {
        let player_pos = (WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0);
        let player_vel = (0.0, 0.0);
        let player_angle = 0.0;
        let bullets = Vec::new();
        let mut asteroids = Vec::new();
        let last_update = Instant::now();
        let is_game_over = false;
        // let is_destroyed: bool = false;
        let destroyed_count = 0.0;

        let mut rng = rand::thread_rng();
        for _ in 0..ASTEROID_COUNT {
            let x = rng.gen_range(0.0..WINDOW_WIDTH);
            let y = rng.gen_range(0.0..WINDOW_HEIGHT);
            let vel_x = rng.gen_range(-50.0..50.0);
            let vel_y = rng.gen_range(-50.0..50.0);
            let size = rng.gen_range(15.0..ASTEROID_SIZE);
            asteroids.push(Asteroid {
                pos: (x, y),
                vel: (vel_x, vel_y),
                size,
                is_destroyed: false,
            });
        }

        Ok(AsteroidsGame {
            player_pos,
            player_vel,
            player_angle,
            bullets,
            asteroids,
            last_update,
            is_game_over,
            destroyed_count,
            particles: Vec::new(),
        })
    }

    fn update_player(&mut self, dt: f32) {
        self.player_pos.0 += self.player_vel.0 * dt;
        self.player_pos.1 += self.player_vel.1 * dt;

        // Wrap around the screen
        if self.player_pos.0 > WINDOW_WIDTH {
            self.player_pos.0 = 0.0;
        }
        if self.player_pos.0 < 0.0 {
            self.player_pos.0 = WINDOW_WIDTH;
        }
        if self.player_pos.1 > WINDOW_HEIGHT {
            self.player_pos.1 = 0.0;
        }
        if self.player_pos.1 < 0.0 {
            self.player_pos.1 = WINDOW_HEIGHT;
        }
    }

    fn update_bullets(&mut self, dt: f32) {
        for bullet in &mut self.bullets {
            bullet.pos.0 += bullet.vel.0 * dt;
            bullet.pos.1 += bullet.vel.1 * dt;
        }
        // Remove bullets that are out of bounds
        self.bullets.retain(|bullet| {
            bullet.pos.0 >= 0.0 && bullet.pos.0 <= WINDOW_WIDTH && bullet.pos.1 >= 0.0 && bullet.pos.1 <= WINDOW_HEIGHT
        });
    }

    fn update_asteroids(&mut self, dt: f32) {
        for asteroid in &mut self.asteroids {
            asteroid.pos.0 += asteroid.vel.0 * dt;
            asteroid.pos.1 += asteroid.vel.1 * dt;

            // Wrap around the screen
            if asteroid.pos.0 > WINDOW_WIDTH {
                asteroid.pos.0 = 0.0;
            }
            if asteroid.pos.0 < 0.0 {
                asteroid.pos.0 = WINDOW_WIDTH;
            }
            if asteroid.pos.1 > WINDOW_HEIGHT {
                asteroid.pos.1 = 0.0;
            }
            if asteroid.pos.1 < 0.0 {
                asteroid.pos.1 = WINDOW_HEIGHT;
            }
        }
    }

    fn check_collisions(&mut self) {
        // Check for collisions between player and asteroids
        for asteroid in &self.asteroids {
            let dist = ((self.player_pos.0 - asteroid.pos.0).powi(2)
                + (self.player_pos.1 - asteroid.pos.1).powi(2))
                .sqrt();
            if dist < asteroid.size + PLAYER_SIZE {
                self.is_game_over = true;
                return;
            }
        }

        // Check for collisions between asteroids
        for i in 0..self.asteroids.len() {
            for j in (i + 1)..self.asteroids.len() {
                let asteroid1 = &self.asteroids[i];
                let asteroid2 = &self.asteroids[j];
                let dist = ((asteroid1.pos.0 - asteroid2.pos.0).powi(2)
                    + (asteroid1.pos.1 - asteroid2.pos.1).powi(2))
                    .sqrt();
                if dist < asteroid1.size + asteroid2.size {
                    // Handle collision by bouncing asteroids off each other
                    let temp_vel = asteroid1.vel;
                    self.asteroids[i].vel = asteroid2.vel;
                    self.asteroids[j].vel = temp_vel;
                }
            }
        }

        // Check for collisions between bullets and asteroids
        let mut new_asteroids = Vec::new();
        let mut bullets_to_remove = Vec::new();
        for (bullet_idx, bullet) in self.bullets.iter().enumerate() {
            for asteroid in &mut self.asteroids {
                if asteroid.is_destroyed {
                    continue;
                }
                let dist = ((bullet.pos.0 - asteroid.pos.0).powi(2)
                    + (bullet.pos.1 - asteroid.pos.1).powi(2))
                    .sqrt();
                if dist < asteroid.size {
                    bullets_to_remove.push(bullet_idx);
                    asteroid.is_destroyed = true;
                    self.destroyed_count += 1.0;

                    // Dividir el asteroide en piezas más pequeñas
                    if asteroid.size > 10.0 {
                        let new_size = asteroid.size / 2.0;
                        let new_vel = (-asteroid.vel.0, -asteroid.vel.1);
                        new_asteroids.push(Asteroid {
                            pos: asteroid.pos,
                            vel: new_vel,
                            size: new_size,
                            is_destroyed: false,
                        });
                        new_asteroids.push(Asteroid {
                            pos: asteroid.pos,
                            vel: asteroid.vel,
                            size: new_size,
                            is_destroyed: false,
                        });
                    }
                    // Generar partículas de explosión
                    // self.generate_explosion(asteroid.pos);
                }
            }
        }

        // Remove hit bullets
        bullets_to_remove.sort_unstable_by(|a, b| b.cmp(a));
        for idx in bullets_to_remove {
            self.bullets.remove(idx);
        }

        self.asteroids.extend(new_asteroids);
        self.asteroids.retain(|asteroid| !asteroid.is_destroyed && asteroid.size > 10.0);
        // let destroyed_count = self.asteroids.iter().filter(|asteroid| asteroid.is_destroyed).count() as f32;

        // Update score or display destroyed count
        // println!("Asteroids destroyed: {}", destroyed_count); // Print for debugging or use for UI
    }

    fn generate_explosion(&mut self, pos: (f32, f32)) {
        let num_particles = 20;
        for _ in 0..num_particles {
            let angle = rand::random::<f32>() * std::f32::consts::PI * 2.0;
            let speed = rand::random::<f32>() * 100.0;
            self.particles.push(Particle {
                pos,
                vel: (speed * angle.cos(), speed * angle.sin()),
                life: 1.0, // tiempo de vida de la partícula
            });
        }
    }

    fn update_particles(&mut self, dt: f32) {
        for particle in &mut self.particles {
            particle.pos.0 += particle.vel.0 * dt;
            particle.pos.1 += particle.vel.1 * dt;
            particle.life -= dt;
        }
        self.particles.retain(|particle| particle.life > 0.0);
    }

    fn draw_particles(&self, ctx: &mut Context) -> GameResult<()> {
        for particle in &self.particles {
            let particle_mesh = Mesh::new_circle(
                ctx,
                DrawMode::fill(),
                ggez::mint::Point2 { x: 0.0, y: 0.0 },
                2.0,
                0.1,
                Color::WHITE,
            )?;
            graphics::draw(
                ctx,
                &particle_mesh,
                (ggez::mint::Point2 {
                    x: particle.pos.0,
                    y: particle.pos.1,
                },),
            )?;
        }
        Ok(())
    }

    fn shoot(&mut self) {
        let (dir_x, dir_y) = (self.player_angle.cos(), self.player_angle.sin());
        let bullet = Bullet {
            pos: self.player_pos,
            vel: (dir_x * BULLET_SPEED, dir_y * BULLET_SPEED),
        };
        self.bullets.push(bullet);
    }

    // fn draw_score(&self, ctx: &mut Context) -> GameResult<()> {
    //     let score_text = Text::new(format!("Puntuación: {}", self.destroyed_count));
    //     let (width, _) = graphics::drawable_size(ctx);
    //     let dest_point = ggez::mint::Point2 { x: width - 150.0, y: 20.0 };
    //     graphics::draw(ctx, &score_text, (dest_point, 0.0, Color::WHITE))?;
    //     Ok(())
    // }
    fn draw_score(&self, ctx: &mut Context) -> GameResult<()> {
        let font = Font::default();
        // let scale = Scale::uniform(24.0); // Ajusta el tamaño del texto aquí
        let score_text = Text::new((format!("Puntuación: {}", self.destroyed_count), font, 26.0));
        let (width, _) = graphics::drawable_size(ctx);
        let dest_point = ggez::mint::Point2 { x: width - 200.0, y: 20.0 }; // Ajusta la posición según sea necesario
        graphics::draw(ctx, &score_text, (dest_point, 0.0, Color::WHITE))?;
        Ok(())
    }

    fn restart_game(&mut self) {
        self.player_pos = (WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0);
        self.player_vel = (0.0, 0.0);
        self.player_angle = 0.0;
        self.bullets.clear();
        self.asteroids.clear();
        self.is_game_over = false;

        let mut rng = rand::thread_rng();
        for _ in 0..ASTEROID_COUNT {
            let x = rng.gen_range(0.0..WINDOW_WIDTH);
            let y = rng.gen_range(0.0..WINDOW_HEIGHT);
            let vel_x = rng.gen_range(-50.0..50.0);
            let vel_y = rng.gen_range(-50.0..50.0);
            let size = rng.gen_range(15.0..ASTEROID_SIZE);
            self.asteroids.push(Asteroid {
                pos: (x, y),
                vel: (vel_x, vel_y),
                size,
                is_destroyed: false,
            });
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);
        let destroyed_count = self.asteroids.iter().filter(|asteroid| asteroid.is_destroyed).count();

        // Draw player ship as a triangle
        let player_points = [
            [0.0, -PLAYER_SIZE],
            [-PLAYER_SIZE / 2.0, PLAYER_SIZE],
            [PLAYER_SIZE / 2.0, PLAYER_SIZE],
        ];
        let player_mesh = Mesh::new_polygon(
            ctx,
            DrawMode::stroke(1.0),
            &player_points,
            Color::WHITE,
        )?;
        let draw_param = graphics::DrawParam::default()
            .dest([self.player_pos.0, self.player_pos.1])
            .rotation(self.player_angle + std::f32::consts::FRAC_PI_2) // Rotate 90 degrees to the right
            .offset([0.5, 0.5]);
        graphics::draw(ctx, &player_mesh, draw_param)?;

        for bullet in &self.bullets {
            let bullet_mesh = Mesh::new_circle(
                ctx,
                DrawMode::fill(),
                [bullet.pos.0, bullet.pos.1],
                BULLET_SIZE,
                0.1,
                Color::WHITE,
            )?;
            graphics::draw(ctx, &bullet_mesh, graphics::DrawParam::default())?;
        }

        for asteroid in &self.asteroids {
            let asteroid_mesh = Mesh::new_circle(
                ctx,
                DrawMode::stroke(1.0),
                [asteroid.pos.0, asteroid.pos.1],
                asteroid.size,
                0.1,
                Color::WHITE,
            )?;
            graphics::draw(ctx, &asteroid_mesh, graphics::DrawParam::default())?;
        }

        if self.is_game_over {
            let game_over_text = graphics::Text::new("Game Over");
            let text_width = game_over_text.width(ctx) as f32;
            let text_height = game_over_text.height(ctx) as f32;
            graphics::draw(
                ctx,
                &game_over_text,
                (ggez::mint::Point2 {
                    x: (WINDOW_WIDTH - text_width) / 2.0,
                    y: (WINDOW_HEIGHT - text_height) / 2.0,
                },),
            )?;
            let restart_text = graphics::Text::new("Press R to restart");            
            let text_width = restart_text.width(ctx) as f32;
            let text_height = restart_text.height(ctx) as f32;
            graphics::draw(
                ctx,
                &restart_text,
                (ggez::mint::Point2 {
                    x: (WINDOW_WIDTH - text_width) / 2.0,
                    y: (WINDOW_HEIGHT - text_height) / 2.0 + 20.0, // Offset to position below "Game Over"
                },),
            )?;
        } else {
            // Draw destroyed asteroid count
            let destroyed_text = graphics::Text::new(format!("Destroyed: {}", destroyed_count));
            let text_width = destroyed_text.width(ctx) as f32;
            let text_height = destroyed_text.height(ctx) as f32;
            graphics::draw(
                ctx,
                &destroyed_text,
                (ggez::mint::Point2 {
                    x: 10.0, // Adjust as needed
                    y: 10.0, // Adjust as needed
                },),
            )?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

impl EventHandler for AsteroidsGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if self.is_game_over {
            return Ok(());
        }

        let dt = self.last_update.elapsed().as_secs_f32();
        self.last_update = Instant::now();

        self.update_player(dt);
        self.update_bullets(dt);
        self.update_asteroids(dt);
        self.check_collisions();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);

        // Draw player ship as a triangle
        let player_points = [
            [0.0, -PLAYER_SIZE],
            [-PLAYER_SIZE / 2.0, PLAYER_SIZE],
            [PLAYER_SIZE / 2.0, PLAYER_SIZE],
        ];
        let player_mesh = Mesh::new_polygon(
            ctx,
            DrawMode::stroke(1.0),
            &player_points,
            Color::WHITE,
        )?;
        let draw_param = graphics::DrawParam::default()
            .dest([self.player_pos.0, self.player_pos.1])
            .rotation(self.player_angle + std::f32::consts::FRAC_PI_2) // Rotate 90 degrees to the right
            .offset([0.5, 0.5]);
        graphics::draw(ctx, &player_mesh, draw_param)?;

        for bullet in &self.bullets {
            let bullet_mesh = Mesh::new_circle(
                ctx,
                DrawMode::fill(),
                [bullet.pos.0, bullet.pos.1],
                BULLET_SIZE,
                0.1,
                Color::WHITE,
            )?;
            graphics::draw(ctx, &bullet_mesh, graphics::DrawParam::default())?;
        }

        for asteroid in &self.asteroids {
            let asteroid_mesh = Mesh::new_circle(
                ctx,
                DrawMode::stroke(1.0),
                [asteroid.pos.0, asteroid.pos.1],
                asteroid.size,
                0.1,
                Color::WHITE,
            )?;
            graphics::draw(ctx, &asteroid_mesh, graphics::DrawParam::default())?;
        }

        if self.is_game_over {
            let game_over_text = graphics::Text::new("Game Over");
            let text_width = game_over_text.width(ctx) as f32;
            let text_height = game_over_text.height(ctx) as f32;
            graphics::draw(
                ctx,
                &game_over_text,
                (ggez::mint::Point2 {
                    x: (WINDOW_WIDTH - text_width) / 2.0,
                    y: (WINDOW_HEIGHT - text_height) / 2.0,
                },),
            )?;
            let restart_text = graphics::Text::new("Press R to restart");            
            let text_width = restart_text.width(ctx) as f32;
            let text_height = restart_text.height(ctx) as f32;
            graphics::draw(
            ctx,
            &restart_text,
            (ggez::mint::Point2 {
                x: (WINDOW_WIDTH - text_width) / 2.0,
                y: (WINDOW_HEIGHT - text_height) / 2.0 + 20.0, // Offset to position below "Game Over"
            },),
        )?;
        }

        // Draw score
        self.draw_score(ctx)?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Up => {
                let (dir_x, dir_y) = (self.player_angle.cos(), self.player_angle.sin());
                self.player_vel.0 += dir_x * 100.0;
                self.player_vel.1 += dir_y * 100.0;
            }
            KeyCode::Left => {
                self.player_angle -= 0.1;
            }
            KeyCode::Right => {
                self.player_angle += 0.1;
            }
            KeyCode::Space => {
                self.shoot();
            }
            KeyCode::R => {
                self.restart_game();
            }
            _ => (),
        }
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("Bubbleroids", "Oscar Abad")
        .window_setup(ggez::conf::WindowSetup::default().title("Bubbleroids - Rust Retro Game by Oscar Abad"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build()?;

    let game = AsteroidsGame::new(&mut ctx)?;
    
    event::run(ctx, event_loop, game)
}
