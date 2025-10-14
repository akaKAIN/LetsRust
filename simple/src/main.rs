fn main() {
    println!("Hello, world!");
    let r = square(4.0);
    println!("P: {} S: {}, D: {}", r.0, r.1, r.2);
}

/*
Квадрат.
Написать функцию square, принимающую 1 аргумент — сторону квадрата,
и возвращающую 3 значения (с помощью кортежа): периметр квадрата,
площадь квадрата и диагональ квадрата.
 */
fn square(side_size: f32) -> (f32, f32, f32) {
    let long_d = side_size * 2.0_f32.sqrt();
    let short_d = (long_d * 100.0_f32).round() / 100.0;
    (side_size * 4.0, side_size * side_size, short_d)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_is_correct() {
        assert_eq!(square(2.0), (8.0, 4.0, 2.83));
        assert_eq!(square(4.0), (16.0, 16.0, 5.66))
    }

}