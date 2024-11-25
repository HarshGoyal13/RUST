trait Summary {
    fn summarize_author(&self) -> &str;  // THIS DEFAULT TRAIT, 
    fn summarize(&self)-> String {
        format!("Read More from.... : {}", self.summarize_author() )
    }
}


struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for Tweet {
    fn summarize(&self)-> String {
        format!("Tweet by {} : {}", self.username, self.content)
    }
    fn summarize_author(&self)-> &str {
        self.username.as_str()
    }
}
impl Summary for NewsArticle {
    fn summarize(&self)-> String {
        format!("News from {} : {}", self.location, self.headline) // WHEN I ACCESS THIS, THEN THAT NOT CALL DEFAULT FUNCTION
    }
    fn summarize_author(&self)-> &str {
         self.author.as_str()
    }
}


 struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}


fn news_aggregator(source : &impl Summary){
    println!("{}", source.summarize());
}

fn main() {

    let tweet = Tweet{
    username: String::from("Harsh"),
    content: String::from("Rust Book chapter 10"),
    reply: false,
    retweet: false,
    };

    let news = NewsArticle{
        headline: String::from("Finally we Reached on RUST topic Traits"),
        location: String::from("Chapter 10"),
        author: String::from("Harsh"),
        content: String::from("Secret"),
    };

    news_aggregator(&tweet);
    news_aggregator(&news);

}
