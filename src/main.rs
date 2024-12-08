fn main() {
    let s1 = String::from("hellowrold");

    let s2 = calculate_length(&s1);

    println!("---------the value of s1={s1}----------");
    //這裏calculate_length(some:&String)的some只是引用了s2的值，並未取得所有權，因此s2自己的堆的數據還在
    println!("---------the value of s2={s2}----------");

}

fn calculate_length(some:&String)->usize{
    return some.len()
}
