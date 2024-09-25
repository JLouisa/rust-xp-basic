fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build the Request
    let req = RequestBuilder::new()
        .url("https://example.com")
        .method("GET")
        .header("User-Agent", "USER_uuid.exp-sign")
        .build()?;
    println!("{:#?}", req);

    Ok(())
}

// region: --- States
#[derive(Default, Clone, Debug)]
pub struct NoUrl;
#[derive(Default, Clone, Debug)]
pub struct Url(String);
// endregion: --- States

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
pub struct RequestBuilder<U> {
    url: U,
    method: Option<String>,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

impl RequestBuilder<NoUrl> {
    pub fn new() -> Self {
        RequestBuilder::default()
    }
}
impl<U> RequestBuilder<U> {
    pub fn url(mut self, url: impl Into<String>) -> RequestBuilder<Url> {
        RequestBuilder {
            url: Url(url.into()),
            method: self.method,
            headers: self.headers,
            body: self.body,
        }
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
}

impl RequestBuilder<Url> {
    pub fn build(self) -> Result<Request, Box<dyn std::error::Error>> {
        let url = self.url;
        let method = self.method.unwrap_or("GET".to_string());
        let headers = self.headers;
        let body = self.body;
        Ok(Request {
            url: url.0,
            method,
            headers,
            body,
        })
    }
}
