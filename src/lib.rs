#[allow(warnings)]
mod bindings;

use bindings::Guest;

struct Component;

impl Guest for Component {
    fn shout(input: String) -> String {
        let mut s = input.to_uppercase();
        s.push_str("!");
        s.into()
    }
}

bindings::export!(Component with_types_in bindings);
