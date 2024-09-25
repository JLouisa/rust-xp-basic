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

#[derive(Default, Clone, Debug)]
pub struct NoMethod;
#[derive(Default, Clone, Debug)]
pub struct Method(String);
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
pub struct RequestBuilder<U, M> {
    url: U,
    method: M,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

impl RequestBuilder<NoUrl, NoMethod> {
    pub fn new() -> Self {
        RequestBuilder::default()
    }
}

impl<U, M> RequestBuilder<U, M> {
    pub fn url(mut self, url: impl Into<String>) -> RequestBuilder<Url, M> {
        RequestBuilder {
            url: Url(url.into()),
            method: self.method,
            headers: self.headers,
            body: self.body,
        }
    }
    pub fn method(mut self, method: impl Into<String>) -> RequestBuilder<U, Method> {
        RequestBuilder {
            url: self.url,
            method: Method(method.into()),
            headers: self.headers,
            body: self.body,
        }
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

impl RequestBuilder<Url, Method> {
    pub fn build(self) -> Result<Request, Box<dyn std::error::Error>> {
        let url = self.url;
        let method = self.method;
        let headers = self.headers;
        let body = self.body;
        Ok(Request {
            url: url.0,
            method: method.0,
            headers,
            body,
        })
    }
}
