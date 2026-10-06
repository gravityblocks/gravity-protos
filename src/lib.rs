#![allow(warnings)]
// To regenerate the checked-in bindings, run:
// REGENERATE_PROTO=1 cargo build -p gravity-protos

pub mod auth {
    include!("generated/auth.rs");
}

pub mod packet {
    include!("generated/packet.rs");
}

pub mod shared {
    include!("generated/shared.rs");
}

pub mod bundle {
    include!("generated/bundle.rs");
}

pub mod searcher {
    include!("generated/searcher.rs");
}
