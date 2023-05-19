use bevy::pbr::{AlphaMode, Material};
use bevy::prelude::Color;
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "e690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct LandMaterial {
    #[uniform(0)]
    pub color: Color,
    #[uniform(1)]
    pub offset: f32,
    pub alpha_mode: AlphaMode,
}

impl Material for LandMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/land.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/land.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-1225-57e2b3f056e0"]
pub struct ShipMaterial {
    #[uniform(0)]
    pub color: Color,
    pub alpha_mode: AlphaMode,
}

impl Material for ShipMaterial {
    // fn vertex_shader() -> ShaderRef {
    //     "shaders/ship.wgsl".into()
    // }

    fn fragment_shader() -> ShaderRef {
        "shaders/ship.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}
