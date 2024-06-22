# Bubbleroid: Asteroids Game in Rust

This project is an implementation of the classic Asteroids game using Rust and the GGEZ game engine. It features a unique gravitational system and deformable asteroids that add extra layers of complexity and visual appeal to the traditional gameplay.

## Design Pattern

The game follows a structure inspired by the Entity-Component-System (ECS) architectural pattern, commonly used in game development:

- Entities: Player ship, asteroids, bullets, and particles (including thruster particles).
- Components: Position, velocity, size, mass, deformation, and destruction status.
- Systems: Update, collision detection, rendering, particle generation, and input handling.

The main game loop is handled by the GGEZ event system, which calls the appropriate update and draw methods.

## Libraries Used

1. **ggez** (0.x): A lightweight game framework for making 2D games with minimum effort.
   - Handles window creation, event loop, and rendering.
   - Provides utilities for drawing shapes, text, and handling input.
2. **rand** (0.x): A Rust library for random number generation.
   - Used for generating random positions, sizes, and velocities for asteroids and particles.

## Features

1. **Gravitational Physics**:
   - Implements a simplified gravitational system where asteroids and the player ship are affected by each other's gravity.
   - Gravity strength is proportional to the mass (size) of the asteroids.

2. **Deformable Asteroids**:
   - Asteroids can deform upon collision, creating more realistic and visually interesting interactions.
   - Deformation is simulated using a spring-mass system.

3. **Dynamic Asteroid Generation**:
   - Asteroids are randomly generated with varying sizes and velocities.
   - When destroyed, larger asteroids split into smaller ones.

4. **Level Progression**:
   - Difficulty increases with each level, introducing more asteroids.

5. **Particle Effects**:
   - Explosions are visualized using a particle system when asteroids are destroyed.
   - Thruster particles are generated behind the player's ship, with intensity inversely proportional to ship speed.

6. **Scoring System**:
   - Players earn points for destroying asteroids, with smaller asteroids worth more points.

7. **Wrap-around World**:
   - Objects that move off one edge of the screen appear on the opposite side.

8. **Player Controls**:
   - Thrust: Up Arrow
   - Rotate: Left/Right Arrows
   - Shoot: Spacebar

9. **Game Over and Restart**:
   - The game ends when the player collides with an asteroid.
   - Players can restart the game after a game over by pressing 'R'.

10. **HUD (Heads-Up Display)**:
    - Displays current score, level, and other game information.

11. **Dynamic Thruster Effect**:
    - The player's ship generates more thruster particles when moving slowly or stationary, and fewer when moving quickly.

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
   cd rust-asteroids-game
   ```
3. Build and run the game:
   ```
   cargo run --release
   ```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.


## Acknowledgments

- Original Asteroids game by Atari
- GGEZ community for the excellent game framework
- Rust community for the amazing programming language and ecosystem


## Future Improvements for Bubbleroids

### 1. Physics and Movement
- **Player Inertia**: Implement gradual deceleration when the acceleration key is not pressed.
- **Speed Limit**: Add a maximum speed limit for the player.
- **Smooth Rotation**: Implement smoother player rotation.

### 2. Gameplay
- **Levels**: Implement a level system with increasing difficulty.
- **Power-ups**: Add power-ups such as temporary shields or multiple shots.
- **Lives**: Implement a life system for the player.
- **Scoring**: Improve the scoring system, differentiating between large and small asteroids.

### 3. Graphics and Visual Effects
- **Particle Enhancement**: Expand the particle system for more impressive explosions.
- **Animations**: Add animations for asteroid destruction and player ship.
- **Sound Effects**: Implement sound effects for shots, explosions, and background.

### 4. Optimization and Performance
- **Use of SpriteBatch**: Utilize SpriteBatch to render multiple similar objects more efficiently.
- **Optimized Collisions**: Implement a more efficient collision system, such as a quad-tree.

### 5. User Interface
- **Main Menu**: Add a main menu with options like "Play", "Settings", and "Exit".
- **Pause Screen**: Implement a pause screen during the game.
- **High Score Table**: Add a system to save and display the highest scores.

### 6. Code and Structure
- **State Management**: Implement a state machine to handle different game screens.
- **Configuration**: Move constants to a separate configuration file.
- **Unit Tests**: Add unit tests for the main game functions.

### 7. Additional Features
- **Multiplayer Mode**: Implement a local multiplayer mode.
- **Customization**: Allow customization of the player's ship.
- **Achievements**: Add an achievement system to increase replayability.