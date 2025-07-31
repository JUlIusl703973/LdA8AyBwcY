use actix::{Actor, Context, Handler, Message, WrapFuture};
use actix_web::{web, HttpResponse, Responder, Error};
use std::time::Duration;
use std::sync::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::time::sleep;
use actix_web::dev::ServiceResponse;
use futures::future::{ok, Ready};
use actix_web::Error as WebError;
use futures::FutureExt;

/// Cache Key
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct CacheKey(String);

/// Cached Response - a simple cache store
#[derive(Clone)]
struct CachedResponse {
    response: HttpResponse,
    expires: tokio::time::Instant,
}

/// Cache Actor - Manages caching
pub struct CacheActor {
    cache: Arc<Mutex<HashMap<CacheKey, CachedResponse>>>,
}

impl CacheActor {
    pub fn new() -> Self {
        CacheActor {
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Actor for CacheActor {
    type Context = Context<Self>;
}

/// Message that checks and returns cached response if available
#[derive(Message, Clone)]
#[rtype(result = "Result<HttpResponse, WebError>