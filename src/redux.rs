pub type ReducerFunction<S, A> = fn(&S, &A) -> S;
pub type Subscriber<S> = fn(&S);
pub type Middleware<S, A> = fn(&mut Store<S, A>, A) -> Option<A>;

pub struct Store<S, A> {
    reducer: ReducerFunction<S, A>,
    state: S,
    middleware: Vec<Middleware<S, A>>,
    subscribers: Vec<Subscriber<S>>,
}

impl<S, A> Store<S, A> {
    pub fn new(reducer: ReducerFunction<S, A>, initial_state: S) -> Self {
        Self {
            reducer,
            state: initial_state,
            middleware: Vec::new(),
            subscribers: Vec::new(),
        }
    }

    pub fn get_state(&self) -> &S {
        &self.state
    }
    /// execute reducer with action to update state
    pub fn dispatch(&mut self, action: A) {
        if self.middleware.is_empty() {
            self.dispatch_reducer(&action);
        } else {
            self.dispatch_middleware(0, action);
        }
    }

    /// execute listener when state changes
    pub fn subscribe(&mut self, listener: Subscriber<S>) {
        self.subscribers.push(listener);
    }

    /// applies a middleware to the store
    /// dispatched actions will be piped through the middleware to the current reducer function
    pub fn apply_middleware(&mut self, middleware: Middleware<S, A>) {
        self.middleware.push(middleware);
    }

    /// replace the current reducer function
    pub fn replace_reducer(&mut self, new_reducer: ReducerFunction<S, A>) {
        self.reducer = new_reducer;
    }

    /// execute reducer function
    fn dispatch_reducer(&mut self, action: &A) {
        self.state = (&self.reducer)(self.get_state(), action);
        self.dispatch_subscribers();
    }
    /// execute middlewares
    fn dispatch_middleware(&mut self, i: usize, action: A) {
        if i == self.middleware.len() {
            self.dispatch_reducer(&action);
            return;
        }
        let next_middleware = self.middleware[i](self, action);
        if next_middleware.is_none() {
            return;
        }

        self.dispatch_middleware(i + 1, next_middleware.unwrap());
    }
    // execute subscribers to the store
    fn dispatch_subscribers(&self) {
        for subscriber in &self.subscribers {
            subscriber(self.get_state());
        }
    }
}
