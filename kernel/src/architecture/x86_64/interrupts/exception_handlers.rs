macro_rules! exception_handler {
    (fn $name:ident() => $message:expr) => {
        pub extern "x86-interrupt" fn $name() {
            log::debug!("In {}: {}", stringify!($name), $message);
        }
    };
}

exception_handler!(fn breakpoint_handler() => "BREAKPOINT");
