// pub type Callback = dyn FnMut(KeyState) -> ();

#[derive(Hash, Clone, Copy, Debug, Eq, PartialEq)]
pub enum Key {
    Space,
    ArrowUp,
    ArrowRight,
    ArrowDown,
    ArrowLeft,
}

impl Key {
    pub fn from_event_key(s: &str) -> Option<Key> {
        match s {
            "Space" => Some(Key::Space),
            "ArrowUp" => Some(Key::ArrowUp),
            "ArrowRight" => Some(Key::ArrowRight),
            "ArrowDown" => Some(Key::ArrowDown),
            "ArrowLeft" => Some(Key::ArrowLeft),
            // Pause, quit
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KeyState {
    Pressed,
    Released,
}

impl KeyState {
    pub fn from_event_type(event_type: String) -> Self {
        if event_type == "keydown" {
            KeyState::Pressed
        } else {
            KeyState::Released
        }
    }
}

// pub struct Keyboard {
//     key_states: Rc<RefCell<HashMap<Key, KeyState>>>,
//     key_map: Rc<RefCell<HashMap<Key, Box<Callback>>>>,
// }
//
// impl Keyboard {
//     pub fn new() -> Self {
//         let key_states = Rc::new(RefCell::new(HashMap::default()));
//         let key_map = Rc::new(RefCell::new(HashMap::default()));
//         Self { key_states, key_map }
//     }

// pub fn add_mapping(&mut self, key: &str, callback: Box<Callback>) {
//     self.key_map.borrow_mut().insert(String::from(key), callback);
// }
/*
pub fn listen_to<T: 'static>(&mut self, event_type: &str, callback: Rc<RefCell<T>>)
    where T: FnMut(Key, KeyState) -> () {
    let states = self.key_states.clone();
    let cb = callback.clone();

    let closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        let key = event.code() as Key;
        let state = KeyState::from_event_type(event.type_());

        // let m = mapping.borrow();
        // let opt = m.get(&key);
        // match opt {
        //     None => event.prevent_default(),
        //     Some(callback) => {
        let upd = states.borrow_mut().insert(key.clone(), state);
        if upd != Some(state) {
            cb.borrow_mut()(key, state);
        }
        //     }
        // }
    }) as Box<dyn FnMut(_)>);

    window().add_event_listener_with_callback(event_type, closure.as_ref().unchecked_ref())
        .expect("Cannot listen the event");

    closure.forget();
}*/
// }
