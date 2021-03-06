// FIXME: missing sysroot spans (#53081)
// ignore-i586-unknown-linux-gnu
// ignore-i586-unknown-linux-musl
// ignore-i686-unknown-linux-musl

#![feature(never_type)]
#![feature(exhaustive_patterns)]

mod private {
    pub struct Private {
        _bot: !,
        pub misc: bool,
    }
    pub const DATA: Option<Private> = None;
}

fn main() {
    match private::DATA {
    //~^ ERROR non-exhaustive patterns: `Some(Private { misc: true, .. })` not covered
        None => {}
        Some(private::Private {
            misc: false,
            ..
        }) => {}
    }
}
