use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("Part 1 trees: {}", traverse_slope(&input, 3, 1));
    println!(
        "Part 2 trees: {}",
        traverse_slope(&input, 1, 1)
            * traverse_slope(&input, 3, 1)
            * traverse_slope(&input, 5, 1)
            * traverse_slope(&input, 7, 1)
            * traverse_slope(&input, 1, 2)
    );

    Ok(())
}

fn traverse_slope(ski_map: &str, x_inc: usize, y_inc: usize) -> u64 {
    let mut map_grid: Vec<Vec<char>> = Vec::new();
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut tree_count: u64 = 0;
    for line in ski_map.lines() {
        map_grid.push(line.trim().chars().collect())
    }
    let y_max: usize = map_grid.len();
    let x_max: usize = map_grid[0].len();
    while y < y_max {
        if map_grid[y][x] == '#' {
            tree_count += 1
        }
        x = (x + x_inc) % x_max;
        y += y_inc;
    }
    tree_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_traverse_slope() {
        assert_eq!(
            7,
            traverse_slope(
                "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#",
                3,
                1
            )
        );
        assert_eq!(
            2,
            traverse_slope(
                "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#",
                1,
                1
            )
        );
        assert_eq!(
            3,
            traverse_slope(
                "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#",
                5,
                1
            )
        );
        assert_eq!(
            4,
            traverse_slope(
                "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#",
                7,
                1
            )
        );
        assert_eq!(
            2,
            traverse_slope(
                "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#",
                1,
                2
            )
        );
    }
}
