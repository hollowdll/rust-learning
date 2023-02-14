// Different states of a blog post
// Try changing the order of post's methods

use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("Today was Sunday");
    println!("Content after modifying draft: {}", post.content());
    assert_eq!("", post.content());

    post.request_review();
    println!("Content after requesting review: {}", post.content());
    assert_eq!("", post.content());

    post.approve();
    println!("Content after trying to approve: {}", post.content());
    assert_eq!("Today was Sunday", post.content());
}
