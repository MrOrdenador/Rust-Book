#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

fn main(){
    let rectangle = Rectangle{
        width: 30,
        height: 50,
    }
    println!("The area of the rectangle is {area(rectangle)} pixels")
    println!("Rectangle 1 is {rectangle:#?}")
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}