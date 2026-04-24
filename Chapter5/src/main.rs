struct Point{
    x : f32,
    y : f32,
}

fn build_point(val_x : f32, val_y : f32) -> Point {
    Point {
        x : val_x,
        y : val_y,
    }
}

fn main() {
    //Chapter 5:
    let point1 = build_point(0.0, 0.0);
    
    println!("Point1 : {}, {}", point1.x, point1.y);

    let point2 = Point {
        x : 1.0,
        ..point1
    };
    println!("Point2 : {}, {}", point2.x, point2.y);


}
