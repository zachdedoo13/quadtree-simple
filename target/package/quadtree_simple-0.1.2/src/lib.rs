// A point in 2D space with some data
#[derive(Clone, Debug)]
pub struct Point<T: Clone> {
    pub x: f32,
    pub y: f32,
    pub data: T,
}
impl<T: Clone> Point<T> {
    pub fn new(x: f32, y: f32, data: T) -> Self {
        Self { x, y, data }
    }
}


// A rectangle anchored on center x, y with width w and height h
#[derive(Clone)]
pub struct Qrect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}
impl Qrect {
    pub fn new(x:f32, y:f32, w:f32, h:f32) -> Self {
        Self { x, y, w, h }
    }

    pub fn range(x: f32, y: f32, range: f32) -> Self {
        Self { x, y, w: range, h: range }
    }

    pub fn corners(top_left: f32, bottom_right: f32) -> Self {
        let w = bottom_right - top_left;
        let h = bottom_right - top_left;
        let x = top_left + w / 2.;
        let y = top_left + h / 2.;
        Self { w, h, x, y }
    }

    pub fn screen_size(width: f32, height: f32) -> Self {
        Self { x: width / 2., y: height / 2., w: width / 2., h: height / 2. }
    }

    fn contains_point<T: Clone>(&self, p: &Point<T>) -> bool {
        return p.x >= self.x - self.w &&
            p.x <= self.x + self.w &&
            p.y >= self.y - self.h &&
            p.y <= self.y + self.h
    }

    fn intersects_rect(&self, range: &Qrect) -> bool {
        return !(range.x - range.w > self.x + self.w ||
                range.x + range.w < self.x - self.w ||
                range.y - range.h > self.y + self.h ||
                range.y + range.h < self.y - self.h)
    }
}



// A quadtree that can store points in 2D space
#[derive(Clone)]
pub struct Quadtree<T: Clone> {
    boundary: Qrect,
    capacity: usize,
    points: Vec<Point<T>>,
    divided: bool,

    top_left: Option<Box<Quadtree<T>>>,
    top_right: Option<Box<Quadtree<T>>>,
    bottom_left: Option<Box<Quadtree<T>>>,
    bottom_right: Option<Box<Quadtree<T>>>,
}
impl<T: Clone> Quadtree<T> {
    pub fn new(boundary: Qrect, capacity: usize) -> Self {
        Self {
            boundary,
            capacity,
            points: vec![],
            divided: false,

            top_left: None,
            top_right: None,
            bottom_left: None,
            bottom_right: None,
        }
    }

    // Insert a point into the quadtree at the first possible location (x, y)
    pub fn insert(&mut self, point: &Point<T>) -> bool {
        if !self.boundary.contains_point(&point) {
            return false
        }

        if self.points.len() < self.capacity {
            self.points.push(point.clone());
            return true
        } else {
            if !self.divided {
                self.subdivide();
            }

            if self.top_left.as_mut().unwrap().insert(point) { return true }
            if self.top_right.as_mut().unwrap().insert(point) { return true }
            if self.bottom_left.as_mut().unwrap().insert(point) { return true }
            if self.bottom_right.as_mut().unwrap().insert(point) { return true }

            return false
        }

    }

    fn subdivide(&mut self) {
        let x = self.boundary.x; let y = self.boundary.y;
        let w = self.boundary.w; let h = self.boundary.h;

        let tr = Qrect::new(x + w / 2., y - h / 2., w / 2., h / 2.);
        let tl = Qrect::new(x - w / 2., y - h / 2., w / 2., h / 2.);
        let br = Qrect::new(x + w / 2., y + h / 2., w / 2., h / 2.);
        let bl = Qrect::new(x - w / 2., y + h / 2., w / 2., h / 2.);

        self.top_left = Some(Box::new(Quadtree::new(tl, self.capacity)));
        self.top_right = Some(Box::new(Quadtree::new(tr, self.capacity)));
        self.bottom_left = Some(Box::new(Quadtree::new(bl, self.capacity)));
        self.bottom_right = Some(Box::new(Quadtree::new(br, self.capacity)));

        self.divided = true;
    }

    // Query the quadtree for points within a rectangle
    pub fn query_rect(&self, range: &Qrect) -> Vec<Point<T>> {
        let mut found = vec![];
        if !self.boundary.intersects_rect(range) {
            return found
        } else {
            for point in &self.points {
                if range.contains_point(point) {
                    found.push(point.clone());
                }
            }

            if self.divided {
                let top_left_points = self.top_left.as_ref().unwrap().query_rect(range);
                let top_right_points = self.top_right.as_ref().unwrap().query_rect(range);
                let bottom_left_points = self.bottom_left.as_ref().unwrap().query_rect(range);
                let bottom_right_points = self.bottom_right.as_ref().unwrap().query_rect(range);

                found.extend(top_left_points);
                found.extend(top_right_points);
                found.extend(bottom_left_points);
                found.extend(bottom_right_points);
            }
        }

        return found
    }

    // Query the quadtree for points within a circle
    pub fn query_circle(&self, x:f32, y:f32, range: f32) -> Vec<Point<T>> {
        // make a rect that fits around the range circle
        let rect = Qrect::new(x, y, range, range);
        // draw the circle and the rect

        let mut temp = self.query_rect(&rect);

        temp.retain(|point| {
            let dist_x = point.x - x;
            let dist_y = point.y - y;
            let dist = (dist_x * dist_x + dist_y * dist_y).sqrt();
            if dist < range {
                true
            } else {
                false
            }
        });

        temp
    }

    pub fn collect(&self) -> Vec<Point<T>> {
        self.query_rect(&self.boundary)
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works(){
        let size = 50.0;
        let mut qt = Quadtree::new(Qrect::new(size, size, size, size), 4);
        qt.insert(&Point::new(25., 25., 0));
    }

    #[test]
    fn insert_at_known_points() {
        let size = 50.0;
        let mut qt = Quadtree::new(Qrect::new(size, size, size, size), 4);
        qt.insert(&Point::new(25., 25., 0));
        qt.insert(&Point::new(25., 25., 1));
        qt.insert(&Point::new(25., 25., 2));
        qt.insert(&Point::new(25., 25., 3));
        // check
        let found = qt.query_rect(&Qrect::range(25., 25., 1.));
        assert_eq!(found.len(), 4);
        // check locations
        assert_eq!(found[0].data, 0);
        assert_eq!(found[1].data, 1);
        assert_eq!(found[2].data, 2);
        assert_eq!(found[3].data, 3);
    }

}
