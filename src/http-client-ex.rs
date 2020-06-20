use error_chain::error_chain;
use serde::Deserialize;
use std::io::Read;

#[derive(Deserialize, Debug)]
struct Post {
    #[serde(rename = "userId")]
    user_id: u32,
    id: u32,
    title: String,
    body: String,
}

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main() -> Result<()> {
    let res = reqwest::blocking::get("https://jsonplaceholder.typicode.com/posts")?;
    // let mut body = String::new();
    // res.read_to_string(&mut body)?;

    // println!("Status: {}", res.status());
    // println!("Headers:\n{:#?}", res.headers());
    // println!("Body:\n{}", body);

    let posts: Vec<Post> = res.json()?;
    for post in &posts {
        println!(
            "id: {}, userId: {}, title: {}",
            post.id, post.user_id, post.title
        );
    }

    Ok(())
}
