```rust
fn get_neighbor_tile(chunk_sys:ChunkSystem, tile:Tile, direction:Direction) -> Option<Tile> {
    let neighbor = match direction {
        Direction::North => (tile.x, tile.y - 1),
        Direction::East => (tile.x + 1, tile.y),
        Direction::South => (tile.x, tile.y + 1),
        Direction::West => (tile.x - 1, tile.y),
    };
    chunk_sys.get_tile(neighbor.0, neighbor.1)
}
```