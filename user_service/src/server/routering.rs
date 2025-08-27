use axum::{Router, routing::get};

pub trait ISubRouter {
    fn route(&self) -> Router;
    fn path(&self) -> &'static str;
}

pub struct MainRouter {
    handlers: Vec<Box<dyn ISubRouter>>,
}

impl MainRouter {
    pub fn new() -> Self {
        return Self {
            handlers: Vec::new(),
        };
    }

    pub fn add<T: ISubRouter + 'static>(mut self, handler: T) -> Self {
        self.handlers.push(Box::new(handler));
        self
    }

    pub fn build_router(&self) -> Router {
        let mut app: Router = Router::new().route("/", get(check_health));

        for handler in &self.handlers {
            app = app.nest(handler.path(), handler.route());
        }

        return app;
    }
}

pub async fn check_health() -> &'static str {
    "Im A good server buddy 😎🔥\nAsk and I'll Help You On My Life 💪🔥"
}
