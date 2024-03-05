struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String:: from("someone@example.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
            ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;

    //STARTING 5.2
    let width1 = 30;
    let height1 = 50;
    let rect1 = (30, 50);
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.", area(width1, height1));
    println!("The area of the rectangle is {} square pixels.", area1(rect1));
    println!("The area of the rectangle is {} square pixels.", area2(&rect2));
    println!("rect2 is {:?}.", rect2);

    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect3);

    println!("The area of the rectangle is {} square pixels.", rect2.area());
    if rect2.width() {
        println!("The Rectangle has a nonzero width; it is {}.", rect2.width);
    }

    let rect4 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect5 = Rectangle {
        width: 60,
        height: 45
    };

    println!("Can rect2 hold rect4? {}", rect2.can_hold(&rect4));
    println!("Can rect2 hold rect5? {}", rect2.can_hold(&rect5));





}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
