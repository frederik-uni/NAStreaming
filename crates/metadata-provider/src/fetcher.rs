pub type Result<T> = std::result::Result<T, Error>;

use std::fmt;
use std::net::SocketAddr;
use std::time::Duration;

use ::reqwest::header;
use ::reqwest::header::HeaderName;
use ::reqwest::header::HeaderValue;
use ::reqwest::Body;
use ::reqwest::IntoUrl;
pub use ::reqwest::Method;
use ::reqwest::Request;
pub use ::reqwest::Url;
pub use ::reqwest::Version;
pub use bytes::Bytes;
use http::header::USER_AGENT;
pub use http::HeaderMap;
pub use http::StatusCode;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::Error;

pub mod reqwest {
    pub use reqwest::*;
}

#[derive(Clone, Default)]
pub struct Client {
    client: reqwest::Client,
    retries: u32,
}

impl Client {
    pub async fn retries(mut self, retries: u32) -> Self {
        self.retries = retries;
        self
    }
}

pub struct Response {
    bytes: Bytes,
    headers: HeaderMap,
    status: StatusCode,
    version: Version,
    url: Url,
    remote_addr: Option<SocketAddr>,
    extensions: http::Extensions,
}

impl Response {
    /// Get the `StatusCode` of this `Response`.
    ///
    /// # Examples
    ///
    /// Checking for general status class:
    ///
    /// ```rust
    /// # #[cfg(feature = "json")]
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let resp = reqwest::get("http://httpbin.org/get")?;
    /// if resp.status().is_success() {
    ///     println!("success!");
    /// } else if resp.status().is_server_error() {
    ///     println!("server error!");
    /// } else {
    ///     println!("Something else happened. Status: {:?}", resp.status());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Checking for specific status codes:
    ///
    /// ```rust
    /// use reqwest::Client;
    /// use reqwest::StatusCode;
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new();
    ///
    /// let resp = client.post("http://httpbin.org/post")
    ///     .body("possibly too large")
    ///     .send()?;
    ///
    /// match resp.status() {
    ///     StatusCode::OK => println!("success!"),
    ///     StatusCode::PAYLOAD_TOO_LARGE => {
    ///         println!("Request payload is too large!");
    ///     }
    ///     s => println!("Received response status: {s:?}"),
    /// };
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn status(&self) -> StatusCode {
        self.status
    }

    /// Get the `Headers` of this `Response`.
    ///
    /// # Example
    ///
    /// Saving an etag when caching a file:
    ///
    /// ```
    /// use reqwest::Client;
    /// use reqwest::header::ETAG;
    ///
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new();
    ///
    /// let mut resp = client.get("http://httpbin.org/cache").send()?;
    /// if resp.status().is_success() {
    ///     if let Some(etag) = resp.headers().get(ETAG) {
    ///         std::fs::write("etag", etag.as_bytes());
    ///     }
    ///     let mut file = std::fs::File::create("file")?;
    ///     resp.copy_to(&mut file)?;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// Get a mutable reference to the `Headers` of this `Response`.
    #[inline]
    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        &mut self.headers
    }

    /// Get the HTTP `Version` of this `Response`.
    #[inline]
    pub fn version(&self) -> Version {
        self.version
    }

    /// Get the final `Url` of this `Response`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let resp = reqwest::get("http://httpbin.org/redirect/1")?;
    /// assert_eq!(resp.url().as_str(), "http://httpbin.org/get");
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn url(&self) -> &Url {
        &self.url
    }

    /// Get the remote address used to get this `Response`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let resp = reqwest::get("http://httpbin.org/redirect/1")?;
    /// println!("httpbin.org address: {:?}", resp.remote_addr());
    /// # Ok(())
    /// # }
    /// ```
    pub fn remote_addr(&self) -> Option<SocketAddr> {
        self.remote_addr
    }

    /// Returns a reference to the associated extensions.
    pub fn extensions(&self) -> &http::Extensions {
        &self.extensions
    }

    /// Returns a mutable reference to the associated extensions.
    pub fn extensions_mut(&mut self) -> &mut http::Extensions {
        &mut self.extensions
    }

    /// Get the content-length of the response, if it is known.
    ///
    /// Reasons it may not be known:
    ///
    /// - The server didn't send a `content-length` header.
    /// - The response is gzipped and automatically decoded (thus changing
    ///   the actual decoded length).
    pub fn content_length(&self) -> usize {
        self.bytes.len()
    }

    /// Try and deserialize the response body as JSON using `serde`.
    ///
    /// # Optional
    ///
    /// This requires the optional `json` feature enabled.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate reqwest;
    /// # extern crate serde;
    /// #
    /// # use reqwest::Error;
    /// # use serde::Deserialize;
    /// #
    /// // This `derive` requires the `serde` dependency.
    /// #[derive(Deserialize)]
    /// struct Ip {
    ///     origin: String,
    /// }
    ///
    /// # fn run() -> Result<(), Error> {
    /// let json: Ip = reqwest::get("http://httpbin.org/ip")?.json()?;
    /// # Ok(())
    /// # }
    /// #
    /// # fn main() { }
    /// ```
    ///
    /// # Errors
    ///
    /// This method fails whenever the response body is not in JSON format,
    /// or it cannot be properly deserialized to target type `T`. For more
    /// details please see [`serde_json::from_reader`].
    ///
    /// [`serde_json::from_reader`]: https://docs.serde.rs/serde_json/fn.from_reader.html
    pub async fn json<T: DeserializeOwned>(self) -> Result<T> {
        Ok(serde_json::from_slice(&self.bytes.to_vec())?)
    }

    /// Get the full response body as `Bytes`.
    ///
    /// # Example
    ///
    /// ```
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let bytes = reqwest::get("http://httpbin.org/ip")?.bytes()?;
    ///
    /// println!("bytes: {bytes:?}");
    /// # Ok(())
    /// # }
    /// ```
    pub fn bytes(self) -> Bytes {
        self.bytes
    }

    /// Get the response text.
    ///
    /// This method decodes the response body with BOM sniffing
    /// and with malformed sequences replaced with the REPLACEMENT CHARACTER.
    /// Encoding is determined from the `charset` parameter of `Content-Type` header,
    /// and defaults to `utf-8` if not presented.
    ///
    /// # Note
    ///
    /// If the `charset` feature is disabled the method will only attempt to decode the
    /// response as UTF-8, regardless of the given `Content-Type`
    ///
    /// # Example
    ///
    /// ```rust
    /// # extern crate reqwest;
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let content = reqwest::get("http://httpbin.org/range/26")?.text()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn text(self) -> String {
        let text = String::from_utf8_lossy(&self.bytes);
        text.into_owned()
    }

    pub fn html(self) -> scraper::Html {
        scraper::Html::parse_document(self.text().as_str())
    }
}

pub struct RequestBuilder {
    req: reqwest::RequestBuilder,
    retries: u32,
}

impl RequestBuilder {
    pub fn retries(self) -> RequestBuilder {
        RequestBuilder {
            req: self.req,
            retries: self.retries,
        }
    }

    pub fn with_user_agent(self) -> RequestBuilder {
        RequestBuilder {
            req: self.req.header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36"),
            retries: self.retries,
        }
    }
    /// Add a `Header` to this Request.
    ///
    /// ```rust
    /// use reqwest::header::USER_AGENT;
    ///
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = reqwest::Client::new();
    /// let res = client.get("https://www.rust-lang.org")
    ///     .header(USER_AGENT, "foo")
    ///     .send()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn header<K, V>(self, key: K, value: V) -> RequestBuilder
    where
        HeaderName: TryFrom<K>,
        HeaderValue: TryFrom<V>,
        <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        <HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
    {
        RequestBuilder {
            req: self.req.header(key, value),
            retries: self.retries,
        }
    }

    /// Add a set of Headers to the existing ones on this Request.
    ///
    /// The headers will be merged in to any already set.
    ///
    /// ```rust
    /// use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, CONTENT_TYPE};
    /// # use std::fs;
    ///
    /// fn construct_headers() -> HeaderMap {
    ///     let mut headers = HeaderMap::new();
    ///     headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
    ///     headers.insert(CONTENT_TYPE, HeaderValue::from_static("image/png"));
    ///     headers
    /// }
    ///
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let file = fs::File::open("much_beauty.png")?;
    /// let client = reqwest::Client::new();
    /// let res = client.post("http://httpbin.org/post")
    ///     .headers(construct_headers())
    ///     .body(file)
    ///     .send()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn headers(self, headers: header::HeaderMap) -> RequestBuilder {
        RequestBuilder {
            req: self.req.headers(headers),
            retries: self.retries,
        }
    }

    /// Enable HTTP basic authentication.
    ///
    /// ```rust
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = reqwest::Client::new();
    /// let resp = client.delete("http://httpbin.org/delete")
    ///     .basic_auth("admin", Some("good password"))
    ///     .send()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn basic_auth<U, P>(self, username: U, password: Option<P>) -> RequestBuilder
    where
        U: fmt::Display,
        P: fmt::Display,
    {
        RequestBuilder {
            req: self.req.basic_auth(username, password),
            retries: self.retries,
        }
    }

    /// Enable HTTP bearer authentication.
    ///
    /// ```rust
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = reqwest::Client::new();
    /// let resp = client.delete("http://httpbin.org/delete")
    ///     .bearer_auth("token")
    ///     .send()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn bearer_auth<T>(self, token: T) -> RequestBuilder
    where
        T: fmt::Display,
    {
        RequestBuilder {
            req: self.req.bearer_auth(token),
            retries: self.retries,
        }
    }

    /// Set the request body.
    ///
    /// # Examples
    ///
    /// Using a string:
    ///
    /// ```rust
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = reqwest::Client::new();
    /// let res = client.post("http://httpbin.org/post")
    ///     .body("from a &str!")
    ///     .send()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using a `File`:
    ///
    /// ```rust
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let file = std::fs::File::open("from_a_file.txt")?;
    /// let client = reqwest::Client::new();
    /// let res = client.post("http://httpbin.org/post")
    ///     .body(file)
    ///     .send()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using arbitrary bytes:
    ///
    /// ```rust
    /// # use std::fs;
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// // from bytes!
    /// let bytes: Vec<u8> = vec![1, 10, 100];
    /// let client = reqwest::Client::new();
    /// let res = client.post("http://httpbin.org/post")
    ///     .body(bytes)
    ///     .send()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn body<T: Into<Body>>(self, body: T) -> RequestBuilder {
        RequestBuilder {
            req: self.req.body(body),
            retries: self.retries,
        }
    }

    /// Enables a request timeout.
    ///
    /// The timeout is applied from when the request starts connecting until the
    /// response body has finished. It affects only this request and overrides
    /// the timeout configured using `ClientBuilder::timeout()`.
    pub fn timeout(self, timeout: Duration) -> RequestBuilder {
        RequestBuilder {
            req: self.req.timeout(timeout),
            retries: self.retries,
        }
    }

    /// Modify the query string of the URL.
    ///
    /// Modifies the URL of this request, adding the parameters provided.
    /// This method appends and does not overwrite. This means that it can
    /// be called multiple times and that existing query parameters are not
    /// overwritten if the same key is used. The key will simply show up
    /// twice in the query string.
    /// Calling `.query(&[("foo", "a"), ("foo", "b")])` gives `"foo=a&foo=b"`.
    ///
    /// ```rust
    /// # use reqwest::Error;
    /// #
    /// # fn run() -> Result<(), Error> {
    /// let client = reqwest::Client::new();
    /// let res = client.get("http://httpbin.org")
    ///     .query(&[("lang", "rust")])
    ///     .send()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Note
    /// This method does not support serializing a single key-value
    /// pair. Instead of using `.query(("key", "val"))`, use a sequence, such
    /// as `.query(&[("key", "val")])`. It's also possible to serialize structs
    /// and maps into a key-value pair.
    ///
    /// # Errors
    /// This method will fail if the object you provide cannot be serialized
    /// into a query string.
    pub fn query<T: Serialize + ?Sized>(self, query: &T) -> RequestBuilder {
        RequestBuilder {
            req: self.req.query(query),
            retries: self.retries,
        }
    }

    /// Set HTTP version
    pub fn version(self, version: Version) -> RequestBuilder {
        RequestBuilder {
            req: self.req.version(version),
            retries: self.retries,
        }
    }

    /// Send a form body.
    ///
    /// Sets the body to the url encoded serialization of the passed value,
    /// and also sets the `Content-Type: application/x-www-form-urlencoded`
    /// header.
    ///
    /// ```rust
    /// # use reqwest::Error;
    /// # use std::collections::HashMap;
    /// #
    /// # fn run() -> Result<(), Error> {
    /// let mut params = HashMap::new();
    /// params.insert("lang", "rust");
    ///
    /// let client = reqwest::Client::new();
    /// let res = client.post("http://httpbin.org")
    ///     .form(&params)
    ///     .send()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This method fails if the passed value cannot be serialized into
    /// url encoded format
    pub fn form<T: Serialize + ?Sized>(self, form: &T) -> RequestBuilder {
        RequestBuilder {
            req: self.req.form(form),
            retries: self.retries,
        }
    }

    /// Send a JSON body.
    ///
    /// Sets the body to the JSON serialization of the passed value, and
    /// also sets the `Content-Type: application/json` header.
    ///
    /// # Optional
    ///
    /// This requires the optional `json` feature enabled.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use reqwest::Error;
    /// # use std::collections::HashMap;
    /// #
    /// # fn run() -> Result<(), Error> {
    /// let mut map = HashMap::new();
    /// map.insert("lang", "rust");
    ///
    /// let client = reqwest::Client::new();
    /// let res = client.post("http://httpbin.org")
    ///     .json(&map)
    ///     .send()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Serialization can fail if `T`'s implementation of `Serialize` decides to
    /// fail, or if `T` contains a map with non-string keys.
    pub fn json<T: Serialize + ?Sized>(self, json: &T) -> RequestBuilder {
        RequestBuilder {
            req: self.req.json(json),
            retries: self.retries,
        }
    }

    /// Build a `Request`, which can be inspected, modified and executed with
    /// `Client::execute()`.
    pub fn build(self) -> Result<Request> {
        Ok(self.req.build()?)
    }

    /// Build a `Request`, which can be inspected, modified and executed with
    /// `Client::execute()`.
    ///
    /// This is similar to [`RequestBuilder::build()`], but also returns the
    /// embedded `Client`.
    pub fn build_split(self) -> (Client, Result<Request>) {
        let (c, r) = self.req.build_split();
        (
            Client {
                client: c,
                retries: self.retries,
            },
            r.map_err(Error::Reqwest),
        )
    }

    /// Constructs the Request and sends it the target URL, returning a Response.
    ///
    /// # Errors
    ///
    /// This method fails if there was an error while sending request,
    /// redirect loop was detected or redirect limit was exhausted.
    pub async fn send(self) -> Result<Response> {
        let mut tries = 0;
        let mut req = self.req;
        let mut copy = req.try_clone();
        while tries <= self.retries {
            let resp = match req.send().await {
                Ok(v) => v,
                Err(e) => {
                    if tries >= self.retries || copy.is_none() {
                        return Err(e.into());
                    }
                    tries += 1;
                    req = copy.take().unwrap();
                    copy = req.try_clone();
                    continue;
                }
            };
            return Ok(Response {
                headers: resp.headers().clone(),
                status: resp.status(),
                version: resp.version(),
                url: resp.url().clone(),
                remote_addr: resp.remote_addr(),
                extensions: resp.extensions().clone(),
                bytes: match resp.bytes().await {
                    Ok(v) => v,
                    Err(e) => {
                        if tries >= self.retries || copy.is_none() {
                            return Err(e.into());
                        }
                        tries += 1;
                        req = copy.take().unwrap();
                        copy = req.try_clone();
                        continue;
                    }
                },
            });
        }
        unreachable!()
    }

    /// Attempts to clone the `RequestBuilder`.
    ///
    /// None is returned if a body is which can not be cloned. This can be because the body is a
    /// stream.
    ///
    /// # Examples
    ///
    /// With a static body
    ///
    /// ```rust
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = reqwest::Client::new();
    /// let builder = client.post("http://httpbin.org/post")
    ///     .body("from a &str!");
    /// let clone = builder.try_clone();
    /// assert!(clone.is_some());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Without a body
    ///
    /// ```rust
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = reqwest::Client::new();
    /// let builder = client.get("http://httpbin.org/get");
    /// let clone = builder.try_clone();
    /// assert!(clone.is_some());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// With a non-cloneable body
    ///
    /// ```rust
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = reqwest::Client::new();
    /// let builder = client.get("http://httpbin.org/get")
    ///     .body(reqwest::Body::new(std::io::empty()));
    /// let clone = builder.try_clone();
    /// assert!(clone.is_none());
    /// # Ok(())
    /// # }
    /// ```
    pub fn try_clone(&self) -> Option<RequestBuilder> {
        self.req.try_clone().map(|req| RequestBuilder {
            req,
            retries: self.retries,
        })
    }
}

impl Client {
    /// Convenience method to make a `GET` request to a URL.
    ///
    /// # Errors
    ///
    /// This method fails whenever supplied `Url` cannot be parsed.
    pub fn get<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        self.request(Method::GET, url)
    }

    /// Convenience method to make a `POST` request to a URL.
    ///
    /// # Errors
    ///
    /// This method fails whenever supplied `Url` cannot be parsed.
    pub fn post<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        self.request(Method::POST, url)
    }

    /// Convenience method to make a `PUT` request to a URL.
    ///
    /// # Errors
    ///
    /// This method fails whenever supplied `Url` cannot be parsed.
    pub fn put<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        self.request(Method::PUT, url)
    }

    /// Convenience method to make a `PATCH` request to a URL.
    ///
    /// # Errors
    ///
    /// This method fails whenever supplied `Url` cannot be parsed.
    pub fn patch<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        self.request(Method::PATCH, url)
    }

    /// Convenience method to make a `DELETE` request to a URL.
    ///
    /// # Errors
    ///
    /// This method fails whenever supplied `Url` cannot be parsed.
    pub fn delete<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        self.request(Method::DELETE, url)
    }

    /// Convenience method to make a `HEAD` request to a URL.
    ///
    /// # Errors
    ///
    /// This method fails whenever supplied `Url` cannot be parsed.
    pub fn head<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        self.request(Method::HEAD, url)
    }

    /// Start building a `Request` with the `Method` and `Url`.
    ///
    /// Returns a `RequestBuilder`, which will allow setting headers and
    /// request body before sending.
    ///
    /// # Errors
    ///
    /// This method fails whenever supplied `Url` cannot be parsed.
    pub fn request<U: IntoUrl>(&self, method: Method, url: U) -> RequestBuilder {
        RequestBuilder {
            req: self.client.request(method, url),
            retries: self.retries,
        }
    }
}
