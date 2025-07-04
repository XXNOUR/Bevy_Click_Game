# SpongeBob Particle Clicker (Bevy Game Dev Practice)

A simple 2D game built with **Rust** + **Bevy Engine** where SpongeBob sprites float around the screen, and your goal is to click them for points.  
Inspired by old-school arcade reaction games, but mainly made to practice ECS, sprite animation, and basic game loops in Bevy.

---

## 🚀 Features
- 🖱️ Click to destroy SpongeBob sprites and increase your score.
- 🎲 Random spawn locations + movement speeds for each sprite.
- 🔄 Smooth elliptical motion using sine/cosine animation.
- 🔧 Score display with custom pixel font.
- 🌌 Clean 2D Camera setup.
- 🔨 Built using modern Bevy ECS architecture.

---

## 🛠️ Tech Stack
- [Rust](https://www.rust-lang.org/)
- [Bevy Engine](https://bevyengine.org/) (v0.13 or later)
- [Rand crate](https://crates.io/crates/rand) for random coordinates & speed.

---

## 🎮 Controls
| Action         | Input                  |
|---------------|------------------------|
| Destroy sprite | Left Mouse Button Click |
| Quit           | Close the window (no pause/quit key yet) |

---

## 📁 Assets
- `fonts/PixelatedEleganceRegular-ovyAA.ttf` — pixel font for score display.
- `spong_Ass.png` — (replace this with a better sprite if needed).

---

## 🔧 Setup & Run
```bash
git clone https://github.com/XXNOUR/Bevy_Click_Game.git
cd Bevy_Click_Game
cargo run

