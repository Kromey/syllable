fn main() {
    let generator = syllable::Syllable::default();

    for _ in 0..30 {
        println!("{}", generator.gen_name(3));
    }
}
