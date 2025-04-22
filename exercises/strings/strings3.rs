// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.


fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    input.trim().to_string()
    /* 作用​​：去除字符串两端的空白字符（包括空格、制表符、换行符等 Unicode 空白）
​       ​返回值​​：&str（原字符串的切片视图）
​       ​特点​​：不修改原字符串，返回新切片 */
}
fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    let mut x = input.to_string();
    x.push_str(" world!");
    x
    /* ​​作用​​：向 String 追加字符串内容
​       ​所属类型​​：String 的实例方法
​       ​特点​​：原地修改字符串，需可变引用 */
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    input.replace("cars","balloons")
    /* ​​作用​​：替换字符串中的子串，返回新的 String
       ​参数​​：replace(旧子串, 新子串)
       ​特点​​：不修改原字符串，返回新字符串 */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
