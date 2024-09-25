fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build the Request
    let req = RequestBuilder::new()
        .url("https://example.com")
        .method("GET")
        .header("User-Agent", "rust-reqwest/0.11.8")
        .build()?;
    println!("{:#?}", req);

    // If you want to do something between building the request and sending it, you can do so like this:
    let mut req_builder = RequestBuilder::new();
    req_builder.url("https://example.com").method("GET");
    // ... do other stuff
    let req = req_builder
        .header("User-Agent", "rust-reqwest/0.11.8")
        .build()?;
    Ok(())
}

#[derive(Debug)]
pub struct Request {
    pub url: String,
    pub method: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

#[derive(Default)]
pub struct RequestBuilder {
    pub url: Option<String>,
    pub method: Option<String>,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

impl RequestBuilder {
    pub fn new() -> Self {
        RequestBuilder::default()
    }

    pub fn url(&mut self, url: impl Into<String>) -> &mut Self {
        self.url = Some(url.into());
        self
    }
    pub fn method(&mut self, method: impl Into<String>) -> &mut Self {
        self.method = Some(method.into());
        self
    }
    pub fn body(&mut self, body: impl Into<String>) -> &mut Self {
        self.body = Some(body.into());
        self
    }
    pub fn header(&mut self, name: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.headers.push((name.into(), value.into()));
        self
    }

    pub fn build(&self) -> Result<Request, Box<dyn std::error::Error>> {
        let url = self.url.clone().ok_or("URL is required")?;
        let method = self.method.clone().unwrap_or("GET".to_string());
        let headers = self.headers.clone();
        let body = self.body.clone();
        Ok(Request {
            url,
            method,
            headers,
            body,
        })
    }
}
