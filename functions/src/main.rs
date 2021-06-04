fn main() {
    println!("Hello, world!");
    another_function(5, 6);

    println!("emmm: {}", {
        let x = 4;
        x + 5
    });

    println!("emmm:{}", plus_one( 3 ));
}

fn another_function(x: i32, y: i32) {
    println!("value x : {}, value y :{}", x, y);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
