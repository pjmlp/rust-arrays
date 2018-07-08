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

use std::io::{self, BufRead, Stdin, ErrorKind};

/// Reads the number of array elements
fn test_cases(stdin: &Stdin) -> Result<i32, io::Error> {

    match row_data(stdin) {
       Ok(v) =>  if v.len() == 1 && v[0] >= 1 && v[0] <= 1000 {
                       Ok(v[0])
                 } else {
                       Err(io::Error::new(ErrorKind::Other, "Invalid vector"))
                 },
       Err(e) => Err(e)
    }
}

/// Gets one row of data from the input as set of numeric values
fn row_data(stdin: &Stdin) -> Result<Vec<i32>, io::Error> {
    let mut buffer = String::new();
    let mut handle = stdin.lock();

    handle.read_line(&mut buffer)?;

    let v = buffer.trim()
        .split(" ")
        .map(|x| match i32::from_str_radix(x, 10) {
            Ok(num) => num,
            Err(_) => 0,
        }).collect();
        
    Ok(v)
}

/// Reads the matrix related data from the input file.
/// The array will only be retured if the number of entries match the
/// expected size.
fn read_matrix_data(stdin: &Stdin, count: usize) -> Result<(Vec<i32>, i32), io::Error> {
    match row_data(stdin) {
       Ok(v) =>  if v.len() == count {
                    let cnt = v[0];
                    Ok((v[1..].to_vec(), cnt))
                 } else {
                       Err(io::Error::new(ErrorKind::Other, "Invalid vector size"))
                 },
       Err(e) => Err(e)
    }
}

/// Retrieves how many entries are in the file to process
fn matrix_queries(stdin: &Stdin) -> Result<i32, io::Error> {
    match row_data(stdin) {
       Ok(v) =>  if v.len() == 1 && v[0] >= 1 {
                    Ok(v[0])
                 } else {
                    Err(io::Error::new(ErrorKind::Other, "Invalid vector size"))
                 },
       Err(e) => Err(e)
    }
}

/// Basic way to index into the vector data
fn matrix_at(data: &Vec<i32>, elems: i32, row: i32, col: i32) -> i32 {
    data[(elems * (row - 1) + (col - 1)) as usize]
}

fn main() ->  Result<(), io::Error>{
    let stdin = io::stdin();
    let count = test_cases(&stdin)?;

    //println!("{}", count);

    if count > 0 {
        let data = read_matrix_data(&stdin, count as usize)?;
        //println!("{:?}", data);

        let queries = matrix_queries(&stdin)?;
        if queries >= 1 && queries <= count - 1 {
            //println!("{}", queries);

            for _ in 0..queries {
                let idx = row_data(&stdin)?;
                if idx.len() == 2 {
                    println!("{}", matrix_at(&data.0, data.1, idx[0], idx[1]));
                }
            }
        }
    }
    Ok(())
}
