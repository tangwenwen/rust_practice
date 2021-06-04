fn main() {
    let num :i32 = 10;
    if num<10{
        println!("aaa");
    }else if num>=10{
        println!("bbb");
    }

    let a = 3<5;

    let b = if a{
        let c = 2;
        c+1
    }else {
        let c = 2;
        c+2
    };
    println!("b is : {}",b);
}
