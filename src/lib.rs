#![allow(missing_docs)] // todo: make this a deny eventually

use std::fmt;
use std::pin::Pin;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use futures::{future, prelude::*, Future as StdFuture, Stream as StdStream};
#[cfg(feature = "httpcache")]
use http::header::IF_NONE_MATCH;
use http::header::{HeaderMap, HeaderValue};
use http::header::{ACCEPT, AUTHORIZATION, ETAG, LINK, USER_AGENT};
use http::{Method, StatusCode};
#[cfg(feature = "httpcache")]
use hyperx::header::LinkValue;
use hyperx::header::{qitem, Link, RelationType};
use log::{debug, trace};
use mime::Mime;
use reqwest::Url;
use reqwest::{Body, Client};
use serde::de::DeserializeOwned;

#[doc(hidden)] // public for doc testing and integration testing only
#[cfg(feature = "httpcache")]
pub mod http_cache;
#[macro_use]
mod macros; // expose json! macro to child modules
pub mod errors;
pub mod projects;
pub mod repository;
pub mod pull_requests;

pub use crate::errors::{Error, ErrorKind, Result};
#[cfg(feature = "httpcache")]
pub use crate::http_cache::{BoxedHttpCache, HttpCache};

use crate::projects::{Project, Projects};
use crate::repository::{Repositories, Repository};

const DEFAULT_HOST: &str = "https://dev.azure.com";
/// A type alias for `Futures` that may return `azure_rs::Errors`
pub type Future<T> = Pin<Box<dyn StdFuture<Output = Result<T>> + Send>>;

/// A type alias for `Streams` that may result in `azure_rs::Errors`
pub type Stream<T> = Pin<Box<dyn StdStream<Item = Result<T>> + Send>>;

/// Rate limiting
///
/// https://docs.microsoft.com/en-us/azure/devops/integrate/concepts/rate-limits?view=azure-devops#api-client-experience
const X_RATELIMIT_LIMIT: &str = "x-ratelimit-limit";
const X_RATELIMIT_REMAINING: &str = "x-ratelimit-remaining";
const X_RATELIMIT_RESET: &str = "x-ratelimit-reset";

#[derive(Clone, Copy)]
pub enum MediaType {
    /// Return json (the default)
    Json,
    /// Return json in preview form
    JsonPatch,
}

impl Default for MediaType {
    fn default() -> MediaType {
        MediaType::Json
    }
}

impl From<MediaType> for Mime {
    fn from(media: MediaType) -> Mime {
        match media {
            MediaType::Json => "application/json".parse().unwrap(),
            MediaType::JsonPatch => "application/json-patch+json".parse().unwrap(),
        }
    }
}

/// Controls what sort of authentication is required for this request
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AuthenticationConstraint {
    /// No constraint
    Unconstrained,
}

/// enum representation of Azure list sorting options
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SortDirection {
    /// Sort in ascending order (the default)
    Asc,
    /// Sort in descending order
    Desc,
}

impl fmt::Display for SortDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SortDirection::Asc => "asc",
            SortDirection::Desc => "desc",
        }
        .fmt(f)
    }
}

impl Default for SortDirection {
    fn default() -> SortDirection {
        SortDirection::Asc
    }
}

/// enum representation of Azure api versions
/// by default 5.1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ApiVersion {
    V5_1,
    V5_0,
}

impl fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ApiVersion::V5_1 => "api-version=5.1",
            ApiVersion::V5_0 => "api-version=5.0",
        }
        .fmt(f)
    }
}

impl Default for ApiVersion {
    fn default() -> ApiVersion {
        ApiVersion::V5_1
    }
}

/// Various forms of authentication credentials supported by Azure
#[derive(Debug, PartialEq, Clone)]
pub enum Credentials {
    /// Oauth token string
    /// https://developer.github.com/v3/#oauth2-token-sent-in-a-header
    Token(String),
    /// Basic authentication base64 encoded
    Basic(String),
    /// Oauth client id and secret
    /// https://developer.github.com/v3/#oauth2-keysecret
    Client(String, String),
}

/// Entry point interface for interacting with Azure API
#[derive(Clone, Debug)]
pub struct AzureClient {
    host: String,
    agent: String,
    org: String,
    client: Client,
    credentials: Option<Credentials>,
    #[cfg(feature = "httpcache")]
    http_cache: BoxedHttpCache,
    api_version: ApiVersion,
}

impl AzureClient {
    pub fn new<A, O, C>(agent: A, org: O, credentials: C) -> Result<Self>
    where
        A: Into<String>,
        O: Into<String>,
        C: Into<Option<Credentials>>,
    {
        Self::host(DEFAULT_HOST, agent, org, credentials)
    }

    pub fn host<H, O, A, C>(host: H, agent: A, org: O, credentials: C) -> Result<Self>
    where
        H: Into<String>,
        A: Into<String>,
        O: Into<String>,
        C: Into<Option<Credentials>>,
    {
        let http = Client::builder().build()?;
        #[cfg(feature = "httpcache")]
        {
            Ok(Self::custom(
                host,
                agent,
                org,
                credentials,
                http,
                HttpCache::noop(),
            ))
        }
        #[cfg(not(feature = "httpcache"))]
        {
            Ok(Self::custom(host, agent, org, credentials, http))
        }
    }

    #[cfg(feature = "httpcache")]
    pub fn custom<H, A, O, CR>(
        host: H,
        agent: A,
        org: O,
        credentials: CR,
        http: Client,
        http_cache: BoxedHttpCache,
    ) -> Self
    where
        H: Into<String>,
        A: Into<String>,
        O: Into<String>,
        CR: Into<Option<Credentials>>,
    {
        Self {
            host: host.into(),
            agent: agent.into(),
            org: org.into(),
            client: http,
            credentials: credentials.into(),
            http_cache,
            api_version: ApiVersion::default(),
        }
    }

    #[cfg(not(feature = "httpcache"))]
    pub fn custom<H, A, O, CR>(host: H, agent: A, org: O, credentials: CR, http: Client) -> Self
    where
        H: Into<String>,
        A: Into<String>,
        O: Into<String>,
        CR: Into<Option<Credentials>>,
    {
        Self {
            host: host.into(),
            agent: agent.into(),
            org: org.into(),
            client: http,
            credentials: credentials.into(),
            api_version: ApiVersion::default(),
        }
    }

    pub fn set_credentials<CR>(&mut self, credentials: CR)
    where
        CR: Into<Option<Credentials>>,
    {
        self.credentials = credentials.into();
    }
    pub fn set_api_version<V>(&mut self, version: V)
    where
        V: Into<ApiVersion>,
    {
        self.api_version = version.into();
    }

    pub fn set_host<H>(&mut self, host: H)
    where
    H: Into<String>,
    {
        self.host = host.into();
    }

    pub fn set_organization<O>(&mut self, org: O)
    where
        O: Into<String>,
    {
        self.org = org.into();
    }

    pub fn projects(&self) -> Projects {
        Projects::new(self.clone())
    }

    pub fn project<P>(&self, project: P) -> Project
    where
        P: Into<String>,
    {
        Project::new(self.clone(), project)
    }

    pub fn repo<P, R>(&self, project: P, repo: R) -> Repository
    where
        P: Into<String>,
        R: Into<String>,
    {
        Repository::new(self.clone(), project, repo)
    }

    /// Get all repos in a organization
    ///
    /// GET https://dev.azure.com/{organization}/_apis/git/repositories?api-version=
    pub fn org_repos(&self) {
        // TODO
    }

    /// Get all repos in a project
    ///
    /// GET https://dev.azure.com/{organization}/{project}/_apis/git/repositories?api-version=5.1
    pub fn repos<P>(&self, project: P) -> Repositories
    where
        P: Into<String>,
    {
        Repositories::new(self.clone(), project)
    }

    fn credentials(&self, authentication: AuthenticationConstraint) -> Option<&Credentials> {
        match (authentication, self.credentials.as_ref()) {
            (AuthenticationConstraint::Unconstrained, creds) => creds,
        }
    }

    fn url_and_auth(
        &self,
        uri: &str,
        authentication: AuthenticationConstraint,
    ) -> Future<(Url, Option<String>)> {
        let mut m = uri.to_owned();
        m.push_str(&format!("?{}", self.api_version.to_string()));
        let parsed_url = m.parse::<Url>();

        match self.credentials(authentication) {
            Some(&Credentials::Client(ref id, ref secret)) => Box::pin(future::ready(
                parsed_url
                    .map(|mut u| {
                        u.query_pairs_mut()
                            .append_pair("client_id", id)
                            .append_pair("client_secret", secret);
                        (u, None)
                    })
                    .map_err(Error::from),
            )),
            Some(&Credentials::Token(ref token)) => {
                let auth = format!("token {}", token);
                Box::pin(future::ready(
                    parsed_url.map(|u| (u, Some(auth))).map_err(Error::from),
                ))
            }
            Some(&Credentials::Basic(ref token)) => {
                
                let b = base64::encode(format!("pat:{}", token));
                let auth = format!("Basic {}", b);
                Box::pin(future::ready(
                    parsed_url.map(|u| (u, Some(auth))).map_err(Error::from),
                ))
            }
            None => Box::pin(future::ready(
                parsed_url.map(|u| (u, None)).map_err(Error::from),
            )),
        }
    }

    fn request<Out>(
        &self,
        method: Method,
        uri: &str,
        body: Option<Vec<u8>>,
        media_type: MediaType,
        authentication: AuthenticationConstraint,
    ) -> Future<(Option<Link>, Out)>
    where
        Out: DeserializeOwned + 'static + Send,
    {
        let url_and_auth = self.url_and_auth(uri, authentication);

        let instance = self.clone();
        #[cfg(feature = "httpcache")]
        let uri2 = uri.to_string();
        let body2 = body.clone();
        let method2 = method.clone();
        let response = url_and_auth
            .map_err(Error::from)
            .and_then(move |(url, auth)| {
                #[cfg(not(feature = "httpcache"))]
                let mut req = instance.client.request(method2, url);

                #[cfg(feature = "httpcache")]
                let mut req = {
                    let mut req = instance.client.request(method2.clone(), url);
                    if method2 == Method::GET {
                        if let Ok(etag) = instance.http_cache.lookup_etag(&uri2) {
                            req = req.header(IF_NONE_MATCH, etag);
                        }
                    }
                    req
                };

                req = req.header(USER_AGENT, &*instance.agent);
                req = req.header(
                    ACCEPT,
                    &*format!("{}", qitem::<Mime>(From::from(media_type))),
                );
                req = req.header(
                    "Content-Type",
                    &*format!("{}", qitem::<Mime>(From::from(media_type))),
                );

                if let Some(auth_str) = auth {
                    req = req.header(AUTHORIZATION, &*auth_str);
                }

                trace!("Body: {:?}", &body2);
                if let Some(body) = body2 {
                    req = req.body(Body::from(body));
                }
                debug!("Request: {:?}", &req);
                req.send().map_err(Error::from)
            });

        #[cfg(feature = "httpcache")]
        let instance2 = self.clone();

        #[cfg(feature = "httpcache")]
        let uri3 = uri.to_string();
        Box::pin(response.and_then(move |response| {
            #[cfg(not(feature = "httpcache"))]
            let (remaining, reset) = get_header_values(response.headers());
            #[cfg(feature = "httpcache")]
            let (remaining, reset, etag) = get_header_values(response.headers());

            let status = response.status();
            let link = response
                .headers()
                .get(LINK)
                .and_then(|l| l.to_str().ok())
                .and_then(|l| l.parse().ok());

            Box::pin(
                response
                    .bytes()
                    .map_err(Error::from)
                    .and_then(move |response_body| async move {
                        if status.is_success() {
                            debug!(
                                "response payload {}",
                                String::from_utf8_lossy(&response_body)
                            );
                            #[cfg(feature = "httpcache")]
                            {
                                if let Some(etag) = etag {
                                    let next_link = link.as_ref().and_then(|l| next_link(&l));
                                    if let Err(e) = instance2.http_cache.cache_response(
                                        &uri3,
                                        &response_body,
                                        &etag,
                                        &next_link,
                                    ) {
                                        // failing to cache isn't fatal, so just log & swallow the error
                                        debug!("Failed to cache body & etag: {}", e);
                                    }
                                }
                            }
                            let parsed_response : std::result::Result<Out, serde_json::error::Error> = if status == StatusCode::NO_CONTENT { serde_json::from_str("null") } else { serde_json::from_slice::<Out>(&response_body) };
                            parsed_response
                                .map(|out| (link, out))
                                .map_err(|error| ErrorKind::Codec(error).into())
                        } else if status == StatusCode::NOT_MODIFIED {
                            // only supported case is when client provides if-none-match
                            // header when cargo builds with --cfg feature="httpcache"
                            #[cfg(feature = "httpcache")]
                            {
                                instance2
                                    .http_cache
                                    .lookup_body(&uri3)
                                    .map_err(Error::from)
                                    .and_then(|body| {
                                        serde_json::from_str::<Out>(&body)
                                            .map_err(Error::from)
                                            .and_then(|out| {
                                                let link = match link {
                                                    Some(link) => Ok(Some(link)),
                                                    None => instance2
                                                        .http_cache
                                                        .lookup_next_link(&uri3)
                                                        .map(|next_link| next_link.map(|next| {
                                                            let next = LinkValue::new(next).push_rel(RelationType::Next);
                                                            Link::new(vec![next])
                                                        }))
                                                };
                                                link.map(|link| (link, out))
                                            })
                                    })
                            }
                            #[cfg(not(feature = "httpcache"))]
                            {
                                unreachable!("this should not be reachable without the httpcache feature enabled")
                            }
                        } else {
                            let error = match (remaining, reset) {
                                (Some(remaining), Some(reset)) if remaining == 0 => {
                                    let now = SystemTime::now()
                                        .duration_since(UNIX_EPOCH)
                                        .unwrap()
                                        .as_secs();
                                    ErrorKind::RateLimit {
                                        reset: Duration::from_secs(u64::from(reset) - now),
                                    }
                                }
                                _ => ErrorKind::Fault {
                                    code: status,
                                    error: serde_json::from_slice(&response_body)?,
                                },
                            };
                            Err(error.into())
                        }
                    }),
            )
        }))
    }

    fn request_entity<D>(
        &self,
        method: Method,
        uri: &str,
        body: Option<Vec<u8>>,
        media_type: MediaType,
        authentication: AuthenticationConstraint,
    ) -> Future<D>
    where
        D: DeserializeOwned + 'static + Send,
    {
        Box::pin(
            self.request(method, uri, body, media_type, authentication)
                .map_ok(|(_, entity)| entity),
        )
    }

    fn get<D>(&self, uri: &str) -> Future<D>
    where
        D: DeserializeOwned + 'static + Send,
    {
        self.get_media(uri, MediaType::Json)
    }

    fn get_media<D>(&self, uri: &str, media: MediaType) -> Future<D>
    where
        D: DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            Method::GET,
            &(self.host.clone() + uri),
            None,
            media,
            AuthenticationConstraint::Unconstrained,
        )
    }

    /// a delete request that returns a response
    fn delete<D>(&self, uri: &str) -> Future<D>
    where
        D: DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            Method::DELETE,
            &(self.host.clone() + uri),
            None,
            MediaType::Json,
            AuthenticationConstraint::Unconstrained,
        )
    }

    fn post<D>(&self, uri: &str, message: Vec<u8>) -> Future<D>
    where
        D: DeserializeOwned + 'static + Send,
    {
        self.post_media(
            uri,
            message,
            MediaType::Json,
            AuthenticationConstraint::Unconstrained,
        )
    }

    fn post_media<D>(
        &self,
        uri: &str,
        message: Vec<u8>,
        media: MediaType,
        authentication: AuthenticationConstraint,
    ) -> Future<D>
    where
        D: DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            Method::POST,
            &(self.host.clone() + uri),
            Some(message),
            media,
            authentication,
        )
    }

    fn patch_media<D>(&self, uri: &str, message: Vec<u8>, media: MediaType) -> Future<D>
    where
        D: DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            Method::PATCH,
            &(self.host.clone() + uri),
            Some(message),
            media,
            AuthenticationConstraint::Unconstrained,
        )
    }

    fn patch<D>(&self, uri: &str, message: Vec<u8>) -> Future<D>
    where
        D: DeserializeOwned + 'static + Send,
    {
        self.patch_media(uri, message, MediaType::Json)
    }
}

#[allow(unused)]
fn next_link(l: &Link) -> Option<String> {
    l.values()
        .into_iter()
        .find(|v| v.rel().unwrap_or(&[]).get(0) == Some(&RelationType::Next))
        .map(|v| v.link().to_owned())
}

#[cfg(not(feature = "httpcache"))]
type HeaderValues = (Option<u32>, Option<u32>);
#[cfg(feature = "httpcache")]
type HeaderValues = (Option<u32>, Option<u32>, Option<Vec<u8>>);

/// [See docs](https://docs.microsoft.com/en-us/azure/azure-resource-manager/management/request-limits-and-throttling)
fn get_header_values(headers: &HeaderMap<HeaderValue>) -> HeaderValues {
    if let Some(value) = headers.get(X_RATELIMIT_LIMIT) {
        debug!("x-rate-limit-limit: {:?}", value)
    }
    let remaining = headers
        .get(X_RATELIMIT_REMAINING)
        .and_then(|val| val.to_str().ok())
        .and_then(|val| val.parse::<u32>().ok());
    let reset = headers
        .get(X_RATELIMIT_RESET)
        .and_then(|val| val.to_str().ok())
        .and_then(|val| val.parse::<u32>().ok());
    if let Some(value) = remaining {
        debug!("x-rate-limit-remaining: {}", value)
    }
    if let Some(value) = reset {
        debug!("x-rate-limit-reset: {}", value)
    }
    let etag = headers.get(ETAG);
    if let Some(value) = etag {
        debug!("etag: {:?}", value)
    }

    #[cfg(feature = "httpcache")]
    {
        let etag = etag.map(|etag| etag.as_bytes().to_vec());
        (remaining, reset, etag)
    }
    #[cfg(not(feature = "httpcache"))]
    (remaining, reset)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_sort_direction() {
        let default: SortDirection = Default::default();
        assert_eq!(default, SortDirection::Asc)
    }

    #[test]
    #[cfg(not(feature = "httpcache"))]
    fn header_values() {
        let empty = HeaderMap::new();
        let actual = get_header_values(&empty);
        let expected = (None, None);
        assert_eq!(actual, expected);

        let mut all_valid = HeaderMap::new();
        all_valid.insert(X_RATELIMIT_REMAINING, HeaderValue::from_static("1234"));
        all_valid.insert(X_RATELIMIT_RESET, HeaderValue::from_static("5678"));
        let actual = get_header_values(&all_valid);
        let expected = (Some(1234), Some(5678));
        assert_eq!(actual, expected);

        let mut invalid = HeaderMap::new();
        invalid.insert(X_RATELIMIT_REMAINING, HeaderValue::from_static("foo"));
        invalid.insert(X_RATELIMIT_RESET, HeaderValue::from_static("bar"));
        let actual = get_header_values(&invalid);
        let expected = (None, None);
        assert_eq!(actual, expected);
    }

    #[test]
    #[cfg(feature = "httpcache")]
    fn header_values() {
        let empty = HeaderMap::new();
        let actual = get_header_values(&empty);
        let expected = (None, None, None);
        assert_eq!(actual, expected);

        let mut all_valid = HeaderMap::new();
        all_valid.insert(X_RATELIMIT_REMAINING, HeaderValue::from_static("1234"));
        all_valid.insert(X_RATELIMIT_RESET, HeaderValue::from_static("5678"));
        all_valid.insert(ETAG, HeaderValue::from_static("foobar"));
        let actual = get_header_values(&all_valid);
        let expected = (Some(1234), Some(5678), Some(b"foobar".to_vec()));
        assert_eq!(actual, expected);

        let mut invalid = HeaderMap::new();
        invalid.insert(X_RATELIMIT_REMAINING, HeaderValue::from_static("foo"));
        invalid.insert(X_RATELIMIT_RESET, HeaderValue::from_static("bar"));
        invalid.insert(ETAG, HeaderValue::from_static(""));
        let actual = get_header_values(&invalid);
        let expected = (None, None, Some(Vec::new()));
        assert_eq!(actual, expected);
    }
}
