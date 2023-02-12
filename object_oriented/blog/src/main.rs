// TODO
// Implementation does not work yet!

use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("Today was Sunday");
    assert_eq!("", post.content());

    /*
    post.request_review();
    assert_eq!("", post.content);

    post.approve();
    assert_eq!("Today was Sunday", post.content());
    */
}
