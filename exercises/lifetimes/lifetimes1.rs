// lifetimes1.rs
//
// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk of
// going out of scope before it is used. Remember, references are borrows and do
// not own their own data. What if their owner goes out of scope?
//
// Execute `rustlings hint lifetimes1` or use the `hint` watch subcommand for a
// hint.



//生命周期必须标注的场景
/*1、函数返回引用且依赖多个输入参数	    fn foo<'a>(x: &'a i32, y: &i32) -> &'a i32
  2、结构体/枚举包含引用	          struct Foo<'a> { data: &'a str }
  3、trait 对象中的引用	             trait Bar { fn get(&self) -> &str; }

*/

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
}
