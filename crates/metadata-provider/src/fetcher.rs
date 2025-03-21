pub type Result<T> = std::result::Result<T, Error>;
use std::fmt;
use std::net::SocketAddr;
use std::time::Duration;

use ::reqwest::blocking::Body;
use ::reqwest::blocking::Request;
use ::reqwest::header;
use ::reqwest::header::HeaderName;
use ::reqwest::header::HeaderValue;
use ::reqwest::IntoUrl;
pub use ::reqwest::Method;
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
pub struct Client(reqwest::blocking::Client);

pub struct Response(reqwest::blocking::Response);

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
    /// let resp = reqwest::blocking::get("http://httpbin.org/get")?;
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
    /// use reqwest::blocking::Client;
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
        self.0.status()
    }

    /// Get the `Headers` of this `Response`.
    ///
    /// # Example
    ///
    /// Saving an etag when caching a file:
    ///
    /// ```
    /// use reqwest::blocking::Client;
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
        self.0.headers()
    }

    /// Get a mutable reference to the `Headers` of this `Response`.
    #[inline]
    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        self.0.headers_mut()
    }

    /// Get the HTTP `Version` of this `Response`.
    #[inline]
    pub fn version(&self) -> Version {
        self.0.version()
    }

    /// Get the final `Url` of this `Response`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let resp = reqwest::blocking::get("http://httpbin.org/redirect/1")?;
    /// assert_eq!(resp.url().as_str(), "http://httpbin.org/get");
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn url(&self) -> &Url {
        self.0.url()
    }

    /// Get the remote address used to get this `Response`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let resp = reqwest::blocking::get("http://httpbin.org/redirect/1")?;
    /// println!("httpbin.org address: {:?}", resp.remote_addr());
    /// # Ok(())
    /// # }
    /// ```
    pub fn remote_addr(&self) -> Option<SocketAddr> {
        self.0.remote_addr()
    }

    /// Returns a reference to the associated extensions.
    pub fn extensions(&self) -> &http::Extensions {
        self.0.extensions()
    }

    /// Returns a mutable reference to the associated extensions.
    pub fn extensions_mut(&mut self) -> &mut http::Extensions {
        self.0.extensions_mut()
    }

    /// Get the content-length of the response, if it is known.
    ///
    /// Reasons it may not be known:
    ///
    /// - The server didn't send a `content-length` header.
    /// - The response is gzipped and automatically decoded (thus changing
    ///   the actual decoded length).
    pub fn content_length(&self) -> Option<u64> {
        self.0.content_length()
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
    /// let json: Ip = reqwest::blocking::get("http://httpbin.org/ip")?.json()?;
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
    pub fn json<T: DeserializeOwned>(self) -> Result<T> {
        Ok(self.0.json()?)
    }

    /// Get the full response body as `Bytes`.
    ///
    /// # Example
    ///
    /// ```
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let bytes = reqwest::blocking::get("http://httpbin.org/ip")?.bytes()?;
    ///
    /// println!("bytes: {bytes:?}");
    /// # Ok(())
    /// # }
    /// ```
    pub fn bytes(self) -> Result<Bytes> {
        Ok(self.0.bytes()?)
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
    /// let content = reqwest::blocking::get("http://httpbin.org/range/26")?.text()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn text(self) -> Result<String> {
        Ok(self.0.text()?)
    }

    pub fn html(self) -> Result<scraper::Html> {
        Ok(scraper::Html::parse_document(self.text()?.as_str()))
    }

    /// Copy the response body into a writer.
    ///
    /// This function internally uses [`std::io::copy`] and hence will continuously read data from
    /// the body and then write it into writer in a streaming fashion until EOF is met.
    ///
    /// On success, the total number of bytes that were copied to `writer` is returned.
    ///
    /// [`std::io::copy`]: https://doc.rust-lang.org/std/io/fn.copy.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut resp = reqwest::blocking::get("http://httpbin.org/range/5")?;
    /// let mut buf: Vec<u8> = vec![];
    /// resp.copy_to(&mut buf)?;
    /// assert_eq!(b"abcde", buf.as_slice());
    /// # Ok(())
    /// # }
    /// ```
    pub fn copy_to<W: ?Sized>(&mut self, w: &mut W) -> Result<u64>
    where
        W: std::io::Write,
    {
        Ok(self.0.copy_to(w)?)
    }

    /// Turn a response into an error if the server returned an error.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # extern crate reqwest;
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let res = reqwest::blocking::get("http://httpbin.org/status/400")?
    ///     .error_for_status();
    /// if let Err(err) = res {
    ///     assert_eq!(err.status(), Some(reqwest::StatusCode::BAD_REQUEST));
    /// }
    /// # Ok(())
    /// # }
    /// # fn main() {}
    /// ```
    pub fn error_for_status(self) -> Result<Self> {
        Ok(self.0.error_for_status()?).map(Self)
    }
}

pub struct RequestBuilder(reqwest::blocking::RequestBuilder);

impl RequestBuilder {
    pub fn with_user_agent(self) -> RequestBuilder {
        RequestBuilder(self.0.header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36"))
    }
    /// Add a `Header` to this Request.
    ///
    /// ```rust
    /// use reqwest::header::USER_AGENT;
    ///
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = reqwest::blocking::Client::new();
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
        RequestBuilder(self.0.header(key, value))
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
    /// let client = reqwest::blocking::Client::new();
    /// let res = client.post("http://httpbin.org/post")
    ///     .headers(construct_headers())
    ///     .body(file)
    ///     .send()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn headers(self, headers: header::HeaderMap) -> RequestBuilder {
        RequestBuilder(self.0.headers(headers))
    }

    /// Enable HTTP basic authentication.
    ///
    /// ```rust
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = reqwest::blocking::Client::new();
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
        RequestBuilder(self.0.basic_auth(username, password))
    }

    /// Enable HTTP bearer authentication.
    ///
    /// ```rust
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = reqwest::blocking::Client::new();
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
        RequestBuilder(self.0.bearer_auth(token))
    }

    /// Set the request body.
    ///
    /// # Examples
    ///
    /// Using a string:
    ///
    /// ```rust
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = reqwest::blocking::Client::new();
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
    /// let client = reqwest::blocking::Client::new();
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
    /// let client = reqwest::blocking::Client::new();
    /// let res = client.post("http://httpbin.org/post")
    ///     .body(bytes)
    ///     .send()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn body<T: Into<Body>>(self, body: T) -> RequestBuilder {
        RequestBuilder(self.0.body(body))
    }

    /// Enables a request timeout.
    ///
    /// The timeout is applied from when the request starts connecting until the
    /// response body has finished. It affects only this request and overrides
    /// the timeout configured using `ClientBuilder::timeout()`.
    pub fn timeout(self, timeout: Duration) -> RequestBuilder {
        RequestBuilder(self.0.timeout(timeout))
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
    /// let client = reqwest::blocking::Client::new();
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
        RequestBuilder(self.0.query(query))
    }

    /// Set HTTP version
    pub fn version(self, version: Version) -> RequestBuilder {
        RequestBuilder(self.0.version(version))
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
    /// let client = reqwest::blocking::Client::new();
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
        RequestBuilder(self.0.form(form))
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
    /// let client = reqwest::blocking::Client::new();
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
        RequestBuilder(self.0.json(json))
    }

    /// Build a `Request`, which can be inspected, modified and executed with
    /// `Client::execute()`.
    pub fn build(self) -> Result<Request> {
        Ok(self.0.build()?)
    }

    /// Build a `Request`, which can be inspected, modified and executed with
    /// `Client::execute()`.
    ///
    /// This is similar to [`RequestBuilder::build()`], but also returns the
    /// embedded `Client`.
    pub fn build_split(self) -> (Client, Result<Request>) {
        let (c, r) = self.0.build_split();
        (Client(c), r.map_err(Error::Reqwest))
    }

    /// Constructs the Request and sends it the target URL, returning a Response.
    ///
    /// # Errors
    ///
    /// This method fails if there was an error while sending request,
    /// redirect loop was detected or redirect limit was exhausted.
    pub fn send(self) -> Result<Response> {
        Ok(self.0.send()?).map(Response)
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
    /// let client = reqwest::blocking::Client::new();
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
    /// let client = reqwest::blocking::Client::new();
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
    /// let client = reqwest::blocking::Client::new();
    /// let builder = client.get("http://httpbin.org/get")
    ///     .body(reqwest::blocking::Body::new(std::io::empty()));
    /// let clone = builder.try_clone();
    /// assert!(clone.is_none());
    /// # Ok(())
    /// # }
    /// ```
    pub fn try_clone(&self) -> Option<RequestBuilder> {
        self.0.try_clone().map(RequestBuilder)
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
        RequestBuilder(self.0.request(method, url))
    }
}
