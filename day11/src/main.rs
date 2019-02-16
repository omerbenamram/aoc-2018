use nalgebra::base::Dynamic;
use nalgebra::base::VecStorage;
use nalgebra::MatrixSlice3;
use nalgebra::{Matrix, U3, U5};

type Grid = Matrix<f32, Dynamic, Dynamic, VecStorage<f32, Dynamic, Dynamic>>;

fn hundreath_digit(n: i32) -> i32 {
    let mut hundreath = n / 100;
    if hundreath > 10 {
        hundreath = hundreath % 10;
    }
    hundreath
}

#[test]
fn test_hundreath() {
    assert_eq!(hundreath_digit(949), 9);
    assert_eq!(hundreath_digit(500), 5);
    assert_eq!(hundreath_digit(5000), 0);
    assert_eq!(hundreath_digit(7111), 1);
}

struct PowerGrid(Grid);

impl PowerGrid {
    pub fn with_serial_number(serial_number: i32) -> Self {
        let mut grid = Grid::zeros(300, 300);
        let serial_number = serial_number as f32;

        for (x, mut row) in grid.row_iter_mut().enumerate() {
            for (y, mut col) in row.iter_mut().enumerate() {
                let x_pos = x + 1;
                let y_pos = y + 1;
                let rack_id = (x_pos + 10) as f32;
                let power_level = rack_id * y_pos as f32;
                let with_serial_number = power_level + serial_number;
                let multiplied_by_rack_id = (with_serial_number * rack_id) as i32;
                // Integer div
                let hundreath_digit = hundreath_digit(multiplied_by_rack_id);
                let power_cell_value = hundreath_digit as f32 - 5_f32;

                *col = power_cell_value;
            }
        }

        PowerGrid(grid)
    }

    pub fn power_cell_value(&self, x: usize, y: usize) -> i32 {
        *self.0.index((x - 1, y - 1)) as i32
    }

    pub fn max_3x3(&self) -> (f32, i32, i32) {
        let mut max = 0_f32;
        let mut max_i = 0;
        let mut max_j = 0;

        for i in 0..300 - 3 {
            for j in 0..300 - 3 {
                let current = self.0.fixed_slice::<U3, U3>(i, j).sum();
                if current > max {
                    max = current;
                    max_i = i;
                    max_j = j;
                }
            }
        }
        (max, max_i as i32, max_j as i32)
    }

    pub fn max_size(&self) -> (f32, i32, i32, i32) {
        let mut max = 0_f32;
        let mut max_i = 0;
        let mut max_j = 0;
        let mut max_sz = 1;

        for sz in 1..=300 {
            for i in 0..300 - sz {
                for j in 0..300 - sz {
                    let current = self.0.slice((i, j), (sz, sz)).sum();
                    if current > max {
                        max = current;
                        max_i = i;
                        max_j = j;
                        max_sz = sz;
                    }
                }
            }
        }
        (max, max_i as i32, max_j as i32, max_sz as i32)
    }
}

fn main() {
    let grid = PowerGrid::with_serial_number(1718);
    println!("{:?}", grid.max_3x3());
    println!("{:?}", grid.max_size());
}
