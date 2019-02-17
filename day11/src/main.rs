use nalgebra::base::Dynamic;
use nalgebra::base::VecStorage;
use nalgebra::MatrixSlice3;
use nalgebra::{Matrix, U3, U5};
use std::fs::File;
use std::io::Write;

type Grid = Matrix<i32, Dynamic, Dynamic, VecStorage<i32, Dynamic, Dynamic>>;

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

struct PowerGrid {
    pub grid: Grid,
    pub summed_area_table: Grid,
}

impl PowerGrid {
    pub fn with_serial_number(serial_number: i32) -> Self {
        let mut grid = Grid::zeros(300, 300);

        for (x, mut row) in grid.row_iter_mut().enumerate() {
            for (y, mut col) in row.iter_mut().enumerate() {
                let x_pos = (x + 1) as i32;
                let y_pos = (y + 1) as i32;
                let rack_id = x_pos + 10;
                let power_level = rack_id * y_pos;
                let with_serial_number = power_level + serial_number;
                let multiplied_by_rack_id = (with_serial_number * rack_id) as i32;
                // Integer div
                let hundreath_digit = hundreath_digit(multiplied_by_rack_id);
                let power_cell_value = hundreath_digit - 5;

                *col = power_cell_value;
            }
        }

        let summed_area_table = PowerGrid::summed_area_table(&grid);

        PowerGrid {
            grid,
            summed_area_table,
        }
    }

    pub fn summed_area_table(g: &Grid) -> Grid {
        let mut new = g.clone();
        for mut row in new.row_iter_mut() {
            let mut agg = 0;
            for val in row.iter_mut() {
                agg += *val;
                *val = agg;
            }
        }

        for mut col in new.column_iter_mut() {
            let mut agg = 0;
            for val in col.iter_mut() {
                agg += *val;
                *val = agg;
            }
        }

        new
    }

    pub fn max3x3(&self) -> (i32, i32, i32) {
        let mut max = 0;
        let mut max_x = 0;
        let mut max_y = 0;
        for x in 2..300 {
            for y in 2..300 {
                let mut sum = *self.summed_area_table.index((x, y));
                if x >= 3 {
                    let subtract_x = *self.summed_area_table.index((x - 3, y));
                    sum -= subtract_x;
                };
                if y >= 3 {
                    let subtract_y = *self.summed_area_table.index((x, y - 3));
                    sum -= subtract_y;
                }
                if x >= 3 && y >= 3 {
                    let subtracted_twice = *self.summed_area_table.index((x - 3, y - 3));
                    sum += subtracted_twice;
                }

                if sum > max {
                    max = sum;
                    max_x = x;
                    max_y = y;
                }
            }
        }
        // Coordinates are for bottom right, 0-based.
        // To convert to top left, add 'max_sz' + 1 + 1.
        let max_x_top_left = max_x - 3 + 2;
        let max_y_top_left = max_y - 3 + 2;
        (max, (max_x_top_left) as i32, (max_y_top_left) as i32)
    }

    pub fn max_any(&self) -> (i32, i32, i32, i32) {
        let mut max = 0;
        let mut max_sz = 0;
        let mut max_x = 0;
        let mut max_y = 0;

        for sz in 3..=300 {
            for x in sz..300 - (sz - 1){
                for y in sz..300 - (sz - 1) {
                    let mut sum = *self.summed_area_table.index((x, y));
                    if x >= sz  {
                        let subtract_x = *self.summed_area_table.index((x - sz, y));
                        sum -= subtract_x;
                    };
                    if y >= sz  {
                        let subtract_y = *self.summed_area_table.index((x, y - sz));
                        sum -= subtract_y;
                    }
                    if x >= sz && y >= sz {
                        let subtracted_twice = *self.summed_area_table.index((x - sz, y - sz));
                        sum += subtracted_twice;
                    }

                    if sum > max {
                        max = sum;
                        max_sz = sz;
                        max_x = x;
                        max_y = y;
                    }
                }
            }
        }
        // Coordinates are for bottom right, 0-based.
        // To convert to top left, add 'max_sz' + 1 + 1.
        let max_x_top_left = max_x - max_sz + 2;
        let max_y_top_left = max_y - max_sz + 2;
        (max, max_sz as i32, (max_x_top_left) as i32, (max_y_top_left) as i32)
    }
}

fn main() {
    let grid = PowerGrid::with_serial_number(1718);
    println!("{:?}", grid.max3x3());
    println!("{:?}", grid.max_any());
}
