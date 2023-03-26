// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

// I AM DONE

fn main() {
    let vec0 = Vec::new();

    // 涉及到所有权的转移，我们并不想将vec0的所有权转交给vec1，因此需要借用
    let mut vec1 = fill_vec(&vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// 因为传参是借用，因此标签也需要引用
fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    // 但是我们发现，只是借用还是不行的，如果单纯的借用vec0，那么此处vec的类型是&Vec，这样就会导致返回值类型不一致
    // 如果修改返回值类型为&Vec，就会导致后续的&mut，而后继续更改最终是悬垂引用，因此，干脆直接clone出一个变量转交，使其类型为Vec
    let mut vec = vec.clone();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
