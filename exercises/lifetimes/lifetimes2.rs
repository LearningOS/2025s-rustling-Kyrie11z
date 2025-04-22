// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the annotated
// parameters and the return type, what do we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.



//生命周期必须标注的场景
/*1、函数返回引用且依赖多个输入参数	    fn foo<'a>(x: &'a i32, y: &i32) -> &'a i32
  2、结构体/枚举包含引用	          struct Foo<'a> { data: &'a str }
  3、trait 对象中的引用	             trait Bar { fn get(&self) -> &str; }

*/



fn longest(x: &str, y: &str) -> String {
    if x.len() > y.len() {
        x.to_owned()
    } else {
        y.to_owned()
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2= String::from("xyz");
        result = longest(string1.as_str(), string2.clone().as_str());
    };
    println!("The longest string is '{}'", result);
}
