// fn main() {
//     let  s = "123123";

//     let s = "13";
//     println!("s is : {}",s);

//     let mut a = String::from("hello");

//     a.push_str(", world!");

//     println!("{}",a);

//     let oo = String::from("aaa");

//     let pp = oo.clone();  //克隆，深度复制变量

//     println!("pp :{}",oo);

// }

//所有权丢掉了。
// fn main(){

//     let a = String::from("a");

//     take(a);

//     // println!("after a is : {}",a);   //value borrowed here after move

//     let b = 1;

//     copy(b);

//     println!("afer b is {}",b);

// }

// fn take(b :String){
//     println!("b is :{}",b)
// }

// fn copy(b : i32){
//     println!("b is :{}",b)
// }

// //利用元组来传入多个值
// fn main(){

//     let s1 = String::from("asdccc");

//     let (s2,len) = lenss(s1);

//     println!("s2: {},len: {},s1:{}",s2,len,s1);

// }

// fn lenss(s :String)->(String,usize){
//     let lenth = s.len();
//     (s,lenth)
// }

//传入引用
// fn main(){

//     let s1 = String::from("asdccc");

//     let len = lenss(&s1);

//     println!("len: {},s1:{}",len,s1);

// }

// fn lenss(s :&String)->usize{
//     s.len()

// }
// //引用不可改变，可传入可变引用
// fn main() {
//     let mut s1 = String::from("asdccc");

//     lenss(&mut s1);

//     println!("s1:{}", s1);
// }

// fn lenss(s: &mut String) {
//     s.push_str("aaaa");
// }

//可变与不可变引用
// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s; // 没问题
//     let r2 = &s; // 没问题
//     println!("{}, {}", r1, r2 );

//     let r3 = &mut s; // 大问题

//     println!(" and {}", r3);
// }

// //垂悬引用 编译器检查报错
// fn main() {
//    let s =  dange();
// }

// fn dange()->&String{
//     let s = String::from("abc");

//     &s
// }

//slice


// fn main(){
//     let s = String::from("asd asddd");
//     let i = first_word(&s);
//     println!("i: {}",i);
// }

// fn first_word(s :&String)->usize{
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

// fn main(){
//     let s = String::from("asd asddd");
//     let s1 = &s[0..2];
//     println!("s:{},s1:{}",s,s1);
// }


fn main(){
    let s = String::from("asdaaaaddd asddd");
    let i = first_word(&s);
    println!("i: {}",i);
}

fn first_word(s :&String)->&str{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}