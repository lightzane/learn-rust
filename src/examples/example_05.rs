struct ImportantExcerpt<'a> {
    part: &'a str,
    //     👆🏻 This annotation means an instance of `ImportantExcerpt`
    //        can’t outlive the reference it holds in its `part` field.
}

impl<'a> ImportantExcerpt<'a> {
    // lifetime is not required in this method because it does not return any reference
    fn level(&self) -> i32 {
        3
    }

    // lifetime is required but because of "elision rules", we can omit it in the signature of this method.
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
        /*
            There are two input lifetimes, so Rust applies the first lifetime elision rule
            and gives both &self and announcement their own lifetimes.
            Then, because one of the parameters is &self,
            the return type gets the lifetime of &self,
            and all lifetimes have been accounted for.
        */
    }
}

pub fn run() {
    println!("\x1b[36;7m Lifetimes - Generics in Structs, Methods \x1b[0m");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    // let end_novel = novel; // demo move ownership of novel to end_novel,
    //                       which will cause error because novel is used in i.part
    //                       But if we comment out the println! line below,
    //                       the code will compile successfully because novel
    //                       is not used after move.
    /*
        `novel` doesn’t go out of scope until after the `ImportantExcerpt` goes out of scope,
        so the reference in the `ImportantExcerpt` instance is valid.
    */
    println!("Excerpt: {}", i.part);

    // * Static Lifetime
    /*
        'static
            which denotes that the affected reference
            can live for the entire duration of the program

        All string literals have the 'static lifetime
    */
    let s: &'static str = "I have a static lifetime.";
    println!("{s}");

    /*
        You might see suggestions in error messages to use the 'static lifetime.
        But before specifying 'static as the lifetime for a reference,
        think about whether or not the reference you have
        actually lives the entire lifetime of your program,
        and whether you want it to.

        Most of the time, an error message suggesting the
        'static lifetime results from attempting to create a dangling reference
        or a mismatch of the available lifetimes.

        In such cases, the solution is to fix those problems,
        not to specify the 'static lifetime.
    */
    let longest =
        longest_with_an_announcement("longer", "short", "Starcraft is the best game ever!");
    println!("The longest string is: {longest}");

    let longest2 =
        longest_with_an_announcement_2("short", "longer", "Starcraft is the best game ever!");
    println!("The longest string is: {longest2}");
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: std::fmt::Display,
{
    println!("Announcement 1! {ann}");
    if x.len() > y.len() { x } else { y }
}

// The above function can be written like this (without the 'where' clause)
fn longest_with_an_announcement_2<'a, T: std::fmt::Display>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str {
    println!("Announcement 2! {ann}");
    if x.len() > y.len() { x } else { y }
}
