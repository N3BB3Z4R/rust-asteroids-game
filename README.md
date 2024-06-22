# Asteroids Game in Rust

This project is an implementation of the classic Asteroids game using Rust and the GGEZ game engine. It features a unique gravitational system that adds an extra layer of complexity and challenge to the traditional gameplay.

## Design Pattern

The game follows the Entity-Component-System (ECS) architectural pattern, which is commonly used in game development. While not a full ECS implementation, the game's structure is inspired by this pattern:

- Entities: Player ship, asteroids, bullets, and particles.
- Components: Position, velocity, size, mass, and destruction status.
- Systems: Update, collision detection, rendering, and input handling.

The main game loop is handled by the GGEZ event system, which calls the appropriate update and draw methods.

## Libraries Used

1. **ggez** (0.x): A lightweight game framework for making 2D games with minimum effort.
   - Handles window creation, event loop, and rendering.
   - Provides utilities for drawing shapes, text, and handling input.

2. **rand** (0.x): A Rust library for random number generation.
   - Used for generating random positions, sizes, and velocities for asteroids.

## Features

1. **Gravitational Physics**: 
   - Implements a simplified gravitational system where asteroids and the player ship are affected by each other's gravity.
   - Gravity strength is proportional to the mass (size) of the asteroids.

2. **Dynamic Asteroid Generation**:
   - Asteroids are randomly generated with varying sizes and velocities.
   - When destroyed, larger asteroids split into smaller ones.

3. **Level Progression**:
   - Difficulty increases with each level, introducing more asteroids.

4. **Particle Effects**:
   - Explosions are visualized using a particle system when asteroids are destroyed.

5. **Scoring System**:
   - Players earn points for destroying asteroids, with smaller asteroids worth more points.

6. **Wrap-around World**:
   - Objects that move off one edge of the screen appear on the opposite side.

7. **Player Controls**:
   - Thrust: Up Arrow
   - Rotate: Left/Right Arrows
   - Shoot: Spacebar

8. **Game Over and Restart**:
   - The game ends when the player collides with an asteroid.
   - Players can restart the game after a game over.

9. **HUD (Heads-Up Display)**:
   - Displays current score and level.

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Installation

1. Clone the repository:
   ```
   git clone https://github.com/N3BB3Z4R/rust-asteroids-game.git
   ```

2. Navigate to the project directory:
   ```
   cd asteroids-rust-game
   ```

3. Build and run the game:
   ```
   cargo run --release
   ```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## Acknowledgments

- Original Asteroids game by Atari
- GGEZ community for the excellent game framework
- Rust community for the amazing programming language and ecosystem