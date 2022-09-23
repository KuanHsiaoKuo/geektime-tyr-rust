use macros::query;

fn main() {
    // query!(SELECT * FROM users WHERE age > 10);
    query!(SELECT * FROM users u JOIN (SELECT * from profiles p) WHERE u.id = p.id);
    hello()
}
