use core::marker::PhantomData;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Added Sealed function
    let req = RequestBuilder::new()
        .url("https://example.com")
        .method("GET")
        .header("User-Agent", "USER_uuid.exp-sign")
        .sealed()
        .build()?;
    println!("{:#?}", req);

    let req_builder = RequestBuilder::new()
        .url("https://example.com")
        .method("GET")
        .header("User-Agent", "USER_uuid.exp-sign")
        .sealed();

    let req1 = req_builder.clone().build()?;
    println!("{:#?}", req1);

    let req2 = req_builder.build()?;
    println!("{:#?}", req2);

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

#[derive(Default, Clone, Debug)]
pub struct NotSealed;
#[derive(Default, Clone, Debug)]
pub struct Sealed;
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
pub struct RequestBuilder<U, M, S> {
    url: U,
    method: M,
    headers: Vec<(String, String)>,
    body: Option<String>,
    marker_phanthom: PhantomData<S>,
}

impl RequestBuilder<NoUrl, NoMethod, NotSealed> {
    pub fn new() -> Self {
        RequestBuilder::default()
    }
}

impl<U, M> RequestBuilder<U, M, NotSealed> {
    pub fn sealed(self) -> RequestBuilder<U, M, Sealed> {
        RequestBuilder {
            url: self.url,
            method: self.method,
            headers: self.headers,
            body: self.body,
            marker_phanthom: PhantomData,
        }
    }
}

impl<U, M> RequestBuilder<U, M, NotSealed> {
    pub fn url(self, url: impl Into<String>) -> RequestBuilder<Url, M, NotSealed> {
        RequestBuilder {
            url: Url(url.into()),
            method: self.method,
            headers: self.headers,
            body: self.body,
            marker_phanthom: PhantomData,
        }
    }
    pub fn method(self, method: impl Into<String>) -> RequestBuilder<U, Method, NotSealed> {
        RequestBuilder {
            url: self.url,
            method: Method(method.into()),
            headers: self.headers,
            body: self.body,
            marker_phanthom: PhantomData,
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

impl<S> RequestBuilder<Url, Method, S> {
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
