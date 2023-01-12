use rusty_url::uuid;

fn main() {
    for _ in 0..7 {
        println!("{}", uuid::generate_uuid());
    }
}
