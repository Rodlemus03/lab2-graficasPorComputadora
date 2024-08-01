use minifb::{Window, WindowOptions, Key};
use std::time::Duration;
use std::thread::sleep;
mod framebuffer;

const WIDTH: usize = 80;
const HEIGHT: usize = 60;
const SCALE: usize = 10;
const WINDOW_WIDTH: usize = WIDTH * SCALE;
const WINDOW_HEIGHT: usize = HEIGHT * SCALE;

// Define los colores
const COLOR_GLIDER: u32 = 0xFF0000; // Rojo
const COLOR_PENTADECATHLON: u32 = 0x00FF00; // Verde
const COLOR_OSCILLATOR: u32 = 0x0000FF; // Azul
const COLOR_HWSHIP: u32 = 0xFFFF00; // Amarillo
const COLOR_BACKGROUND: u32 = 0x333355; // Azul oscuro

fn initialize_world() -> Vec<Vec<bool>> {
    let mut world = vec![vec![false; WIDTH]; HEIGHT];
    
    // Agregar figuras iniciales
    add_glider(&mut world, 1, 1);
    add_pentadecathlon(&mut world, 10, 10);
    add_oscillator(&mut world, 20, 20);
    add_heavyweight_spaceship(&mut world, 30, 30);
    add_glider(&mut world, 40, 40);
    add_pentadecathlon(&mut world, 50, 10);
    add_oscillator(&mut world, 60, 20);
    add_heavyweight_spaceship(&mut world, 10, 30);
    add_glider(&mut world, 20, 40);
    add_pentadecathlon(&mut world, 30, 50);

    world
}

fn add_glider(world: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    if x < WIDTH - 2 && y < HEIGHT - 2 {
        world[y][x + 1] = true;
        world[y + 1][x + 2] = true;
        world[y + 2][x] = true;
        world[y + 2][x + 1] = true;
        world[y + 2][x + 2] = true;
    }
}

fn add_pentadecathlon(world: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    let pattern = [
        (0, 1), (0, 2), (1, 0), (1, 3), (2, 1), (2, 2),
        (4, 1), (4, 2), (5, 0), (5, 3), (6, 1), (6, 2),
        (8, 1), (8, 2), (9, 0), (9, 3), (10, 1), (10, 2)
    ];
    for &(dx, dy) in &pattern {
        if x + dx < WIDTH && y + dy < HEIGHT {
            world[y + dy][x + dx] = true;
        }
    }
}

fn add_oscillator(world: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    let pattern = [
        (0, 1), (1, 1), (2, 1)
    ];
    for &(dx, dy) in &pattern {
        if x + dx < WIDTH && y + dy < HEIGHT {
            world[y + dy][x + dx] = true;
        }
    }
}

fn add_heavyweight_spaceship(world: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    let pattern = [
        (0, 1), (0, 2), (0, 3), (0, 4),
        (1, 0), (1, 4),
        (2, 4),
        (3, 0), (3, 3),
        (4, 0), (4, 1), (4, 2)
    ];
    for &(dx, dy) in &pattern {
        if x + dx < WIDTH && y + dy < HEIGHT {
            world[y + dy][x + dx] = true;
        }
    }
}

fn render(world: &Vec<Vec<bool>>, framebuffer: &mut framebuffer::Framebuffer) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let color = if world[y][x] {
                match determine_pattern(x, y) {
                    Pattern::Glider => COLOR_GLIDER,
                    Pattern::Pentadecathlon => COLOR_PENTADECATHLON,
                    Pattern::Oscillator => COLOR_OSCILLATOR,
                    Pattern::HeavyweightSpaceship => COLOR_HWSHIP,
                    Pattern::Unknown => 0xFFFFFF, // Blanco para otros patrones desconocidos
                }
            } else {
                COLOR_BACKGROUND
            };
            for dy in 0..SCALE {
                for dx in 0..SCALE {
                    framebuffer.set_point(x * SCALE + dx, y * SCALE + dy, color);
                }
            }
        }
    }
}

#[derive(PartialEq)]
enum Pattern {
    Glider,
    Pentadecathlon,
    Oscillator,
    HeavyweightSpaceship,
    Unknown,
}

fn determine_pattern(x: usize, y: usize) -> Pattern {
    // Aquí puedes implementar un método para determinar el patrón basado en la posición
    // y las células vecinas. Por simplicidad, esta función retorna Pattern::Unknown
    Pattern::Unknown
}

fn count_live_neighbors(world: &Vec<Vec<bool>>, x: usize, y: usize) -> usize {
    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && ny >= 0 && nx < WIDTH as isize && ny < HEIGHT as isize {
                if world[ny as usize][nx as usize] {
                    count += 1;
                }
            }
        }
    }
    count
}

fn update_world(world: &mut Vec<Vec<bool>>) {
    let mut new_world = world.clone();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let live_neighbors = count_live_neighbors(&world, x, y);
            if world[y][x] {
                if live_neighbors < 2 || live_neighbors > 3 {
                    new_world[y][x] = false;
                }
            } else {
                if live_neighbors == 3 {
                    new_world[y][x] = true;
                }
            }
        }
    }
    *world = new_world;
}

fn main() {
    let mut world = initialize_world();
    let mut framebuffer = framebuffer::Framebuffer::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    
    let mut window = Window::new(
        "Conway's Game of Life",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    ).unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        framebuffer.clear(COLOR_BACKGROUND); // Clear the framebuffer with a background color
        render(&world, &mut framebuffer);
        update_world(&mut world);
        window.update_with_buffer(&framebuffer.buffer, WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();
        sleep(Duration::from_millis(100));
    }
}
