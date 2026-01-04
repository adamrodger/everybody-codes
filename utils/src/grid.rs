use std::cmp::Ordering;
use std::convert::Infallible;
use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::str::FromStr;

/// A 2D point with signed integer coordinates.
///
/// The coordinate system assumes `x` is the column (increasing to the right)
/// and `y` is the row (increasing downward). This matches common 2D grid
/// indexing where `grid[y][x]` accesses the cell at `(x, y)`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    /// Create a new `Point`.
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// The origin `(0, 0)`.
    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

    /// Move one step north (up / y - 1).
    pub fn north(&self) -> Self {
        Self::new(self.x, self.y - 1)
    }

    /// Move one step south (down / y + 1).
    pub fn south(&self) -> Self {
        Self::new(self.x, self.y + 1)
    }

    /// Move one step east (right / x + 1).
    pub fn east(&self) -> Self {
        Self::new(self.x + 1, self.y)
    }

    /// Move one step west (left / x - 1).
    pub fn west(&self) -> Self {
        Self::new(self.x - 1, self.y)
    }

    /// Return the 4 orthogonally adjacent points in NESW order.
    pub fn neighbours4(&self) -> [Self; 4] {
        [self.north(), self.east(), self.south(), self.west()]
    }

    /// Return the 8 adjacent points (N, NE, E, SE, S, SW, W, NW).
    pub fn neighbours8(&self) -> [Self; 8] {
        [
            self.north(),
            Self::new(self.x + 1, self.y - 1), // NE
            self.east(),
            Self::new(self.x + 1, self.y + 1), // SE
            self.south(),
            Self::new(self.x - 1, self.y + 1), // SW
            self.west(),
            Self::new(self.x - 1, self.y - 1), // NW
        ]
    }

    /// Manhattan distance from another point.
    pub fn manhattan_distance(&self, other: Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    /// Move in a `Compass` direction by `n` steps (can be negative).
    pub fn moved(&self, dir: Compass, n: i32) -> Self {
        let (dx, dy) = dir.delta();
        Self::new(self.x + dx * n, self.y + dy * n)
    }

    /// Move in a `Compass` direction by one step.
    pub fn step(&self, dir: Compass) -> Self {
        self.moved(dir, 1)
    }

    /// Convert to a (x, y) tuple of i32.
    pub fn to_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    /// Returns true if this point lies within the bounds of `grid`.
    /// Negative coordinates are out-of-bounds.
    pub fn in_bounds<T>(&self, grid: &Grid<T>) -> bool {
        if self.x < 0 || self.y < 0 {
            return false;
        }

        let x = self.x as usize;
        let y = self.y as usize;

        y < grid.height() && x < grid.width()
    }
}

impl From<(i32, i32)> for Point {
    fn from(t: (i32, i32)) -> Self {
        Self::new(t.0, t.1)
    }
}

impl From<Point> for (i32, i32) {
    fn from(p: Point) -> Self {
        (p.x, p.y)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Basic arithmetic for points (component-wise)
impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Point {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Point) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x.cmp(&other.x).then(self.y.cmp(&other.y))
    }
}

/// Cardinal directions useful for moving a `Point`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Compass {
    North,
    East,
    South,
    West,
}

impl Compass {
    /// Delta vector for this compass direction as (dx, dy).
    /// Remember `y` increases downward for grid indexing.
    pub fn delta(&self) -> (i32, i32) {
        match self {
            Compass::North => (0, -1),
            Compass::East => (1, 0),
            Compass::South => (0, 1),
            Compass::West => (-1, 0),
        }
    }
}

/// Simple rectangular grid wrapper around `Vec<Vec<T>>`.
///
/// The grid is rectangular: every row must have the same number of columns.
pub struct Grid<T> {
    data: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    /// Construct a grid from rows. Panics if rows have differing lengths.
    pub fn from_rows(rows: Vec<Vec<T>>) -> Self {
        let height = rows.len();
        let width = rows.first().map(|r| r.len()).unwrap_or(0);

        for (i, row) in rows.iter().enumerate() {
            assert_eq!(row.len(), width, "Row {i} length differs from first row");
        }

        Self {
            data: rows,
            width,
            height,
        }
    }

    /// Height (number of rows)
    pub fn height(&self) -> usize {
        self.height
    }

    /// Width (number of columns)
    pub fn width(&self) -> usize {
        self.width
    }

    /// Immutable access by `Point`. Returns `None` if the point is negative or
    /// outside the grid bounds.
    pub fn at(&self, p: Point) -> Option<&T> {
        if p.x < 0 || p.y < 0 {
            return None;
        }

        let x = p.x as usize;
        let y = p.y as usize;

        if y < self.height && x < self.width {
            Some(&self.data[y][x])
        } else {
            None
        }
    }

    /// Mutable access by `Point`.
    pub fn at_mut(&mut self, p: Point) -> Option<&mut T> {
        if p.x < 0 || p.y < 0 {
            return None;
        }

        let x = p.x as usize;
        let y = p.y as usize;

        if y < self.height && x < self.width {
            Some(&mut self.data[y][x])
        } else {
            None
        }
    }

    /// Borrowing iterator over rows as slices (`&[T]`). Useful when you want
    /// to iterate rows without allocating or cloning.
    pub fn rows(&self) -> impl Iterator<Item = &[T]> {
        self.data.iter().map(|r| r.as_slice())
    }

    /// Mutable borrowing iterator over rows as slices (`&mut [T]`).
    pub fn rows_mut(&mut self) -> impl Iterator<Item = &mut [T]> {
        self.data.iter_mut().map(|r| r.as_mut_slice())
    }
}

// IntoIterator implementations for convenient for-loops over rows.
impl<T> IntoIterator for Grid<T> {
    type Item = Vec<T>;
    type IntoIter = std::vec::IntoIter<Vec<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Grid<T> {
    type Item = &'a Vec<T>;
    type IntoIter = std::slice::Iter<'a, Vec<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Grid<T> {
    type Item = &'a mut Vec<T>;
    type IntoIter = std::slice::IterMut<'a, Vec<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}

impl FromStr for Grid<char> {
    type Err = Infallible;

    /// Parse a string where each line is a row and each char is a cell.
    /// Panics if the rows are not all the same length.
    fn from_str(s: &str) -> Result<Self, Infallible> {
        let rows = s
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        Ok(Self::from_rows(rows))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_add_sub_basic() {
        let a = Point::new(3, 4);
        let b = Point::new(-1, 2);
        assert_eq!(a + b, Point::new(2, 6));
        assert_eq!(a - b, Point::new(4, 2));
    }

    #[test]
    fn neighbors_and_moves() {
        let p = Point::new(2, 2);
        let n = p.neighbours4();
        assert_eq!(n[0], Point::new(2, 1)); // north
        assert_eq!(n[1], Point::new(3, 2)); // east
        assert_eq!(n[2], Point::new(2, 3)); // south
        assert_eq!(n[3], Point::new(1, 2)); // west

        let n8 = p.neighbours8();
        assert_eq!(n8[0], Point::new(2, 1)); // N
        assert_eq!(n8[1], Point::new(3, 1)); // NE
        assert_eq!(n8[2], Point::new(3, 2)); // E
        assert_eq!(n8[3], Point::new(3, 3)); // SE
        assert_eq!(n8[4], Point::new(2, 3)); // S
        assert_eq!(n8[5], Point::new(1, 3)); // SW
        assert_eq!(n8[6], Point::new(1, 2)); // W
        assert_eq!(n8[7], Point::new(1, 1)); // NW

        assert_eq!(p.step(Compass::North), Point::new(2, 1));
        assert_eq!(p.moved(Compass::West, 3), Point::new(-1, 2));

        // Manhattan distance
        assert_eq!(p.manhattan_distance(Point::new(5, 7)), 8); // dx=3 dy=5 -> 8
        assert_eq!(Point::new(-1, -1).manhattan_distance(Point::new(1, 2)), 5);
    }

    #[test]
    fn grid_at_behaviour() {
        let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let grid = Grid::from_rows(rows);
        assert_eq!(grid.at(Point::new(0, 0)), Some(&1));
        assert_eq!(grid.at(Point::new(2, 1)), Some(&6));
        assert_eq!(grid.at(Point::new(3, 0)), None);
        assert_eq!(grid.at(Point::new(-1, 0)), None);
    }

    #[test]
    fn grid_at_mut() {
        let rows = vec![vec![10, 20], vec![30, 40]];
        let mut grid = Grid::from_rows(rows);
        if let Some(v) = grid.at_mut(Point::new(1, 0)) {
            *v = 42;
        }
        assert_eq!(grid.at(Point::new(1, 0)), Some(&42));
    }

    #[test]
    fn grid_from_str() {
        let grid = Grid::from_str("ab\ncd").unwrap();
        assert_eq!(grid.width(), 2);
        assert_eq!(grid.height(), 2);
        assert_eq!(grid.at(Point::new(0, 0)), Some(&'a'));
        assert_eq!(grid.at(Point::new(1, 1)), Some(&'d'));
    }

    #[test]
    #[should_panic]
    fn from_str_ragged_panics() {
        let _ = Grid::from_str("ab\nc");
    }

    #[test]
    fn point_in_bounds() {
        let rows = vec![vec![0; 3], vec![0; 3]]; // 2 rows x 3 cols
        let grid = Grid::from_rows(rows);

        assert!(Point::new(0, 0).in_bounds(&grid));
        assert!(Point::new(2, 1).in_bounds(&grid));

        assert!(!Point::new(3, 0).in_bounds(&grid)); // x out of bounds
        assert!(!Point::new(0, 2).in_bounds(&grid)); // y out of bounds
        assert!(!Point::new(-1, 0).in_bounds(&grid)); // negative x
        assert!(!Point::new(0, -1).in_bounds(&grid)); // negative y
    }

    #[test]
    fn iterate_rows_by_ref_and_value() {
        let grid = Grid::from_rows(vec![vec![1, 2], vec![3, 4]]);

        // by reference
        let mut seen = Vec::new();
        for row in &grid {
            seen.push(row.clone());
        }
        assert_eq!(seen, vec![vec![1, 2], vec![3, 4]]);

        // by value
        let grid2 = Grid::from_rows(vec![vec![5, 6], vec![7, 8]]);
        let rows: Vec<Vec<i32>> = grid2.into_iter().collect();
        assert_eq!(rows, vec![vec![5, 6], vec![7, 8]]);
    }

    #[test]
    fn iterate_rows_slices_and_mutate() {
        let mut grid = Grid::from_rows(vec![vec![1, 2], vec![3, 4]]);
        let rows: Vec<Vec<i32>> = grid.rows().map(|s| s.to_vec()).collect();
        assert_eq!(rows, vec![vec![1, 2], vec![3, 4]]);

        for row in grid.rows_mut() {
            if !row.is_empty() {
                row[0] = 9;
                break;
            }
        }
        assert_eq!(grid.at(Point::new(0, 0)), Some(&9));
    }
}
