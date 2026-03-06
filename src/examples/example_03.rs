use learn_rust::{NewsArticle, Pair, SocialPost, Summary}; // See `src/lib.rs`
use std::fmt::{Debug, Display}; // For `some_function` examples

pub fn run() {
    println!("\x1b[36;7m Traits - Example 1 \x1b[0m");

    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new post: {}", post.summarize());
    notify_1(&post);
    notify_2(&post);

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("1 new article: {}", article.summarize());
    notify_1(&article);
    notify_2(&article);

    let summarizable = returns_summarizable();
    println!("Summarizable: {}", summarizable.summarize());

    let pair1 = Pair::new(1, 2);
    pair1.cmd_display();

    let pair2 = Pair::new(8, 1);
    pair2.cmd_display();
}

// * [Example 1] Using traits as parameters (syntax sugar of Example 2)
pub fn notify_1(item: &impl Summary) {
    println!("Breaking news 1! {}", item.summarize());
}

// * [Example 2] Using traits as parameters (without syntax sugar but more verbose)
pub fn notify_2<T: Summary>(item: &T) {
    println!("Breaking news 2! {}", item.summarize());
}

// ** More examples **
// pub fn notify(item1: &impl Summary, item2: &impl Summary)
// pub fn notify<T: Summary>(item1: &T, item2: &T)

// ** Multiple trait bounds ** (using `+` syntax)
// pub fn notify(item: &(impl Summary + Display))
// pub fn notify<T: Summary + Display>(item: &T)

#[allow(unused)]
// * Clearer trait bounds with `where` clauses
// Instead of writing this:
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    0
}

#[allow(unused)]
// (Less cluttered) We can use `where` clauses like this:
fn some_function_2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

// * Returning types that implement traits
fn returns_summarizable() -> impl Summary {
    // ! Important: The return type must always be the same,
    // * so we can't return different types that implement the same trait
    SocialPost {
        username: String::from("starcraft_broodwar"),
        content: String::from("The Zerg race can rapidly reproduce and adapt to any environment."),
        reply: false,
        retweet: false,
    }

    // For example adding "if/else" branch and returns the NewsArticle. ❌
}
