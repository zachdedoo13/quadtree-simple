
# Quadtree Library

This is a Rust library for implementing a Quadtree data structure, which is useful for efficiently storing and querying spatial data in two dimensions.

## Features

- Store points with associated data in a 2D space
- Query points within a rectangular region or circular area
- Subdivision of space based on a configurable capacity
- Efficient insertion and query operations

## Usage

```rust
use quadtree::{Point, Qrect, Quadtree};

// Create a new quadtree with a bounding rectangle and capacity
// rects are (w, y, w, h) with x and y being anchored in the center
let size = 50.0;
let mut qt = Quadtree::new(Qrect::new(size, size, size, size), 4);

// Insert points with associated data
qt.insert(&Point::new(25., 25., 0));
qt.insert(&Point::new(30., 20., 1));

// Query points within a rectangular region
let rect = Qrect::new(20., 20., 10., 10.);
let points_in_rect = qt.query_rect(&rect);

// Query points within a circular area
let circle_points = qt.query_circle(25., 25., 5.);