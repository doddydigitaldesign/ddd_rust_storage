mod redux;

pub use redux::{Middleware, ReducerFunction, Store, Subscriber};
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::Store;
        #[derive(Eq, PartialEq, PartialOrd, Debug)]
        enum ActionType {
            Hello = 0,
            Goodbye = 1,
        }

        #[derive(Default)]
        struct State {
            message: &'static str,
        }

        fn reducer(_state: &State, action_type: &ActionType) -> State {
            match action_type {
                ActionType::Hello => State {
                    message: "Hello there!",
                },
                ActionType::Goodbye => State {
                    message: "Goodbye for now...",
                },
            }
        }

        let mut my_store = Store::new(reducer, State::default());
        fn middleware(_: &mut Store<State, ActionType>, action: ActionType) -> Option<ActionType> {
            match action {
                ActionType::Hello => Some(ActionType::Goodbye),
                ActionType::Goodbye => Some(ActionType::Hello),
            }
        }
        my_store.apply_middleware(middleware);
        my_store.subscribe(|state: &State| {
            println!("State changed: {}", state.message);
        });

        my_store.dispatch(ActionType::Hello);
        my_store.dispatch(ActionType::Goodbye);

        fn replacing_reducer(_state: &State, action_type: &ActionType) -> State {
            match action_type {
                ActionType::Hello => State {
                    message: "Hello, the reducer has been replaced!",
                },
                ActionType::Goodbye => State {
                    message: "Goodbye for real.",
                },
            }
        }

        my_store.replace_reducer(replacing_reducer);
        my_store.dispatch(ActionType::Hello);
        my_store.dispatch(ActionType::Goodbye);
        assert_eq!(2 + 2, 4);
    }
}
