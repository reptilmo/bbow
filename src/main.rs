use bbow::Bbow;

fn main() {
    let s = "c🤚an't hasn't 朋友 朋友 朋友 Բարեւ ընկերներ!";
    let bbow = Bbow::new().extend_from_text(s);
    for w in bbow.words() {
        println!("{}", w);
    }
}
