/// A trait defining a common behavior for summarizing content.
pub trait Summary {
    /// Returns a summary string. Each implementing type must provide its own logic.
    fn summarize(&self) -> String;
}

/// A news article struct with several metadata fields.
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

/// Implement the Summary trait for NewsArticle.
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

/// A tweet struct representing a short status update.
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

/// Implement the Summary trait for Tweet.
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

/// A generic function that works with any type T that implements the Summary trait.
/// This is called "Trait Bound" syntax.
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new article: {}", article.summarize());
    println!("1 new tweet: {}", tweet.summarize());

    notify(&article);
    notify(&tweet);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article_summary() {
        let article = NewsArticle {
            headline: String::from("Rust is awesome"),
            location: String::from("Global"),
            author: String::from("Dev"),
            content: String::from("..."),
        };
        assert_eq!(article.summarize(), "Rust is awesome, by Dev (Global)");
    }

    #[test]
    fn test_tweet_summary() {
        let tweet = Tweet {
            username: String::from("rustlang"),
            content: String::from("Rust 1.75 is out!"),
            reply: false,
            retweet: false,
        };
        assert_eq!(tweet.summarize(), "rustlang: Rust 1.75 is out!");
    }
}
