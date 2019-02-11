use std::vec::Vec;

/// Represents a point of interest within a plane.
pub struct Location {
    pub x: usize,
    pub y: usize,
    pub reach: i32,
    pub expansions: Vec<usize>,
}

/// Represents a point in a plane, and contains a reference to the nearest Location in the plane,
/// along with the distance to that 
pub struct Point<'a> {
    x: usize,
    y: usize,
    location:  Option<&'a mut Location>,
    distance: i32,
}

pub struct Plane<'a> {
    minX: usize,
    minY: usize,
    maxX: usize,
    maxY: usize,
    points: Vec<Point<'a>>,
    locations: Vec<Location>,
}

impl<'a> Plane<'a> {
    pub fn new() -> Plane<'a> {
        return Plane {
            minX: 0,
            minY: 0,
            maxX: 0,
            maxY: 0,
            points: Vec::new(),
            locations: Vec::new(),
        };
    }

    pub fn add_location(&'a mut self, loc: Location) {
        if self.locations.is_empty() {
            // This is the first location added
            self.minX = loc.x;
            self.maxX = loc.x;
            self.minY = loc.y;
            self.maxY = loc.y;
            self.locations.push(loc);
            return;
        }

        // Check our bounds, and update accordingly
        if loc.x < self.minX {
            for y in self.minY..self.maxY+1 {
                let index = self.coords_to_index(loc.x, y);
                let mut point = &self.points[index];
                if let Some(ref mut location) = point.location {
                    location.expansions.push(index);
                }
            }
        }
        return;
    }

    pub fn get_largest_reach(&self) -> Option<&Location> {
        let mut optional = None;
        let mut largest_reach = -1;
        for loc in self.locations.iter() {
            if loc.reach > largest_reach {
                largest_reach = loc.reach;
                optional = Some(loc);
            }
        }
        return optional;
    }

    fn coords_to_index(&self, x: usize, y: usize) -> usize {
        let new_x = x - self.minX;
        let new_y = y - self.minY;
        let width = self.maxX - self.minX + 1;
        return (new_y * width) + new_x;
    }
}