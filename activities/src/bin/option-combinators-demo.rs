fn main() {
    let a: Option<i32> = Some(1);
    let a_is_some = a.is_some();
    let a_is_none = a.is_none();
    let a_mapped = a.map(|num| num + 1);
    let a_filtered = a.filter(|num| num == &1); // & since it's browed
    let a_or_else = a.or_else(|| Some(5));
}
