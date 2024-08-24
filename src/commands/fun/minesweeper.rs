use std::{cell::Cell, collections::HashSet};
use poise::{serenity_prelude::{self as serenity, CreateEmbed}, CreateReply};
use rand::Rng;
use tokio::sync::mpsc;
use crate::{CartChannel, Context, Error};
use grid::Grid;
use tracing::debug;
use rand::prelude::SliceRandom;

const MINE: &'static str = "<:iknowwhatyouare:1276543226152615936>";


/// OWNER ONLY
#[poise::command(
    prefix_command,
    slash_command,
    subcommands("start", "move_com", "classic"),
    subcommand_required,
)]
pub async fn boysweeper(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
async fn start(ctx: Context<'_>) -> Result<(), Error> {
    let user_channel_key = (ctx.author().id, ctx.channel_id());
    let (tx, mut rx) = mpsc::channel::<Coord>(10);
    let mut minesweeper = Minesweeper::new(Dimensions::parse(5, 5)?, 5)?;

    ctx.data().minesweepers.insert(user_channel_key, tx);

    ctx.send(CreateReply::default().embed(create_minesweeper_embed(&minesweeper))).await?;

    // todo: pass coords that are guaranteed to be verified and be fine with the board size
    while let Some(coord) = rx.recv().await {
        let res = minesweeper.reveal(coord)?;
        dbg!(&minesweeper);

        // things to consider:
        // 1. out of bounds
        // 2. already revealed
        // 3. bomb
        // 4. winnar

        // other things to note:
        // - always remove the correct kv from the dashmap (if this command is used in a channel in which the bot cannot speak, an entry will be created, but it wont be removed if it fails in this loop, since we would exit early)

        ctx.send(CreateReply::default()
            .embed(create_minesweeper_embed(&minesweeper)))
            .await?;
    }

    Ok(())
}

fn create_minesweeper_embed(minesweeper: &Minesweeper) -> CreateEmbed {
    let s = minesweeper.to_str_with(|cell| {
        if cell.hidden {
            ":white_large_square:".into()
        } else {
            match cell.data {
                CellData::Bomb => format!("{}", MINE.to_string()),
                CellData::Num(n) => format!(":number_{}:", n)
            }
        }
    });

    CreateEmbed::default()
        .title("minesweeper for user")
        .description(s)
}

#[poise::command(prefix_command, slash_command, rename = "move", ephemeral)]
async fn move_com(
    ctx: Context<'_>, 
    #[rename = "move"]
    #[description = "Your next boysweeper move, for example A1"]
    move_param: String
) -> Result<(), Error> {
    let user_channel_key = (ctx.author().id, ctx.channel_id());

    // ctx.say(format!("You uncovered `{}`. The original embed will be edited. This msg is invisible.", move_param)).await?;

    // todo: make sure this is a verified coord and is guaranteed to be fine with the dimensions and all that
    let coord = Coord::from_str(&move_param)?;

    // DO NOT FORGET THE DAMN CLONE
    // DO NOT HAVE dashmap::Ref, clone out of it
    // or else deadlock
    if let Some(sender) = ctx.data().minesweepers.get(&user_channel_key).map(|sender| sender.clone()) {
        sender.send(coord.clone()).await?;
        ctx.say(format!("your move: `{:?}`", coord)).await?;
    } else {
        ctx.say("start a minesweeper game first bruh").await?;
    }

    Ok(())
}

/// Classic boysweeper, click click click
#[poise::command(prefix_command, slash_command)]
async fn classic(ctx: Context<'_>) -> Result<(), Error> {
    let minesweeper = Minesweeper::new(Dimensions::parse(5, 5)?, 5)?;

    let s = minesweeper.to_str_with(|cell| {
        match cell.data {
            CellData::Bomb => format!("||{}||", MINE.to_string()),
            CellData::Num(n) => format!("||:number_{}:||", n)
        }
    });

    ctx.say(s).await?;

    Ok(())
}




























#[poise::command(prefix_command, slash_command)]
pub async fn shop(ctx: Context<'_>) -> Result<(), Error> {
    let user_channel_key = (ctx.author().id, ctx.channel_id());
    let (tx, mut rx) = mpsc::channel::<String>(10);
    let mut cart: Vec<String> = vec![];

    ctx.data().carts.insert(user_channel_key, tx);

    ctx.send(CreateReply::default().embed(create_embed(&cart))).await?;

    // while let Some(product) = rx.recv().await {
    //     cart.push(product);

    //     ctx.send(CreateReply::default()
    //         .embed(create_embed(&cart))
    //         .content("cart updated"))
    //         .await?;
    // }

    loop {
        match tokio::time::timeout(tokio::time::Duration::from_secs(10), rx.recv()).await {
            Ok(Some(product)) => {
                cart.push(product);
                ctx.send(CreateReply::default()
                    .embed(create_embed(&cart))
                    .content("cart updated"))
                    .await?;
            },
            Ok(None) => {
                break;
            },
            Err(_e) => {
                ctx.say("you waited too long").await?;
                break;
            }
        }
    }

    ctx.data().carts.remove(&user_channel_key);

    Ok(())
}

#[poise::command(prefix_command, slash_command, ephemeral)]
pub async fn add(ctx: Context<'_>, product: String) -> Result<(), Error> {
    let user_channel_key = (ctx.author().id, ctx.channel_id());

    // DO NOT FORGET THE DAMN CLONE
    // DO NOT HAVE dashmap::Ref, clone out of it
    // or else deadlock
    if let Some(cart_sender) = ctx.data().carts.get(&user_channel_key).map(|sender| sender.clone()) {
        cart_sender.send(product.clone()).await?;
        ctx.say(format!("you added `{}`", product)).await?;
    } else {
        ctx.say("start shWopping first bruh").await?;
    }

    Ok(())
}

fn create_embed(cart: &[String]) -> CreateEmbed {
    CreateEmbed::default()
        .title("your shopping cart:")
        .description(cart.join(", "))
}















#[derive(thiserror::Error, Debug, Clone)]
pub enum MinesweeperError {
    #[error("Too many bombs")]
    TooManyBombs { provided: usize, max_allowed: usize },
    #[error("Grid dimensions must be non-negative")]
    InvalidDimensions,
    #[error("Out of bounds")]
    CoordOutOfBounds { provided_coord: Coord, max_allowed_coord: Coord },
    #[error("Invalid coord string, provided `{provided}`, expected something of the form `num num`")]
    InvalidCoordString { provided: String },
    #[error("Parse error: {0}")]
    ParseError(#[from] std::num::ParseIntError),
}


#[derive(Debug, Clone)]
pub struct Minesweeper {
    pub cells: Grid<MinesweeperCell>
}

impl Minesweeper {
    pub fn new(dimensions: Dimensions, bombs_amount: usize) -> Result<Self, MinesweeperError> {
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

        Ok(Self { cells: grid })
    }

    pub fn new_empty(dimensions: Dimensions) -> Self {
        Self { cells: Grid::<MinesweeperCell>::new(dimensions.rows, dimensions.cols) }
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

    fn size_coord(&self) -> Coord {
        Coord::new(self.cells.rows(), self.cells.cols())
    }

    pub fn reveal(&mut self, coord: Coord) -> Result<RevealAction, MinesweeperError> {
        let max_allowed_coord = self.size_coord();
        let cell = self.cells.get_mut(coord.i, coord.j)
            .ok_or(MinesweeperError::CoordOutOfBounds { provided_coord: coord, max_allowed_coord })?;

        if !cell.hidden {
            return Ok(RevealAction::AlreadyRevealed)
        }

        cell.hidden = false;

        Ok(RevealAction::Success { revealed: cell.data.clone() })
    }
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


#[derive(Debug, Clone, Hash, Eq, PartialEq)]
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