#![allow(unused)]
#[derive(Debug)]
struct Point {
    x: u8,
    y: u8,
}

impl Point {
    // associate/static function
    fn init() -> Self {
        Point { x: 0, y: 0 }
    }
    // methods
    fn sum(&self) -> u8 {
        self.x + self.y
    }
    fn update(&mut self, x: u8, y: u8) {
        self.x = x;
        self.y = y;
    }
}

fn main() {
    let mut  point = Point::init();
    println!("init {:?}", point);
    
    point.update(5, 10);
    let sum = point.sum();

    println!("updated {:?}", point);
    println!("sum {:?}", sum);

}
