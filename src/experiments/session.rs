use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

use axum::{
    extract::Request,
    middleware::{from_fn, Next},
    response::{IntoResponse, Response},
    routing::get,
    Extension, Router,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};

pub fn routes() -> Router {
    Router::new()
        .route("/session", get(set_session))
        .layer(from_fn(session_layer))
}

pub type Session = SessionWrapper<SessionData>;

#[derive(Default, Debug, Clone)]
pub struct SessionData {
    now: Option<u64>,
}

async fn session_layer(jar: CookieJar, mut request: Request, next: Next) -> Response {
    let sess = Session::default();
    request.extensions_mut().insert(sess.clone());

    let res = next.run(request).await;

    let sd = sess.into_inner();
    let jar = jar.add(Cookie::new("unix_time", sd.now.unwrap().to_string()));

    (jar, res).into_response()
}

pub async fn set_session(Extension(session): Extension<Session>) -> impl IntoResponse {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    session.update(|s| s.now = Some(now));

    "ok"
}

#[derive(Clone)]
pub struct SessionWrapper<T>(Arc<Mutex<T>>)
where
    T: Debug;

impl<T> Default for SessionWrapper<T>
where
    T: Default + Debug,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> SessionWrapper<T>
where
    T: Debug,
{
    pub fn new(t: T) -> Self {
        Self(Arc::new(Mutex::new(t)))
    }

    pub fn into_inner(self) -> T {
        Arc::try_unwrap(self.0).unwrap().into_inner().unwrap()
    }

    pub fn set(&self, new: T) {
        *self.0.lock().unwrap() = new;
    }

    pub fn update(&self, f: impl FnOnce(&mut T)) {
        tracing::debug!("SessionWrapper::update");
        let mut inner = self.0.lock().unwrap();
        f(&mut inner);
    }

    // TODO: get, set, update
}

impl SessionWrapper<SessionData> {
    pub fn set_now(&self, now: u64) {
        self.0.lock().unwrap().now = Some(now);
    }
}
