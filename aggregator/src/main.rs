use aggregator::{self, Summary, Tweet};


fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
aggregator::notify(&tweet);
 //   println!("1 new tweet: {}", tweet.summarize());
}