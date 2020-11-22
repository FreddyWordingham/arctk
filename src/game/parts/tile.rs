//1 Landscape enumeration.

/// Tile kinds
#[derive(PartialEq, Copy, Clone)]
enum TileType {
    /// Inaccessible.
    Wall,
    /// Open flior.
    Floor,
}
