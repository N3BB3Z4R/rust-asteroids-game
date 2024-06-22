use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::graphics::{self, Color, DrawMode, Mesh, Image, Rect, Text, Font};
use ggez::{Context, ContextBuilder, GameResult};
use rand::Rng;
use std::time::Instant;

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;
const ASTEROID_SIZE: f32 = 40.0;
const PLAYER_SIZE: f32 = 14.0;
const BULLET_SIZE: f32 = 3.0;
const BULLET_SPEED: f32 = 400.0;
const ASTEROID_COUNT: usize = 20;
const PARTICLE_LIFETIME: f32 = 2.0;
const PARTICLE_SPEED: f32 = 80.0;
const PARTICLE_SIZE: f32 = 2.0;
const LEVEL_UP_THRESHOLD: f32 = 10.0;
const G: f32 = 6.67430e-11;
const SCALE_FACTOR: f32 = 1e9;
const SPRING_CONSTANT: f32 = 0.9;
const DAMPING: f32 = 0.5;
const MAX_DEFORMATION: f32 = 0.8;
const MAX_THRUSTER_PARTICLES: usize = 40;
const THRUSTER_PARTICLE_LIFETIME: f32 = 0.7;
const THRUSTER_PARTICLE_SIZE: f32 = 2.0;

fn calculate_gravity(mass1: f32, mass2: f32, distance: f32) -> f32 {
    G * mass1 * mass2 / (distance * distance) * SCALE_FACTOR
}

struct Bullet {
    pos: (f32, f32),
    vel: (f32, f32),
}

struct ThrusterParticle {
    pos: (f32, f32),
    vel: (f32, f32),
    color: Color,
    life: f32,
    initial_life: f32,
}

struct Asteroid {
    pos: (f32, f32),
    vel: (f32, f32),
    size: f32,
    is_destroyed: bool,
    mass: f32,
    deformation: f32,
    deformation_vel: f32,
}

struct Particle {
    pos: (f32, f32),
    vel: (f32, f32),
    life: f32,
    color: Color,
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
    level: u32,
    score: u32,
    thruster_particles: Vec<ThrusterParticle>,
}

impl ThrusterParticle {
    fn new(pos: (f32, f32), vel: (f32, f32), color: Color, lifetime: f32) -> Self {
        ThrusterParticle {
            pos,
            vel,
            color,
            life: lifetime,
            initial_life: lifetime,
        }
    }

    fn update(&mut self, dt: f32) {
        self.pos.0 += self.vel.0 * dt;
        self.pos.1 += self.vel.1 * dt;
        self.life -= dt;
    }
}

impl Asteroid {
    fn new(pos: (f32, f32), vel: (f32, f32), size: f32) -> Self {
        Asteroid {
            pos,
            vel,
            size,
            is_destroyed: false,
            mass: size * size * std::f32::consts::PI * 0.1,
            deformation: 0.0,
            deformation_vel: 0.0,
        }
    }

    fn update_deformation(&mut self, dt: f32) {
        // Simulación de resorte
        let spring_force = -SPRING_CONSTANT * self.deformation;
        let damping_force = -DAMPING * self.deformation_vel;
        let total_force = spring_force + damping_force;

        self.deformation_vel += total_force * dt;
        self.deformation += self.deformation_vel * dt;

        // Limitar la deformación máxima
        self.deformation = self.deformation.clamp(-self.size * MAX_DEFORMATION, self.size * MAX_DEFORMATION);
    }
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
                mass: size * size * std::f32::consts::PI * 0.1,
                deformation: 4.0,
                deformation_vel: 1.0,
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
            level: 1,
            score: 0,
            thruster_particles: Vec::new(),
        })
    }

    fn update_player(&mut self, dt: f32) {
        let mut total_gravity = (0.0, 0.0);
        
        for asteroid in &self.asteroids {
            let dx = asteroid.pos.0 - self.player_pos.0;
            let dy = asteroid.pos.1 - self.player_pos.1;
            let distance = (dx * dx + dy * dy).sqrt();
            let force = calculate_gravity(PLAYER_SIZE * PLAYER_SIZE * std::f32::consts::PI, asteroid.mass, distance);
            let angle = dy.atan2(dx);
            total_gravity.0 += force * angle.cos();
            total_gravity.1 += force * angle.sin();
        }
        
        self.player_vel.0 += total_gravity.0 * dt * 10.0;
        self.player_vel.1 += total_gravity.1 * dt * 10.0;
        
        self.player_pos.0 += self.player_vel.0 * dt;
        self.player_pos.1 += self.player_vel.1 * dt;

        // Wrap around the screen
        self.player_pos.0 = (self.player_pos.0 + WINDOW_WIDTH) % WINDOW_WIDTH;
        self.player_pos.1 = (self.player_pos.1 + WINDOW_HEIGHT) % WINDOW_HEIGHT;

        // Apply friction to slow down the player
        self.player_vel.0 *= 0.99;
        self.player_vel.1 *= 0.99;

        // Generar nuevas partículas
        self.generate_thruster_particles();

        // Actualizar partículas existentes
        self.update_thruster_particles(dt);

        // Generar partículas del propulsor
        let speed = (self.player_vel.0.powi(2) + self.player_vel.1.powi(2)).sqrt();
        let max_speed = 200.0; // Velocidad máxima de la nave
        let normalized_speed = speed / max_speed;
        let inverse_speed_factor = 1.0 - normalized_speed;
        let num_particles = (inverse_speed_factor * 10.0).max(1.0) as usize; // Ajusta estos valores según necesites
        
        let mut rng = rand::thread_rng();
        for _ in 0..num_particles {
            let angle = self.player_angle + std::f32::consts::PI + rng.gen_range(-0.2..0.2);
            let particle_speed = rng.gen_range(20.0..50.0) * (1.0 + inverse_speed_factor);
            let vel = (angle.cos() * particle_speed, angle.sin() * particle_speed);
            
            let pos = (
                self.player_pos.0 - self.player_angle.cos() * PLAYER_SIZE,
                self.player_pos.1 - self.player_angle.sin() * PLAYER_SIZE,
            );

            let lifetime = rng.gen_range(0.3..THRUSTER_PARTICLE_LIFETIME) * (1.0 + inverse_speed_factor);
            let color = self.generate_flame_color(1.0);
            
            self.thruster_particles.push(ThrusterParticle::new(pos, vel, color, lifetime));
        }

        // Limitar el número máximo de partículas
        while self.thruster_particles.len() > MAX_THRUSTER_PARTICLES {
            self.thruster_particles.remove(0);
        }
    }

    fn generate_thruster_particles(&mut self) {
        let speed = (self.player_vel.0.powi(2) + self.player_vel.1.powi(2)).sqrt();
        let num_particles = (speed / 10.0).min(5.0) as usize;
        
        let mut rng = rand::thread_rng();
        for _ in 0..num_particles {
            let angle = self.player_angle + std::f32::consts::PI + rng.gen_range(-0.2..0.2);
            let speed = rng.gen_range(50.0..100.0);
            let vel = (angle.cos() * speed, angle.sin() * speed);
            
            let pos = (
                self.player_pos.0 - self.player_angle.cos() * PLAYER_SIZE,
                self.player_pos.1 - self.player_angle.sin() * PLAYER_SIZE,
            );

            let lifetime = rng.gen_range(0.3..THRUSTER_PARTICLE_LIFETIME);
            let color = self.generate_flame_color(1.0); // Iniciar con el color azul
            
            self.thruster_particles.push(ThrusterParticle::new(pos, vel, color, lifetime));
        }
    }

    fn update_thruster_particles(&mut self, dt: f32) {
        let mut i = 0;
        while i < self.thruster_particles.len() {
            let remove_particle = {
                let particle = &mut self.thruster_particles[i];
                particle.update(dt);
                
                // Calcular el color aquí sin llamar a self.generate_flame_color
                let t = 1.0 - (particle.life / particle.initial_life);
                particle.color = if t < 0.2 {
                    Color::from_rgba(0, 0, 255, 255) // Azul
                } else if t < 0.4 {
                    Color::from_rgba(255, 255, 255, 255) // Blanco
                } else if t < 0.6 {
                    Color::from_rgba(255, 255, 0, 255) // Amarillo
                } else if t < 0.8 {
                    Color::from_rgba(255, 165, 0, 255) // Naranja
                } else {
                    Color::from_rgba(255, 0, 0, 255) // Rojo
                };
                
                particle.life <= 0.0
            };
            
            if remove_particle {
                self.thruster_particles.swap_remove(i);
            } else {
                i += 1;
            }
        }
    }

    fn generate_flame_color(&self, t: f32) -> Color {
        let t = 1.0 - t; // Invertir t para que el azul esté al principio
        if t < 0.2 {
            Color::from_rgba(0, 0, 255, 255) // Azul
        } else if t < 0.4 {
            Color::from_rgba(255, 255, 255, 255) // Blanco
        } else if t < 0.6 {
            Color::from_rgba(255, 255, 0, 255) // Amarillo
        } else if t < 0.8 {
            Color::from_rgba(255, 165, 0, 255) // Naranja
        } else {
            Color::from_rgba(255, 0, 0, 255) // Rojo
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
        let asteroid_count = self.asteroids.len();
        let mut gravity_forces = vec![(0.0, 0.0); asteroid_count];

        for i in 0..asteroid_count {
            for j in (i + 1)..asteroid_count {
                let dx = self.asteroids[j].pos.0 - self.asteroids[i].pos.0;
                let dy = self.asteroids[j].pos.1 - self.asteroids[i].pos.1;
                let distance = (dx * dx + dy * dy).sqrt().max(1.0);
                let force = calculate_gravity(self.asteroids[i].mass, self.asteroids[j].mass, distance);
                let angle = dy.atan2(dx);
                
                let force_x = force * angle.cos();
                let force_y = force * angle.sin();
                
                gravity_forces[i].0 += force_x;
                gravity_forces[i].1 += force_y;
                gravity_forces[j].0 -= force_x;
                gravity_forces[j].1 -= force_y;
            }
        }

        for (asteroid, force) in self.asteroids.iter_mut().zip(gravity_forces.iter()) {
            // Limitar la aceleración máxima
            let max_acceleration = 50.0;
            let acceleration_x = force.0.clamp(-max_acceleration, max_acceleration);
            let acceleration_y = force.1.clamp(-max_acceleration, max_acceleration);
            
            asteroid.vel.0 += acceleration_x * dt;
            asteroid.vel.1 += acceleration_y * dt;
            
            // Limitar la velocidad máxima
            let max_speed = 200.0;
            let speed = (asteroid.vel.0.powi(2) + asteroid.vel.1.powi(2)).sqrt();
            if speed > max_speed {
                asteroid.vel.0 = asteroid.vel.0 / speed * max_speed;
                asteroid.vel.1 = asteroid.vel.1 / speed * max_speed;
            }
            
            asteroid.pos.0 += asteroid.vel.0 * dt;
            asteroid.pos.1 += asteroid.vel.1 * dt;

            // Wrap around the screen
            asteroid.pos.0 = (asteroid.pos.0 + WINDOW_WIDTH) % WINDOW_WIDTH;
            asteroid.pos.1 = (asteroid.pos.1 + WINDOW_HEIGHT) % WINDOW_HEIGHT;
        }

        for asteroid in &mut self.asteroids {
            asteroid.update_deformation(dt);
        }

        self.handle_asteroid_collisions();
    }

    fn handle_asteroid_collisions(&mut self) {
        let mut collisions = Vec::new();

        // Detectar colisiones
        for i in 0..self.asteroids.len() {
            for j in (i + 1)..self.asteroids.len() {
                let asteroid1 = &self.asteroids[i];
                let asteroid2 = &self.asteroids[j];

                let dx = asteroid1.pos.0 - asteroid2.pos.0;
                let dy = asteroid1.pos.1 - asteroid2.pos.1;
                let distance = (dx * dx + dy * dy).sqrt();

                if distance < asteroid1.size + asteroid2.size {
                    collisions.push((i, j));
                }
            }
        }

        // Resolver colisiones
        for (i, j) in collisions {
            let (asteroid1, asteroid2) = self.asteroids.split_at_mut(j);
            let asteroid1 = &mut asteroid1[i];
            let asteroid2 = &mut asteroid2[0];

            // Calcular la normal de colisión
            let nx = asteroid2.pos.0 - asteroid1.pos.0;
            let ny = asteroid2.pos.1 - asteroid1.pos.1;
            let d = (nx * nx + ny * ny).sqrt();
            let nx = nx / d;
            let ny = ny / d;

            // Calcular la velocidad relativa
            let dvx = asteroid2.vel.0 - asteroid1.vel.0;
            let dvy = asteroid2.vel.1 - asteroid1.vel.1;

            // Calcular el impulso
            let impulse = 2.0 * (dvx * nx + dvy * ny) / (asteroid1.mass + asteroid2.mass);

            // Aplicar el impulso
            asteroid1.vel.0 += impulse * asteroid2.mass * nx;
            asteroid1.vel.1 += impulse * asteroid2.mass * ny;
            asteroid2.vel.0 -= impulse * asteroid1.mass * nx;
            asteroid2.vel.1 -= impulse * asteroid1.mass * ny;

            // Separar los asteroides para evitar superposición
            let overlap = asteroid1.size + asteroid2.size - d;
            let separation = overlap / 2.0;
            asteroid1.pos.0 -= separation * nx;
            asteroid1.pos.1 -= separation * ny;
            asteroid2.pos.0 += separation * nx;
            asteroid2.pos.1 += separation * ny;

            // Aplicar deformación
            let collision_force = (asteroid1.mass * asteroid2.mass).sqrt() * 0.01;
            asteroid1.deformation_vel += collision_force / asteroid1.mass;
            asteroid2.deformation_vel += collision_force / asteroid2.mass;
        }
    }

    fn check_collisions(&mut self) {
        let mut should_level_up = false;

        // Check for collisions between player and asteroids
        for asteroid in &self.asteroids {
            let dist = ((self.player_pos.0 - asteroid.pos.0).powi(2)
                + (self.player_pos.1 - asteroid.pos.1).powi(2))
                .sqrt();
            if dist < asteroid.size + PLAYER_SIZE {
                self.is_game_over = true;
                self.generate_explosion(self.player_pos, 50, Color::RED);
                return;
            }
        }

        // Check for collisions between bullets and asteroids
        let mut new_asteroids = Vec::new();
        let mut bullets_to_remove = Vec::new();
        let mut explosions_to_generate = Vec::new();

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
            
                    let points = (100.0 / asteroid.size) as u32 * self.level;
                    self.score += points;
                    
                    explosions_to_generate.push((asteroid.pos, 20, Color::GREEN));

                    // Dividir el asteroide en piezas más pequeñas
                    if asteroid.size > 20.0 {
                        let new_size = asteroid.size / 2.0;
                        let new_mass = new_size * new_size * std::f32::consts::PI * 0.1;
                        let new_vel = (-asteroid.vel.0, -asteroid.vel.1);
                        new_asteroids.push(Asteroid {
                            pos: asteroid.pos,
                            vel: new_vel,
                            size: new_size,
                            is_destroyed: false,
                            mass: new_mass,
                            deformation: 8.0,
                            deformation_vel: 1.0,
                        });
                        new_asteroids.push(Asteroid {
                            pos: asteroid.pos,
                            vel: asteroid.vel,
                            size: new_size,
                            is_destroyed: false,
                            mass: new_mass,
                            deformation: 8.0,
                            deformation_vel: 1.0,
                        });
                    }

                    if self.destroyed_count >= LEVEL_UP_THRESHOLD * self.level as f32 {
                        should_level_up = true;
                    }
                }
            }
        }

        // Remove hit bullets
        bullets_to_remove.sort_unstable_by(|a, b| b.cmp(a));
        for idx in bullets_to_remove {
            self.bullets.remove(idx);
        }

        self.asteroids.extend(new_asteroids);
        self.asteroids.retain(|asteroid| !asteroid.is_destroyed);

        // Generate explosions
        for (pos, num_particles, color) in explosions_to_generate {
            self.generate_explosion(pos, num_particles, color);
        }

        if should_level_up {
            self.level_up();
        }

    }

    fn level_up(&mut self) {
        self.level += 1;
        let mut rng = rand::thread_rng();
        // Aumentar la dificultad
        for _ in 0..self.level {
            // let mut rng = rand::thread_rng();
            let x = rng.gen_range(0.0..WINDOW_WIDTH);
            let y = rng.gen_range(0.0..WINDOW_HEIGHT);
            let vel_x = rng.gen_range(-50.0..50.0) * (1.0 + self.level as f32 * 0.1);
            let vel_y = rng.gen_range(-50.0..50.0) * (1.0 + self.level as f32 * 0.1);
            let size = rng.gen_range(15.0..ASTEROID_SIZE);
            self.asteroids.push(Asteroid {
                pos: (x, y),
                vel: (vel_x, vel_y),
                size,
                is_destroyed: false,
                mass: size * size * std::f32::consts::PI * 0.1,
                deformation: 5.0,
                deformation_vel: 6.0,
            });
        }
    }

    fn draw_hud(&self, ctx: &mut Context) -> GameResult<()> {
        let font = Font::default();
        let score_text = Text::new((format!("Puntuación: {}", self.score), font, 26.0));
        let level_text = Text::new((format!("Nivel: {}", self.level), font, 26.0));
        
        graphics::draw(ctx, &score_text, (ggez::mint::Point2 { x: 10.0, y: 10.0 }, 0.0, Color::GREEN))?;
        graphics::draw(ctx, &level_text, (ggez::mint::Point2 { x: 10.0, y: 40.0 }, 0.0, Color::GREEN))?;
        
        Ok(())
    }


    fn generate_explosion(&mut self, pos: (f32, f32), num_particles: usize, color: Color) {
        let mut rng = rand::thread_rng();
        for _ in 0..num_particles {
            let angle = rng.gen_range(0.0..std::f32::consts::TAU);
            let speed = rng.gen_range(0.0..PARTICLE_SPEED);
            self.particles.push(Particle {
                pos,
                vel: (speed * angle.cos(), speed * angle.sin()),
                life: PARTICLE_LIFETIME,
                color,
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
                PARTICLE_SIZE,
                0.1,
                particle.color,
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

    fn draw_score(&self, ctx: &mut Context) -> GameResult<()> {
        let font = Font::default();
        let score_text = Text::new((format!("Hits: {}", self.destroyed_count), font, 26.0));
        let (width, _) = graphics::drawable_size(ctx);
        let dest_point = ggez::mint::Point2 { x: width - 200.0, y: 20.0 };
        graphics::draw(ctx, &score_text, (dest_point, 0.0, Color::GREEN))?;
        Ok(())
    }

    fn restart_game(&mut self) {
        self.player_pos = (WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0);
        self.player_vel = (0.0, 0.0);
        self.player_angle = 0.0;
        self.bullets.clear();
        self.asteroids.clear();
        self.particles.clear();
        self.is_game_over = false;
        self.destroyed_count = 0.0;
        self.level = 1;
        self.score = 0;

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
                mass: size * size * std::f32::consts::PI * 0.1,
                deformation: 2.0,
                deformation_vel: 3.0,
            });
        }
    }

    fn create_deformed_asteroid_mesh(&self, ctx: &mut Context, asteroid: &Asteroid) -> GameResult<Mesh> {
        let num_points = 32;
        let mut points = Vec::with_capacity(num_points);

        for i in 0..num_points {
            let angle = i as f32 * 2.0 * std::f32::consts::PI / num_points as f32;
            let (sin, cos) = angle.sin_cos();
            let radius = asteroid.size + asteroid.deformation * cos; // Deformación ovalada
            let x = asteroid.pos.0 + radius * cos;
            let y = asteroid.pos.1 + radius * sin;
            points.push([x, y]);
        }

        Mesh::new_polygon(
            ctx,
            DrawMode::stroke(1.0),
            &points,
            Color::GREEN,
        )
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
        self.update_particles(dt);
        self.check_collisions();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::from_rgb(1, 4, 0));

        // Dibujar partículas del propulsor
        for particle in &self.thruster_particles {
            let particle_mesh = Mesh::new_circle(
                ctx,
                DrawMode::fill(),
                [particle.pos.0, particle.pos.1],
                PARTICLE_SIZE,
                0.1,
                particle.color,
            )?;
            graphics::draw(ctx, &particle_mesh, graphics::DrawParam::default())?;
        }

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
            // Make an orange
            Color::from_rgb(200, 140, 0)
        )?;
        let draw_param = graphics::DrawParam::default()
            .dest([self.player_pos.0, self.player_pos.1])
            .rotation(self.player_angle + std::f32::consts::FRAC_PI_2)
            .offset([0.5, 0.5]);
        graphics::draw(ctx, &player_mesh, draw_param)?;

        for bullet in &self.bullets {
            let bullet_mesh = Mesh::new_circle(
                ctx,
                DrawMode::fill(),
                [bullet.pos.0, bullet.pos.1],
                BULLET_SIZE,
                0.1,
                Color::YELLOW,
            )?;
            graphics::draw(ctx, &bullet_mesh, graphics::DrawParam::default())?;
        }

        for asteroid in &self.asteroids {
            // let asteroid_mesh = Mesh::new_circle(
            //     ctx,
            //     DrawMode::stroke(1.0),
            //     [asteroid.pos.0, asteroid.pos.1],
            //     asteroid.size,
            //     0.1,
            //     Color::GREEN,
            // )?;
            let deformed_size = asteroid.size + asteroid.deformation;
            let asteroid_mesh = self.create_deformed_asteroid_mesh(ctx, asteroid)?;
            graphics::draw(ctx, &asteroid_mesh, graphics::DrawParam::default())?;
        }

        self.draw_particles(ctx)?;

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
                    y: (WINDOW_HEIGHT - text_height) / 2.0 + 20.0,
                },),
            )?;
        }

        self.draw_score(ctx)?;
        self.draw_hud(ctx)?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Up => {
                let (dir_x, dir_y) = (self.player_angle.cos(), self.player_angle.sin());
                self.player_vel.0 += dir_x * 10.0;
                self.player_vel.1 += dir_y * 10.0;
                // Limitar la velocidad máxima
                let speed = (self.player_vel.0.powi(2) + self.player_vel.1.powi(2)).sqrt();
                if speed > 200.0 {
                    self.player_vel.0 = self.player_vel.0 / speed * 200.0;
                    self.player_vel.1 = self.player_vel.1 / speed * 200.0;
                }
            }
            KeyCode::Left => {
                self.player_angle -= 0.2;
            }
            KeyCode::Right => {
                self.player_angle += 0.2;
            }
            KeyCode::Space => {
                self.shoot();
            }
            KeyCode::R if self.is_game_over => {
                self.restart_game();
            }
            _ => (),
        }
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("Bubbleroid", "Oscar Abad")
        .window_setup(ggez::conf::WindowSetup::default().title("Bubbleroid - Rust Retro Game by Oscar Abad"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build()?;

    let game = AsteroidsGame::new(&mut ctx)?;
    
    event::run(ctx, event_loop, game)
}