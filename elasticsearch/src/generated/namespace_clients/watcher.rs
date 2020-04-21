// -----------------------------------------------
// ███╗   ██╗ ██████╗ ████████╗██╗ ██████╗███████╗
// ████╗  ██║██╔═══██╗╚══██╔══╝██║██╔════╝██╔════╝
// ██╔██╗ ██║██║   ██║   ██║   ██║██║     █████╗
// ██║╚██╗██║██║   ██║   ██║   ██║██║     ██╔══╝
// ██║ ╚████║╚██████╔╝   ██║   ██║╚██████╗███████╗
// ╚═╝  ╚═══╝ ╚═════╝    ╚═╝   ╚═╝ ╚═════╝╚══════╝
// -----------------------------------------------
//
// This file is generated,
// Please do not edit it manually.
// Run the following in the root of the repo:
//
// cargo run -p api_generator
//
// -----------------------------------------------
#[allow(unused_imports)]
use crate::{
    client::Elasticsearch,
    error::Error,
    http::{
        headers::{HeaderMap, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE},
        request::{Body, JsonBody, NdBody},
        response::Response,
        Method,
    },
    params::*,
};
use serde::Serialize;
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Watcher Ack Watch API"]
pub enum WatcherAckWatchParts<'b> {
    #[doc = "WatchId"]
    WatchId(&'b str),
    #[doc = "WatchId and ActionId"]
    WatchIdActionId(&'b str, &'b [&'b str]),
}
impl<'b> WatcherAckWatchParts<'b> {
    #[doc = "Builds a relative URL path to the Watcher Ack Watch API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            WatcherAckWatchParts::WatchId(ref watch_id) => {
                let mut p = String::with_capacity(21usize + watch_id.len());
                p.push_str("/_watcher/watch/");
                p.push_str(watch_id.as_ref());
                p.push_str("/_ack");
                p.into()
            }
            WatcherAckWatchParts::WatchIdActionId(ref watch_id, ref action_id) => {
                let action_id_str = action_id.join(",");
                let mut p = String::with_capacity(22usize + watch_id.len() + action_id_str.len());
                p.push_str("/_watcher/watch/");
                p.push_str(watch_id.as_ref());
                p.push_str("/_ack/");
                p.push_str(action_id_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Watcher Ack Watch API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/watcher-api-ack-watch.html)\n\nAcknowledges a watch, manually throttling the execution of the watch's actions."]
pub struct WatcherAckWatch<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: WatcherAckWatchParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> WatcherAckWatch<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [WatcherAckWatch] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: WatcherAckWatchParts<'b>) -> Self {
        let headers = HeaderMap::new();
        WatcherAckWatch {
            client,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> WatcherAckWatch<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        WatcherAckWatch {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Watcher Ack Watch API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Watcher Activate Watch API"]
pub enum WatcherActivateWatchParts<'b> {
    #[doc = "WatchId"]
    WatchId(&'b str),
}
impl<'b> WatcherActivateWatchParts<'b> {
    #[doc = "Builds a relative URL path to the Watcher Activate Watch API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            WatcherActivateWatchParts::WatchId(ref watch_id) => {
                let mut p = String::with_capacity(26usize + watch_id.len());
                p.push_str("/_watcher/watch/");
                p.push_str(watch_id.as_ref());
                p.push_str("/_activate");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Watcher Activate Watch API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/watcher-api-activate-watch.html)\n\nActivates a currently inactive watch."]
pub struct WatcherActivateWatch<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: WatcherActivateWatchParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> WatcherActivateWatch<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [WatcherActivateWatch] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: WatcherActivateWatchParts<'b>) -> Self {
        let headers = HeaderMap::new();
        WatcherActivateWatch {
            client,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> WatcherActivateWatch<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        WatcherActivateWatch {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Watcher Activate Watch API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Watcher Deactivate Watch API"]
pub enum WatcherDeactivateWatchParts<'b> {
    #[doc = "WatchId"]
    WatchId(&'b str),
}
impl<'b> WatcherDeactivateWatchParts<'b> {
    #[doc = "Builds a relative URL path to the Watcher Deactivate Watch API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            WatcherDeactivateWatchParts::WatchId(ref watch_id) => {
                let mut p = String::with_capacity(28usize + watch_id.len());
                p.push_str("/_watcher/watch/");
                p.push_str(watch_id.as_ref());
                p.push_str("/_deactivate");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Watcher Deactivate Watch API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/watcher-api-deactivate-watch.html)\n\nDeactivates a currently active watch."]
pub struct WatcherDeactivateWatch<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: WatcherDeactivateWatchParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> WatcherDeactivateWatch<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [WatcherDeactivateWatch] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: WatcherDeactivateWatchParts<'b>) -> Self {
        let headers = HeaderMap::new();
        WatcherDeactivateWatch {
            client,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> WatcherDeactivateWatch<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        WatcherDeactivateWatch {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Watcher Deactivate Watch API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Watcher Delete Watch API"]
pub enum WatcherDeleteWatchParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> WatcherDeleteWatchParts<'b> {
    #[doc = "Builds a relative URL path to the Watcher Delete Watch API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            WatcherDeleteWatchParts::Id(ref id) => {
                let mut p = String::with_capacity(16usize + id.len());
                p.push_str("/_watcher/watch/");
                p.push_str(id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Watcher Delete Watch API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/watcher-api-delete-watch.html)\n\nRemoves a watch from Watcher."]
pub struct WatcherDeleteWatch<'a, 'b> {
    client: &'a Elasticsearch,
    parts: WatcherDeleteWatchParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> WatcherDeleteWatch<'a, 'b> {
    #[doc = "Creates a new instance of [WatcherDeleteWatch] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: WatcherDeleteWatchParts<'b>) -> Self {
        let headers = HeaderMap::new();
        WatcherDeleteWatch {
            client,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Watcher Delete Watch API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Watcher Execute Watch API"]
pub enum WatcherExecuteWatchParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
    #[doc = "No parts"]
    None,
}
impl<'b> WatcherExecuteWatchParts<'b> {
    #[doc = "Builds a relative URL path to the Watcher Execute Watch API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            WatcherExecuteWatchParts::Id(ref id) => {
                let mut p = String::with_capacity(25usize + id.len());
                p.push_str("/_watcher/watch/");
                p.push_str(id.as_ref());
                p.push_str("/_execute");
                p.into()
            }
            WatcherExecuteWatchParts::None => "/_watcher/watch/_execute".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Watcher Execute Watch API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/watcher-api-execute-watch.html)\n\nForces the execution of a stored watch."]
pub struct WatcherExecuteWatch<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: WatcherExecuteWatchParts<'b>,
    body: Option<B>,
    debug: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> WatcherExecuteWatch<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [WatcherExecuteWatch] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: WatcherExecuteWatchParts<'b>) -> Self {
        let headers = HeaderMap::new();
        WatcherExecuteWatch {
            client,
            parts,
            headers,
            body: None,
            debug: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> WatcherExecuteWatch<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        WatcherExecuteWatch {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            debug: self.debug,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "indicates whether the watch should execute in debug mode"]
    pub fn debug(mut self, debug: bool) -> Self {
        self.debug = Some(debug);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Watcher Execute Watch API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "debug")]
                debug: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                debug: self.debug,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Watcher Get Watch API"]
pub enum WatcherGetWatchParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> WatcherGetWatchParts<'b> {
    #[doc = "Builds a relative URL path to the Watcher Get Watch API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            WatcherGetWatchParts::Id(ref id) => {
                let mut p = String::with_capacity(16usize + id.len());
                p.push_str("/_watcher/watch/");
                p.push_str(id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Watcher Get Watch API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/watcher-api-get-watch.html)\n\nRetrieves a watch by its ID."]
pub struct WatcherGetWatch<'a, 'b> {
    client: &'a Elasticsearch,
    parts: WatcherGetWatchParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> WatcherGetWatch<'a, 'b> {
    #[doc = "Creates a new instance of [WatcherGetWatch] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: WatcherGetWatchParts<'b>) -> Self {
        let headers = HeaderMap::new();
        WatcherGetWatch {
            client,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Watcher Get Watch API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Watcher Put Watch API"]
pub enum WatcherPutWatchParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> WatcherPutWatchParts<'b> {
    #[doc = "Builds a relative URL path to the Watcher Put Watch API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            WatcherPutWatchParts::Id(ref id) => {
                let mut p = String::with_capacity(16usize + id.len());
                p.push_str("/_watcher/watch/");
                p.push_str(id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Watcher Put Watch API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/watcher-api-put-watch.html)\n\nCreates a new watch, or updates an existing one."]
pub struct WatcherPutWatch<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: WatcherPutWatchParts<'b>,
    active: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    if_primary_term: Option<i64>,
    if_seq_no: Option<i64>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    version: Option<i64>,
}
impl<'a, 'b, B> WatcherPutWatch<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [WatcherPutWatch] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: WatcherPutWatchParts<'b>) -> Self {
        let headers = HeaderMap::new();
        WatcherPutWatch {
            client,
            parts,
            headers,
            active: None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            if_primary_term: None,
            if_seq_no: None,
            pretty: None,
            source: None,
            version: None,
        }
    }
    #[doc = "Specify whether the watch is in/active by default"]
    pub fn active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> WatcherPutWatch<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        WatcherPutWatch {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            active: self.active,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            if_primary_term: self.if_primary_term,
            if_seq_no: self.if_seq_no,
            pretty: self.pretty,
            source: self.source,
            version: self.version,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "only update the watch if the last operation that has changed the watch has the specified primary term"]
    pub fn if_primary_term(mut self, if_primary_term: i64) -> Self {
        self.if_primary_term = Some(if_primary_term);
        self
    }
    #[doc = "only update the watch if the last operation that has changed the watch has the specified sequence number"]
    pub fn if_seq_no(mut self, if_seq_no: i64) -> Self {
        self.if_seq_no = Some(if_seq_no);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Explicit version number for concurrency control"]
    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "Creates an asynchronous call to the Watcher Put Watch API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "active")]
                active: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "if_primary_term")]
                if_primary_term: Option<i64>,
                #[serde(rename = "if_seq_no")]
                if_seq_no: Option<i64>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "version")]
                version: Option<i64>,
            }
            let query_params = QueryParams {
                active: self.active,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                if_primary_term: self.if_primary_term,
                if_seq_no: self.if_seq_no,
                pretty: self.pretty,
                source: self.source,
                version: self.version,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Watcher Start API"]
pub enum WatcherStartParts {
    #[doc = "No parts"]
    None,
}
impl WatcherStartParts {
    #[doc = "Builds a relative URL path to the Watcher Start API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            WatcherStartParts::None => "/_watcher/_start".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Watcher Start API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/watcher-api-start.html)\n\nStarts Watcher if it is not already running."]
pub struct WatcherStart<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: WatcherStartParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> WatcherStart<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [WatcherStart]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        let headers = HeaderMap::new();
        WatcherStart {
            client,
            parts: WatcherStartParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> WatcherStart<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        WatcherStart {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Watcher Start API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Watcher Stats API"]
pub enum WatcherStatsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Metric"]
    Metric(&'b [&'b str]),
}
impl<'b> WatcherStatsParts<'b> {
    #[doc = "Builds a relative URL path to the Watcher Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            WatcherStatsParts::None => "/_watcher/stats".into(),
            WatcherStatsParts::Metric(ref metric) => {
                let metric_str = metric.join(",");
                let mut p = String::with_capacity(16usize + metric_str.len());
                p.push_str("/_watcher/stats/");
                p.push_str(metric_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Watcher Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/watcher-api-stats.html)\n\nRetrieves the current Watcher metrics."]
pub struct WatcherStats<'a, 'b> {
    client: &'a Elasticsearch,
    parts: WatcherStatsParts<'b>,
    emit_stacktraces: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    metric: Option<&'b [&'b str]>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> WatcherStats<'a, 'b> {
    #[doc = "Creates a new instance of [WatcherStats] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: WatcherStatsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        WatcherStats {
            client,
            parts,
            headers,
            emit_stacktraces: None,
            error_trace: None,
            filter_path: None,
            human: None,
            metric: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Emits stack traces of currently running watches"]
    pub fn emit_stacktraces(mut self, emit_stacktraces: bool) -> Self {
        self.emit_stacktraces = Some(emit_stacktraces);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Controls what additional stat metrics should be include in the response"]
    pub fn metric(mut self, metric: &'b [&'b str]) -> Self {
        self.metric = Some(metric);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Watcher Stats API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "emit_stacktraces")]
                emit_stacktraces: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "metric", serialize_with = "crate::client::serialize_coll_qs")]
                metric: Option<&'b [&'b str]>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                emit_stacktraces: self.emit_stacktraces,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                metric: self.metric,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Watcher Stop API"]
pub enum WatcherStopParts {
    #[doc = "No parts"]
    None,
}
impl WatcherStopParts {
    #[doc = "Builds a relative URL path to the Watcher Stop API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            WatcherStopParts::None => "/_watcher/_stop".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Watcher Stop API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/watcher-api-stop.html)\n\nStops Watcher if it is running."]
pub struct WatcherStop<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: WatcherStopParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> WatcherStop<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [WatcherStop]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        let headers = HeaderMap::new();
        WatcherStop {
            client,
            parts: WatcherStopParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> WatcherStop<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        WatcherStop {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Watcher Stop API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[doc = "Namespace client for Watcher APIs"]
pub struct Watcher<'a> {
    client: &'a Elasticsearch,
}
impl<'a> Watcher<'a> {
    #[doc = "Creates a new instance of [Watcher]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        Self { client }
    }
    #[doc = "[Watcher Ack Watch API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/watcher-api-ack-watch.html)\n\nAcknowledges a watch, manually throttling the execution of the watch's actions."]
    pub fn ack_watch<'b>(&'a self, parts: WatcherAckWatchParts<'b>) -> WatcherAckWatch<'a, 'b, ()> {
        WatcherAckWatch::new(&self.client, parts)
    }
    #[doc = "[Watcher Activate Watch API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/watcher-api-activate-watch.html)\n\nActivates a currently inactive watch."]
    pub fn activate_watch<'b>(
        &'a self,
        parts: WatcherActivateWatchParts<'b>,
    ) -> WatcherActivateWatch<'a, 'b, ()> {
        WatcherActivateWatch::new(&self.client, parts)
    }
    #[doc = "[Watcher Deactivate Watch API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/watcher-api-deactivate-watch.html)\n\nDeactivates a currently active watch."]
    pub fn deactivate_watch<'b>(
        &'a self,
        parts: WatcherDeactivateWatchParts<'b>,
    ) -> WatcherDeactivateWatch<'a, 'b, ()> {
        WatcherDeactivateWatch::new(&self.client, parts)
    }
    #[doc = "[Watcher Delete Watch API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/watcher-api-delete-watch.html)\n\nRemoves a watch from Watcher."]
    pub fn delete_watch<'b>(
        &'a self,
        parts: WatcherDeleteWatchParts<'b>,
    ) -> WatcherDeleteWatch<'a, 'b> {
        WatcherDeleteWatch::new(&self.client, parts)
    }
    #[doc = "[Watcher Execute Watch API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/watcher-api-execute-watch.html)\n\nForces the execution of a stored watch."]
    pub fn execute_watch<'b>(
        &'a self,
        parts: WatcherExecuteWatchParts<'b>,
    ) -> WatcherExecuteWatch<'a, 'b, ()> {
        WatcherExecuteWatch::new(&self.client, parts)
    }
    #[doc = "[Watcher Get Watch API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/watcher-api-get-watch.html)\n\nRetrieves a watch by its ID."]
    pub fn get_watch<'b>(&'a self, parts: WatcherGetWatchParts<'b>) -> WatcherGetWatch<'a, 'b> {
        WatcherGetWatch::new(&self.client, parts)
    }
    #[doc = "[Watcher Put Watch API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/watcher-api-put-watch.html)\n\nCreates a new watch, or updates an existing one."]
    pub fn put_watch<'b>(&'a self, parts: WatcherPutWatchParts<'b>) -> WatcherPutWatch<'a, 'b, ()> {
        WatcherPutWatch::new(&self.client, parts)
    }
    #[doc = "[Watcher Start API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/watcher-api-start.html)\n\nStarts Watcher if it is not already running."]
    pub fn start<'b>(&'a self) -> WatcherStart<'a, 'b, ()> {
        WatcherStart::new(&self.client)
    }
    #[doc = "[Watcher Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/watcher-api-stats.html)\n\nRetrieves the current Watcher metrics."]
    pub fn stats<'b>(&'a self, parts: WatcherStatsParts<'b>) -> WatcherStats<'a, 'b> {
        WatcherStats::new(&self.client, parts)
    }
    #[doc = "[Watcher Stop API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/watcher-api-stop.html)\n\nStops Watcher if it is running."]
    pub fn stop<'b>(&'a self) -> WatcherStop<'a, 'b, ()> {
        WatcherStop::new(&self.client)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Watcher APIs"]
    pub fn watcher(&self) -> Watcher {
        Watcher::new(&self)
    }
}
