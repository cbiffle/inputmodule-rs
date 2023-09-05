use rp2040_hal::{
    gpio::bank0::{Gpio26, Gpio27},
    pac::I2C1,
};

use crate::fl16::LedMatrix;
use crate::led_hal as bsp;
use crate::mapping::*;
use crate::matrix::*;

/// Bytes needed to represent all LEDs with a single bit
/// math.ceil(WIDTH * HEIGHT / 8)
pub const DRAW_BYTES: usize = 39;

/// Maximum number of brightneses levels
pub const BRIGHTNESS_LEVELS: u8 = 255;

pub type Foo = LedMatrix<
    bsp::hal::I2C<
        I2C1,
        (
            bsp::hal::gpio::Pin<Gpio26, bsp::hal::gpio::Function<bsp::hal::gpio::I2C>>,
            bsp::hal::gpio::Pin<Gpio27, bsp::hal::gpio::Function<bsp::hal::gpio::I2C>>,
        ),
    >,
>;

pub fn draw(bytes: &[u8; DRAW_BYTES]) -> Grid {
    let mut grid = Grid::default();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let index = x + WIDTH * y;
            let byte = index / 8;
            let bit = index % 8;
            let val = if bytes[byte] & (1 << bit) > 0 {
                0xFF
            } else {
                0x00
            };
            grid.0[8 - x][y] = val;
        }
    }

    grid
}

pub fn draw_grey_col(grid: &mut Grid, col: u8, levels: &[u8; HEIGHT]) {
    // TODO: I don't think I need the [..HEIGHT] slicing
    grid.0[8 - col as usize][..HEIGHT].copy_from_slice(&levels[..HEIGHT]);
}

pub fn display_sleep_reason(sleep_reason: SleepReason) -> Grid {
    let mut grid = Grid::default();

    match sleep_reason {
        SleepReason::Command => {
            display_letter(20, &mut grid, CAP_C);
            display_letter(10, &mut grid, CAP_M);
            display_letter(0, &mut grid, CAP_D);
        }
        SleepReason::SleepPin => {
            display_letter(23, &mut grid, CAP_S);
            display_letter(13, &mut grid, CAP_L);
            display_letter(7, &mut grid, CAP_P);
            display_letter(0, &mut grid, HASH);
        }
        SleepReason::Timeout => {
            display_letter(24, &mut grid, CAP_T);
            display_letter(16, &mut grid, CAP_I);
            display_letter(8, &mut grid, CAP_M);
            display_letter(0, &mut grid, CAP_E);
        }
        SleepReason::UsbSuspend => {
            display_letter(17, &mut grid, CAP_U);
            display_letter(10, &mut grid, CAP_S);
            display_letter(0, &mut grid, CAP_B);
        }
    };

    grid
}

pub fn display_sleep() -> Grid {
    Grid([
        [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
        [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
        [
            0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00,
            0xFF, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00,
            0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00,
        ],
        [
            0xFF, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00,
            0xFF, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00,
            0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00,
        ],
        [
            0xFF, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00,
            0xFF, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00,
            0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00,
        ],
        [
            0xFF, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00,
            0xFF, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00,
            0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00,
        ],
        [
            0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        ],
        [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
        [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
    ])
}

pub fn display_panic() -> Grid {
    Grid([
        [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
        [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
        [
            0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00,
            0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF,
        ],
        [
            0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00,
            0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF,
        ],
        [
            0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00,
            0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00,
            0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF,
        ],
        [
            0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00,
            0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00,
            0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF,
        ],
        [
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        ],
        [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
        [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
    ])
}

pub fn display_lotus() -> Grid {
    let mut grid = Grid::default();

    display_letter(26, &mut grid, CAP_L);
    display_letter(20, &mut grid, CAP_O);
    display_letter(12, &mut grid, CAP_T);
    display_letter(0, &mut grid, CAP_S);
    display_letter(5, &mut grid, CAP_U);

    grid
}

pub fn display_lotus2() -> Grid {
    Grid([
        [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
        [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
        [
            0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00,
            0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00,
            0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0xFF,
        ],
        [
            0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00,
            0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00,
            0xFF, 0x00, 0xFF, 0x00, 0x00, 0xFF,
        ],
        [
            0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00,
            0xFF, 0x00, 0xFF, 0x00, 0x00, 0xFF,
        ],
        [
            0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00,
            0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00,
            0xFF, 0x00, 0xFF, 0x00, 0x00, 0xFF,
        ],
        [
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00,
            0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00,
            0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF,
        ],
        [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
        [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
    ])
}

pub fn display_letter(pos: usize, grid: &mut Grid, letter: SingleDisplayData) {
    let letter_size = 8;
    for x in 0..letter_size {
        for y in 0..letter_size {
            let val = if letter[x] & (1 << y) > 0 { 0xFF } else { 0 };
            grid.0[letter_size - x][y + pos] = val;
        }
    }
}

/// Gradient getting brighter from top to bottom
pub fn gradient() -> Grid {
    let gradient_drop = 1; // Brightness drop between rows
    let mut grid = Grid::default();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            grid.0[x][y] = (gradient_drop * (y + 1)) as u8;
        }
    }
    grid
}

/// Fill a percentage of the rows from the bottom up
pub fn percentage(percentage: u16) -> Grid {
    let mut grid = Grid::default();
    let first_row = HEIGHT * (percentage as usize) / 100;
    for y in (HEIGHT - first_row)..HEIGHT {
        for x in 0..WIDTH {
            grid.0[x][y] = 0xFF;
        }
    }
    grid
}

/// Double sided gradient, bright in the middle, dim top and bottom
pub fn double_gradient() -> Grid {
    let gradient_drop = 1; // Brightness drop between rows
    let mut grid = Grid::default();
    for y in 0..(HEIGHT / 2) {
        for x in 0..WIDTH {
            grid.0[x][y] = (gradient_drop * (y + 1)) as u8;
        }
    }
    for y in (HEIGHT / 2)..HEIGHT {
        for x in 0..WIDTH {
            grid.0[x][y] = (HEIGHT - gradient_drop * (y + 1)) as u8;
        }
    }
    grid
}

/// Same as fill_grid_pixels but does each pixel individually
/// So it's much slower because it has to send 306 I2C commands
pub fn _fill_grid(grid: &Grid, matrix: &mut Foo) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            matrix.device.pixel(x as u8, y as u8, grid.0[x][y]).unwrap();
        }
    }
}

pub fn set_brightness(state: &mut LedmatrixState, brightness: u8, matrix: &mut Foo) {
    state.brightness = brightness;
    fill_grid_pixels(state, matrix);
}

/// Just sends two I2C commands for the entire grid
pub fn fill_grid_pixels(state: &LedmatrixState, matrix: &mut Foo) {
    // 0xB4 LEDs on the first page, 0xAB on the second page
    let mut brightnesses = [0x00; 0xB4 + 0xAB];
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let (register, page) = (matrix.device.calc_pixel)(x as u8, y as u8);
            brightnesses[(page as usize) * 0xB4 + (register as usize)] =
                ((state.grid.0[x][y] as u64) * (state.brightness as u64)
                    / (BRIGHTNESS_LEVELS as u64)) as u8;
        }
    }
    matrix.device.fill_matrix(&brightnesses).unwrap();
}

pub fn full_brightness(matrix: &mut Foo) {
    // Fills every pixel individually
    //matrix.fill_brightness(0xFF).unwrap();

    // Fills full page at once
    matrix.device.fill(0xFF).unwrap();
}

pub fn zigzag() -> Grid {
    let mut grid = Grid::default();

    // 1st Right to left
    for i in 0..WIDTH {
        grid.0[i][i] = 0xFF;
    }
    // 1st Left to right
    for i in 0..WIDTH {
        grid.0[WIDTH - 1 - i][WIDTH + i] = 0xFF;
    }
    // 2nd right to left
    for i in 0..WIDTH {
        grid.0[i][2 * WIDTH + i] = 0xFF;
    }
    // 2nd left to right
    for i in 0..WIDTH {
        if 3 * WIDTH + i < HEIGHT {
            grid.0[WIDTH - 1 - i][3 * WIDTH + i] = 0xFF;
        }
    }

    // Finish it off nicely
    grid.0[1][33] = 0xFF;

    grid
}

pub fn every_nth_col(n: usize) -> Grid {
    let mut grid = Grid::default();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if x % n == 0 {
                grid.0[x][y] = 0xFF;
            }
        }
    }

    grid
}
