/*
 * rust-arrays, A solution to an Hacherrank exercise.
 * Copyright (C) 2018, Paulo Pinto
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301  USA
 */

use std::io::{self, BufRead, Stdin};

/// Reads the number of array elements
fn test_cases(stdin: &Stdin) -> i32 {
    let v = row_data(stdin);
    if v.len() == 1 && v[0] >= 1 && v[0] <= 1000 {
        v[0]
    } else {
        0
    }
}

/// Gets one row of data from the input as set of numeric values
fn row_data(stdin: &Stdin) -> Vec<i32> {
    let mut buffer = String::new();
    let mut handle = stdin.lock();

    if let Err(_) = handle.read_line(&mut buffer) {
        return vec![];
    }

    buffer.trim()
        .split(" ")
        .map(|x| match i32::from_str_radix(x, 10) {
            Ok(num) => num,
            Err(_) => 0,
        }).collect()
}

/// Reads the matrix related data from the input file.
/// The array will only be retured if the number of entries match the
/// expected size.
fn read_matrix_data(stdin: &Stdin, count: usize) -> (Vec<i32>, i32) {
    let v = row_data(stdin);
    if v.len() == count {
        let cnt = v[0];
        (v[1..].to_vec(), cnt)
    } else {
        (vec![], 0)
    }
}

/// Retrieves how many entries are in the file to process
fn matrix_queries(stdin: &Stdin) -> i32 {
    let v = row_data(stdin);
    if v.len() == 1 && v[0] >= 1 {
        v[0]
    } else {
        0
    }
}

/// Basic way to index into the vector data
fn matrix_at(data: &Vec<i32>, elems: i32, row: i32, col: i32) -> i32 {
    data[(elems * (row - 1) + (col - 1)) as usize]
}

fn main() {
    let stdin = io::stdin();
    let count = test_cases(&stdin);

    //println!("{}", count);

    if count > 0 {
        let data = read_matrix_data(&stdin, count as usize);
        //println!("{:?}", data);

        let queries = matrix_queries(&stdin);
        if queries >= 1 && queries <= count - 1 {
            //println!("{}", queries);

            for _ in 0..queries {
                let idx = row_data(&stdin);
                if idx.len() == 2 {
                    println!("{}", matrix_at(&data.0, data.1, idx[0], idx[1]));
                }
            }
        }
    }
}
