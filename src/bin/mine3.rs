use grid::Grid;
use rand::seq::SliceRandom;
use std::ops::Mul;

// TODO:
// - maybe include a way for DimensionsWithBombsAmount to pick the bombs, instead of delegating it to Minesweeper
// - reinclude NEIGHBORHOOD_AMOUNT and have a better way of calculating max_bombs_amount

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    println!("hi");

    let mut m = Minesweeper::new(DimensionsWithBombsAmount::new(5, 5, 5)?);

    println!("{}", m.to_str());
    m.reveal(Coord::new(1, 1)).unwrap();
    println!("new\n{}", m.to_str());

    Ok(())
}



#[derive(thiserror::Error, Debug, Clone)]
pub enum MinesweeperError {
    #[error("Too many bombs")]
    TooManyBombs { provided: usize, max_allowed: usize },
    #[error("Grid dimensions must be non-negative")]
    InvalidDimensions,
    #[error("Out of bounds")]
    CoordOutOfBounds { provided_coord: Coord, upper_bound_coord: Coord },
    #[error("Invalid coord string, provided `{provided}`, expected something of the form `num num`")]
    InvalidCoordString { provided: String },
    #[error("Parse error: {0}")]
    ParseError(#[from] std::num::ParseIntError),
    #[error("The game has already concluded, you cannot reveal cells anymore")]
    GameAlreadyFinished
}



#[derive(Debug, Clone, Default, PartialEq, Eq)]
enum GameState {
    #[default]
    NoBombs,
    Playing,
    GameOver,
    GameWon
}


#[derive(Debug, Clone)]
pub struct Minesweeper {
    pub cells: Grid<MinesweeperCell>,
    state: GameState,
    bombs_amount: usize
}


pub struct DimensionsWithBombsAmount {
    pub dimensions: Dimensions,
    pub bombs_amount: usize
}

impl DimensionsWithBombsAmount {
    /// 0.5 would mean that at most 50% of the grid space can be bombs
    pub const MAX_BOMB_DENSITY: f64 = 0.4;
    /// when the play chooses a cell we want the neighborhood around it to also be bomb-free, hence 9
    // pub const NEIGHBORHOOD_AMOUNT: usize = 9;

    pub fn parse(dimensions: Dimensions, bombs_amount: usize) -> Result<Self, MinesweeperError> {
        let max_allowed = (dimensions.rows.mul(dimensions.cols) as f64)
            .mul(Self::MAX_BOMB_DENSITY)
            .floor() as usize;

        if bombs_amount > max_allowed {
            return Err(MinesweeperError::TooManyBombs { provided: bombs_amount, max_allowed })
        }

        Ok(Self { dimensions, bombs_amount })
    }

    pub fn new(rows: usize, cols: usize, bombs_amount: usize) -> Result<Self, MinesweeperError> {
        let dimensions = Dimensions::parse(rows, cols)?;

        Self::parse(dimensions, bombs_amount)
    }
}


impl Minesweeper {
    pub fn new(dims_with_bombs: DimensionsWithBombsAmount) -> Self {
        let DimensionsWithBombsAmount {
            dimensions,
            bombs_amount
        } = dims_with_bombs;

        let cells = Grid::<MinesweeperCell>::new(dimensions.rows, dimensions.cols);

        Self { cells, state: GameState::NoBombs, bombs_amount }
    }

    // use a VerifiedCoord instead of just a regular Coord, somehow.
    // with that one could also get the cell later, after the match instead
    pub fn reveal(&mut self, coord: Coord) -> Result<(), MinesweeperError> {
        // let upper_bound_coord = self.upper_bound();
        // let cell = self.cells.get_mut(coord.i, coord.j)
        //     .ok_or(MinesweeperError::CoordOutOfBounds { provided_coord: coord, upper_bound_coord })?;

        match self.state {
            GameState::NoBombs => {
                self.place_bombs(coord);
                self.state = GameState::Playing;
            },
            GameState::Playing => {

            },
            GameState::GameOver | GameState::GameWon => return Err(MinesweeperError::GameAlreadyFinished)
        }
        let upper_bound_coord = self.upper_bound();
        let cell = self.cells.get_mut(coord.i, coord.j)
            .ok_or(MinesweeperError::CoordOutOfBounds { provided_coord: coord, upper_bound_coord })?;
        cell.hidden = false;

        Ok(())
    }

    // could return a result? maybe? idk? what are the cases? test this
    pub fn place_bombs(&mut self, coord_to_ommit: Coord) {
        let rows = self.cells.rows();
        let cols = self.cells.cols();

        // currently no bombs can be placed in the neighborhood of the coord chosen (and the coord itself ofc)
        let mut available_coords = (0..rows)
            .flat_map(|i| (0..cols).map(move |j| Coord::new(i, j)))
            .filter(|&c| c != coord_to_ommit && !coord_to_ommit.neighbors(rows, cols).any(|n| n == c))
            .collect::<Vec<_>>();
        println!("len available_coords: {}", available_coords.len());

        available_coords.shuffle(&mut rand::thread_rng());

        let bomb_coords = available_coords.into_iter().take(self.bombs_amount);

        for c in bomb_coords {
            self.cells[(c.i, c.j)] = MinesweeperCell::new(CellData::Bomb);
            for neighbor in c.neighbors(rows, cols) {
                self.cells[(neighbor.i, neighbor.j)].data.increment_if_possible()
            }
        }

        self.state = GameState::Playing;
    }

    #[deprecated]
    pub fn new2(dimensions: Dimensions, bombs_amount: usize) -> Result<Self, MinesweeperError> {
        let rows = dimensions.rows;
        let cols = dimensions.cols;

        let max_cells = rows * cols;
        if bombs_amount > max_cells {
            return Err(MinesweeperError::TooManyBombs {
                provided: bombs_amount,
                max_allowed: max_cells,
            });
        }

        let mut grid = Grid::<MinesweeperCell>::new(rows, cols);

        let mut all_coords = (0..rows)
            .flat_map(|i| (0..cols).map(move |j| Coord::new(i, j)))
            .collect::<Vec<_>>();

        all_coords.shuffle(&mut rand::thread_rng());

        let bomb_coords = all_coords.into_iter().take(bombs_amount);

        for c in bomb_coords {
            grid[(c.i, c.j)] = MinesweeperCell::new(CellData::Bomb);
            for neighbor in c.neighbors(rows, cols) {
                grid[(neighbor.i, neighbor.j)].data.increment_if_possible()
            }
        }

        Ok(Self { cells: grid, state: GameState::default(), bombs_amount })
    }


    pub fn to_str(&self) -> String {
        self.to_str_with(|cell| match cell.data {
            CellData::Bomb => "*".to_string(),
            CellData::Num(n) => n.to_string(),
        })
    }

    pub fn to_str_with(&self, f: impl Fn(&MinesweeperCell) -> String) -> String {
        self.cells.iter_rows()
            .map(|row| {
                row.map(|cell| f(cell))
                .collect::<Vec<String>>()
                .join("")
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn upper_bound(&self) -> Coord {
        Coord::new(self.cells.rows(), self.cells.cols())
    }

    // pub fn reveal2(&mut self, coord: Coord) -> Result<RevealAction, MinesweeperError> {
    //     let max_allowed_coord = self.upper_bound();
    //     let cell = self.cells.get_mut(coord.i, coord.j)
    //         .ok_or(MinesweeperError::CoordOutOfBounds { provided_coord: coord, max_allowed_coord })?;

    //     if !cell.hidden {
    //         return Ok(RevealAction::AlreadyRevealed)
    //     }

    //     cell.hidden = false;

    //     Ok(RevealAction::Success { revealed: cell.data.clone() })
    // }
}

#[derive(Debug, Clone)]
pub enum RevealAction {
    Success { revealed: CellData },
    AlreadyRevealed,
}


#[derive(Debug, Clone)]
pub struct MinesweeperCell {
    data: CellData,
    hidden: bool
}

impl MinesweeperCell {
    pub fn new(data: CellData) -> Self {
        Self { data, hidden: true }
    }
}

impl Default for MinesweeperCell {
    fn default() -> Self {
        Self {
            data: CellData::default(),
            hidden: true,
        }
    }
}

#[derive(Debug, Clone)]
pub enum CellData {
    Num(i32),
    Bomb
}

impl CellData {
    pub fn increment_if_possible(&mut self) {
        if let Self::Num(n) = self {
           *n += 1;
        }
    }
}

impl Default for CellData {
    fn default() -> Self {
        Self::Num(0)
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Dimensions {
    rows: usize,
    cols: usize
}

impl Dimensions {
    pub fn parse(rows: usize, cols: usize) -> Result<Self, MinesweeperError> {
        if rows <= 0 || cols <= 0 {
            return Err(MinesweeperError::InvalidDimensions);
        }

        Ok(Self { rows, cols })
    }
}


#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Coord {
    i: usize,
    j: usize
}

impl Coord {
    pub fn new(i: usize, j: usize) -> Self {
        Self { i, j }
    }

    /// format: `num num`
    pub fn from_str(s: &str) -> Result<Self, MinesweeperError> {
        let (left, right) = s.split_once(" ")
            .ok_or(MinesweeperError::InvalidCoordString { provided: s.into() })?;

        let i = left.trim().parse::<usize>()?;
        let j = right.trim().parse::<usize>()?;

        Ok(Self::new(i, j))
    }

    /// Returns an iterator over the neighbors of the coordinate.
    ///
    /// The neighbors of a coordinate are the 8 cells that surround it, if they exist. This method takes into account the dimensions of the grid to ensure that it doesn't return coordinates outside the grid.
    ///
    /// # Arguments
    ///
    /// * `rows`: The number of rows in the grid.
    /// * `cols`: The number of columns in the grid.
    ///
    /// # Returns
    ///
    /// An iterator over the neighbors of the coordinate.
    ///
    /// # Examples
    ///
    /// ```
    /// let coord = Coord::new(1, 2);
    /// let neighbors = coord.neighbors(3, 3);
    /// for neighbor in neighbors {
    ///     println!("{:?}", neighbor);
    /// }
    /// ```
    ///
    /// This will print:
    ///
    /// ```
    /// Coord { i: 0, j: 1 }
    /// Coord { i: 0, j: 2 }
    /// Coord { i: 0, j: 3 }
    /// Coord { i: 1, j: 1 }
    /// Coord { i: 1, j: 3 }
    /// Coord { i: 2, j: 1 }
    /// Coord { i: 2, j: 2 }
    /// Coord { i: 2, j: 3 }
    /// ```
    ///
    /// # Visual Representation
    ///
    /// Here's a simple representation of a cell and its neighbors in a grid. The filled square (■) represents the cell in question, and the empty squares (□) represent its neighbors.
    ///
    /// ```
    /// □ □ □
    /// □ ■ □
    /// □ □ □
    /// ```
    ///
    /// In this representation, the cell at the center (■) has eight neighbors (□), assuming it's not on the edge of the grid. If the cell is on the edge or in a corner of the grid, it will have fewer neighbors. For example, a cell in the corner of the grid would have only three neighbors:
    ///
    /// ```
    /// ■ □
    /// □ □
    /// ```
    ///
    /// And a cell on the edge of the grid (but not in a corner) would have five neighbors:
    ///
    /// ```
    /// □ ■ □
    /// □ □ □
    /// ```
    pub fn neighbors(&self, rows: usize, cols: usize) -> impl Iterator<Item = Self> + '_ {
        let lower_i = self.i.saturating_sub(1);
        let lower_j = self.j.saturating_sub(1);
        let upper_i = (rows - 1).min(self.i + 1);
        let upper_j = (cols - 1).min(self.j + 1);

        println!("center: {:?}, bounds: {:?} - {:?}", self, (lower_i, lower_j), (upper_i, upper_j));

        // let idk = (lower_i..upper_i)
        // .flat_map(move |r| {
        //     (lower_j..upper_j).filter_map(move |c| {
        //         if r == self.i && c == self.j {
        //             None
        //         } else {
        //             Some(Self::new(r, c))
        //         }
        //     })
        // });

        (lower_i..=upper_i)
        .flat_map(move |r| (lower_j..=upper_j).map(move |c| Self::new(r, c)))
        .filter(move |coord| coord != self)
    }
}



pub fn random_shuffle<T>(list: &mut [T]) {
    let mut rng = rand::thread_rng();
    list.shuffle(&mut rng);
}