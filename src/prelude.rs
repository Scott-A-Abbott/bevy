pub use crate::{
    app::prelude::*, asset::prelude::*, core::prelude::*, ecs::prelude::*, input::prelude::*,
    math::prelude::*, pbr::prelude::*, property::prelude::*, render::prelude::*, scene::prelude::*,
    shaded_sprite::prelude::*, sprite::prelude::*, text::prelude::*, transform::prelude::*,
    type_registry::RegisterType, ui::prelude::*, window::prelude::*, AddDefaultPlugins,
};

#[cfg(feature = "bevy_audio")]
pub use crate::audio::prelude::*;

#[cfg(feature = "bevy_pbr")]
pub use crate::pbr::prelude::*;

#[cfg(feature = "bevy_render")]
pub use crate::render::prelude::*;

#[cfg(feature = "bevy_sprite")]
pub use crate::sprite::prelude::*;

#[cfg(feature = "bevy_text")]
pub use crate::text::prelude::*;

#[cfg(feature = "bevy_ui")]
pub use crate::ui::prelude::*;
