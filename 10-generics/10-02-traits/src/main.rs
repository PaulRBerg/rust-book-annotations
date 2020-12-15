#![allow(unused)]
use traits::{self, NewsArticleV2, SummaryV2, TweetV2};

fn create_tweet_v2() {
    let tweet = TweetV2 {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

fn create_news_article_v2() {
    let article = NewsArticleV2 {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
}

fn main() {
    create_tweet_v2();
    // create_news_article_v2();
}
