//! Post-generation trimming for linked levels
//!
//! Removes unnecessary wall padding from generated levels by finding the
//! actual bounding box of playable content.

/// Tiles that indicate playable/interesting content (not just walls)
fn is_content_tile(tile: char) -> bool {
    // Anything that's not a solid wall or empty space is content
    !matches!(tile, '#' | ' ')
}

/// Find the bounding box of actual content in a tilemap
/// Returns (min_x, min_y, max_x, max_y) or None if empty
fn find_content_bounds(tiles: &[Vec<char>]) -> Option<(usize, usize, usize, usize)> {
    let height = tiles.len();
    if height == 0 {
        return None;
    }

    let mut min_x = usize::MAX;
    let mut max_x = 0;
    let mut min_y = usize::MAX;
    let mut max_y = 0;

    for (y, row) in tiles.iter().enumerate() {
        for (x, &tile) in row.iter().enumerate() {
            if is_content_tile(tile) {
                min_x = min_x.min(x);
                max_x = max_x.max(x);
                min_y = min_y.min(y);
                max_y = max_y.max(y);
            }
        }
    }

    if min_x == usize::MAX {
        return None;
    }

    Some((min_x, min_y, max_x, max_y))
}

/// Trim a tilemap to its content bounds with a margin of solid walls
///
/// The margin ensures the level has proper boundaries after trimming.
/// Returns the trimmed tilemap and new dimensions.
pub fn trim_tilemap(tiles: &[Vec<char>], margin: usize) -> (Vec<Vec<char>>, usize, usize) {
    let Some((min_x, min_y, max_x, max_y)) = find_content_bounds(tiles) else {
        return (tiles.to_vec(), tiles.first().map(|r| r.len()).unwrap_or(0), tiles.len());
    };

    // Calculate trimmed region with margin (clamped to original bounds)
    let orig_width = tiles.first().map(|r| r.len()).unwrap_or(0);
    let orig_height = tiles.len();

    let trim_min_x = min_x.saturating_sub(margin);
    let trim_min_y = min_y.saturating_sub(margin);
    let trim_max_x = (max_x + margin + 1).min(orig_width);
    let trim_max_y = (max_y + margin + 1).min(orig_height);

    let new_width = trim_max_x - trim_min_x;
    let new_height = trim_max_y - trim_min_y;

    // Extract the trimmed region
    let trimmed: Vec<Vec<char>> = tiles[trim_min_y..trim_max_y]
        .iter()
        .map(|row| {
            if trim_max_x <= row.len() {
                row[trim_min_x..trim_max_x].to_vec()
            } else {
                // Handle rows that are shorter than expected
                let mut new_row = row[trim_min_x.min(row.len())..].to_vec();
                new_row.resize(new_width, '#');
                new_row
            }
        })
        .collect();

    (trimmed, new_width, new_height)
}

/// Trim a tilemap string and return the trimmed string with updated dimensions
pub fn trim_tilemap_string(tilemap: &str, margin: usize) -> (String, usize, usize) {
    // Parse tilemap string into 2D char array
    let tiles: Vec<Vec<char>> = tilemap
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let (trimmed, width, height) = trim_tilemap(&tiles, margin);

    // Convert back to string
    let trimmed_string = trimmed
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");

    (trimmed_string, width, height)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_content_bounds_simple() {
        let tiles = vec![
            vec!['#', '#', '#', '#', '#'],
            vec!['#', ' ', 'P', ' ', '#'],
            vec!['#', ' ', '=', ' ', '#'],
            vec!['#', '#', '#', '#', '#'],
        ];

        let bounds = find_content_bounds(&tiles);
        assert_eq!(bounds, Some((2, 1, 2, 2))); // P at (2,1), = at (2,2)
    }

    #[test]
    fn test_trim_tilemap_removes_padding() {
        let tiles = vec![
            vec!['#', '#', '#', '#', '#', '#', '#'],
            vec!['#', '#', '#', '#', '#', '#', '#'],
            vec!['#', '#', 'P', ' ', '>', '#', '#'],
            vec!['#', '#', '=', '=', '=', '#', '#'],
            vec!['#', '#', '#', '#', '#', '#', '#'],
            vec!['#', '#', '#', '#', '#', '#', '#'],
        ];

        let (trimmed, width, height) = trim_tilemap(&tiles, 1);

        // With margin of 1, should keep one row/col of walls around content
        assert_eq!(height, 4); // rows 1-4 (content rows 2-3 + margin)
        assert_eq!(width, 5);  // cols 1-5 (content cols 2-4 + margin)
    }

    #[test]
    fn test_trim_preserves_all_content() {
        let tiles = vec![
            vec!['#', '#', '#', '#', '#'],
            vec!['#', 'P', '@', '*', '#'],
            vec!['#', '=', '^', 'C', '#'],
            vec!['#', '#', '#', '#', '#'],
        ];

        let (trimmed, _, _) = trim_tilemap(&tiles, 1);

        // Verify all content tiles are preserved
        let flat: String = trimmed.iter().flat_map(|r| r.iter()).collect();
        assert!(flat.contains('P'));
        assert!(flat.contains('@'));
        assert!(flat.contains('*'));
        assert!(flat.contains('='));
        assert!(flat.contains('^'));
        assert!(flat.contains('C'));
    }
}
