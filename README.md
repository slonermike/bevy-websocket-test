# Bevy WebSocket Bouncing Balls

A learning project exploring Rust, Bevy game engine, and WebSocket communication. The goal is a simple physics simulation where balls bounce around the screen, with spawning controlled via WebSocket messages from a React frontend.

## Architecture

```
┌─────────────────┐         WebSocket          ┌─────────────────────────┐
│   React Client  │ ──────────────────────────►│      Bevy Server        │
│   (TypeScript)  │    {"x": 100, "y": 200}    │                         │
│                 │                             │  ┌─────────────────┐   │
│  Click anywhere │                             │  │  Axum WebSocket │   │
│  to spawn ball  │                             │  │     Server      │   │
└─────────────────┘                             │  └────────┬────────┘   │
                                                │           │ channel    │
                                                │           ▼            │
                                                │  ┌─────────────────┐   │
                                                │  │   Ball Plugin   │   │
                                                │  │  (spawn, physics│   │
                                                │  │   rendering)    │   │
                                                │  └─────────────────┘   │
                                                │                         │
                                                │  ┌─────────────────┐   │
                                                │  │  Avian Physics  │   │
                                                │  │  (collisions,   │   │
                                                │  │   bouncing)     │   │
                                                │  └─────────────────┘   │
                                                └─────────────────────────┘
```

## Project Structure

```
bevy-websocket-test/
├── server/                 # Rust/Bevy application
│   ├── src/
│   │   ├── main.rs         # App setup, camera, walls, thread spawning
│   │   ├── protocol.rs     # Shared message types (SpawnRequest, SpawnBallMessage)
│   │   ├── websocket.rs    # Axum WebSocket server, channel bridge
│   │   └── plugins/
│   │       └── ball.rs     # Ball spawning, physics components
│   └── Cargo.toml
├── client/                 # React app (TODO)
└── README.md
```

## Technology Choices

### Bevy (0.18)
A data-driven game engine using an Entity-Component-System (ECS) architecture. Chosen for its Rust-native design and active community.

### Avian Physics (0.6)
A 2D/3D physics engine built specifically for Bevy's ECS. Chosen over `bevy_rapier` for its more idiomatic Bevy integration.

### Axum
A web framework by the Tokio team. Handles both WebSocket connections and can serve static files (for the React app in production).

## Running the Server

```bash
cd server
cargo run
```

The WebSocket server listens on `ws://localhost:3000/ws`.

## Current Features

- Window with 2D camera
- Balls rendered as red circles
- Physics: gravity, velocity, bouncing (restitution)
- Boundary walls at screen edges
- Timer-based automatic ball spawning (for testing)
- WebSocket server receiving spawn requests

## Message Protocol

The React client sends JSON over WebSocket:

```json
{"x": 150.0, "y": -50.0}
```

The server spawns a ball at that position with randomized velocity.

## Key Rust/Bevy Concepts Learned

### Ownership & Borrowing
- `String` vs `&str` - owned data vs borrowed reference
- `move` keyword for closures that take ownership

### Bevy ECS
- **Entities** - IDs (a ball is an entity)
- **Components** - Data attached to entities (`Transform`, `LinearVelocity`)
- **Systems** - Functions that operate on entities with specific components
- **Resources** - Global singleton data (`Time`, `Assets<T>`, custom resources)
- **Messages** - System-to-system communication (formerly "events" in older Bevy)

### Plugins
Group related functionality (systems, resources, messages) into reusable units.

### Concurrency
- Bevy runs on the main thread (game loop)
- Axum runs on a spawned thread with its own Tokio runtime
- `std::sync::mpsc` channel bridges async WebSocket → sync Bevy

## Testing the WebSocket

Before building the React client, you can test WebSocket spawning manually:

### Using websocat
```bash
websocat ws://localhost:3000/ws
# Then type:
{"x": 0, "y": 100}
```

### Using browser console
```javascript
const ws = new WebSocket('ws://localhost:3000/ws');
ws.onopen = () => ws.send('{"x": 0, "y": 100}');
```

## Next Steps

1. Create React client with Vite/TypeScript/Yarn
2. Click-to-spawn functionality in the client
3. Ball-to-ball collisions
4. (Future) Serve React app from Axum in production

## Development Notes

### Compile Times
Bevy has slow initial compile times. Enable dynamic linking for faster iteration:

```toml
[dependencies]
bevy = { version = "0.18", features = ["dynamic_linking"] }
```

### Linux Dependencies
On Ubuntu/Debian:
```bash
sudo apt install libwayland-dev libxkbcommon-dev libasound2-dev libudev-dev
```
