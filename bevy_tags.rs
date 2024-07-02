#[macro_export]
macro_rules! bevy_tags {
    ( $($tag:ident),* ) => {
        $(
            #[derive(Component)]
            pub struct $tag;
        )*
    };
}