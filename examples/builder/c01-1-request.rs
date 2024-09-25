fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build the Request
    let req = RequestBuilder::new()
        .url("https://example.com")
        .method("GET")
        .header("User-Agent", "USER_uuid.exp-sign")
        .build()?;
    println!("{:#?}", req);

    // If you want to do something between building the request and sending it, you can do so like this:
    let req_builder = RequestBuilder::new()
        .url("https://example.com")
        .method("GET")
        .header("User-Agent", "rust-reqwest/0.11.8");
    // ... do other stuff
    let req = req_builder.clone().build()?;
    println!("{:#?}", req);

    let req2 = req_builder.header("User-Agent", "rust-reqwest").build()?;
    println!("{:#?}", req2);

    Ok(())
}

// No need to be publice
#[derive(Debug)]
pub struct Request {
    url: String,
    method: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

// No need to be publice
#[derive(Default, Clone)]
pub struct RequestBuilder {
    url: Option<String>,
    method: Option<String>,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

impl RequestBuilder {
    pub fn new() -> Self {
        RequestBuilder::default()
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url.insert(url.into());
        self
    }
    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method.insert(method.into());
        self
    }
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body.insert(body.into());
        self
    }
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }

    // Consume the data
    pub fn build(self) -> Result<Request, Box<dyn std::error::Error>> {
        let url = self.url.ok_or("URL is required")?;
        let method = self.method.unwrap_or("GET".to_string());
        let headers = self.headers;
        let body = self.body;
        Ok(Request {
            url,
            method,
            headers,
            body,
        })
    }
}
