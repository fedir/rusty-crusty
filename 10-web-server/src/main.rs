use warp::Filter;

#[tokio::main]
async fn main() {
    // Define a route: GET /hello/{name}
    // warp::path! is a macro to easily define path segments.
    // The following route captures a String from the path and maps it to a greeting.
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    println!("Starting server at http://127.0.0.1:3030");
    println!("Try visiting: http://127.0.0.1:3030/hello/world");

    // Start the server on the specified address and port.
    // The .await is needed because starting the server is an asynchronous operation.
    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hello_route() {
        let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

        let resp = warp::test::request()
            .method("GET")
            .path("/hello/rust")
            .reply(&hello)
            .await;

        assert_eq!(resp.status(), 200);
        assert_eq!(resp.body(), "Hello, rust!");
    }
}
