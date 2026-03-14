pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    /*
        ? Why the lifetime ( 'a ) is required ?

        We store a *reference* to T (i.e. &T) instead of owning T directly.
        This means the compiler needs to know: "how long is this reference valid?"

        That's what the lifetime `'a` answers:
        "The reference `&T` must live at least as long as the LimitTracker that holds it."

        Without `'a`, the compiler has no way to guarantee that the thing we're
        pointing to hasn't been dropped while LimitTracker is still using it.

        > LimitTracker cannot outlive the reference it holds.
    */
    messenger: &'a T, // will throw error if no lifetime specified to &T
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    struct MockMessenger {
        // sent_messages: Vec<String>, // ! ❌ (see more details below)
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                // sent_messages: vec![], // ! ❌ (see more details below)
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            // self.sent_messages.push(String::from(msg));
            // ! 👆🏻❌ since `send` method (of Messenger) takes an immutable reference to self.
            // This is now where the interior mutability pattern comes in (see src/main.rs for more details)
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75p_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
