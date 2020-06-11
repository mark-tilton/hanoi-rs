fn hanoi(count: i32, current: i32, target: i32, other: i32) {
    if count == 0 {
        return;
    }
    hanoi(count-1, current, other, target);
    println!("{} -> {}", current, target);
    hanoi(count-1, other, target, current);
}

fn main() {
    hanoi(6, 1, 3, 2);
}
