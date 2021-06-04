fn main() {
    let mut counter = 0;
    let b = loop {
        counter += 1;
        if counter == 5 {
            break counter * 2;
        }
    };
    println!("b is : {}", b);

    let a = [10, 20, 30, 40, 50];
    for num in a.iter(){
        println!("num is :{}",num);
    }

    for a in (1..5).rev(){
        println!("a is :{}",a);
    }
}
