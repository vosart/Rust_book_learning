struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple struct кортежные структуры
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like struct единично-подобные структуры
struct AlwaysEqual;

fn main() {
    let user1 = User {
        active: true,
        username: String::from("vosart"),
        email: String::from("metravs@gmail.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // синтаксис с меньшим количеством кода .. значит остальное как в user2
    let user3 = User {
        email: String::from("envkt@example.com"),
        ..user2
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{}", black.0);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}