use std::str::FromStr;
use warp::{filters::BoxedFilter, http::Uri, path::FullPath, redirect, Filter, Reply};

#[tokio::main]
async fn main() {
    let current_dir = std::env::current_dir().expect("failed to read current directory");

    let www_dir = current_dir.join("www");

    let routes = root_redirect().or(warp::fs::dir(www_dir));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn root_redirect() -> BoxedFilter<(impl Reply,)> {
    warp::path::full()
        .and_then(move |path: FullPath| async move {
            let path = path.as_str();

            // do not redirect if the path ends in a trailing slash
            // or contains a period (indicating a specific file, e.g. style.css)
            if path.ends_with("/") || path.contains(".") {
                return Err(warp::reject());
            }

            Ok(redirect::redirect(
                Uri::from_str(&[path, "/"].concat()).unwrap(),
            ))
        })
        .boxed()
}