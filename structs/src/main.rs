// #[derive(Debug)]
// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn main(){
//     let user1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("someusername123"),
//         active: true,
//         sign_in_count: 1,
//     };

//     let user2 = User{
//         email:String::from("abcc"),
//         username:String::from("ccc"),
//         ..user1
//     };
//     println!("is:{:#?}",user2);
//     let p1 = Point(2,3,4);
//     println!("is:{}",p1.2);
// }

// fn new_user(username:String,email :String)->User{
//     User{
//         username,
//         email,
//         active:true,
//         sign_in_count:1,
//     }
// }

// //声明元组结构体
// struct Point(i32,i32,i32);

//结构体方法
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
