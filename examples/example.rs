use devout::{log, Tag};

const INFO: Tag = Tag::new("Info").show(true);

fn main() {
    log!(INFO, "Result: {}", 4.4);
}
