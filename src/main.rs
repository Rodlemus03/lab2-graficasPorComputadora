use minifb::{Window, WindowOptions, Key};
use std::time::Duration;
use std::thread::sleep;
mod framebuffer;

const WIDTH: usize = 80;
const HEIGHT: usize = 60;
const SCALE: usize = 10;
const WINDOW_WIDTH: usize = WIDTH * SCALE;
const WINDOW_HEIGHT: usize = HEIGHT * SCALE;

fn initialize_world() -> Vec<Vec<bool>> {
    let mut world = vec![vec![false; WIDTH]; HEIGHT];
    // Colocar algunas células vivas iniciales, por ejemplo:
    world[1][2] = true;
    world[2][3] = true;
    world[3][1] = true;
    world[3][2] = true;
    world[3][3] = true;
    world
}

fn render(world: &Vec<Vec<bool>>, framebuffer: &mut framebuffer::Framebuffer) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let color = if world[y][x] { 0xFFFFFF } else { 0x000000 };
            for dy in 0..SCALE {
                for dx in 0..SCALE {
                    framebuffer.set_point(x * SCALE + dx, y * SCALE + dy, color);
                }
            }
        }
    }
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
        framebuffer.clear(0x333355); // Clear the framebuffer with a background color
        render(&world, &mut framebuffer);
        update_world(&mut world);
        window.update_with_buffer(&framebuffer.buffer, WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();
        sleep(Duration::from_millis(100));
    }
}
