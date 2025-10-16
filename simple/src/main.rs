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

/*
Написать функцию season, принимающую 1 аргумент — номер
месяца (от 1 до 12), и возвращающую время года, которому
этот месяц принадлежит (зима, весна, лето или осень).
 */
fn season(month: u8) -> String {
    let winter: &str = "зима";
    let spring = "весна";
    let summer = "лето";
    let autumn = "осень";

    if month == 12 || (month > 0 && month <= 2) {
        return winter.to_string();
    } else if month > 2 && month <= 5 {
        return spring.to_string();
    } else if month > 5 && month <= 8 {
        return summer.to_string();
    } else if month > 8 && month <= 11 {
        return autumn.to_string();
    };

    return autumn.to_string();
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_is_correct() {
        assert_eq!(square(2.0), (8.0, 4.0, 2.83));
        assert_eq!(square(4.0), (16.0, 16.0, 5.66))
    }

    #[test]
    fn season_is_winter() {
        assert_eq!(season(1), "зима".to_string());
        assert_eq!(season(2), "зима".to_string());
        assert_eq!(season(12), "зима".to_string());
    }

    #[test]
    fn season_is_spring() {
        assert_eq!(season(3), "весна".to_string());
        assert_eq!(season(4), "весна".to_string());
        assert_eq!(season(5), "весна".to_string());
    }

    #[test]
    fn season_is_summer() {
        assert_eq!(season(6), "лето".to_string());
        assert_eq!(season(7), "лето".to_string());
        assert_eq!(season(8), "лето".to_string());
    }

    #[test]
    fn season_is_autumn() {
        assert_eq!(season(9), "осень".to_string());
        assert_eq!(season(10), "осень".to_string());
        assert_eq!(season(11), "осень".to_string());
    }

}