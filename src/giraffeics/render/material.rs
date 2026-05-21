use nalgebra::Vector3;

use crate::giraffeics::shader::ShaderProgram;

#[derive(Copy, Clone)]
pub struct MaterialProperties {
    pub ambient: Vector3<f32>,
    pub diffuse: Vector3<f32>,
    pub specular: Vector3<f32>,
    pub shininess: f32,
}

impl MaterialProperties {
    pub const fn new_from_scalars(
        ambient_r: f32,
        ambient_g: f32,
        ambient_b: f32,
        diffuse_r: f32,
        diffuse_g: f32,
        diffuse_b: f32,
        specular_r: f32,
        specular_g: f32,
        specular_b: f32,
        shininess: f32,
    ) -> Self {
        Self {
            ambient: Vector3::new(ambient_r, ambient_g, ambient_b),
            diffuse: Vector3::new(diffuse_r, diffuse_g, diffuse_b),
            specular: Vector3::new(specular_r, specular_g, specular_b),
            shininess: shininess * 128.,
        }
    }

    pub const EMERALD: Self = Self::new_from_scalars(
        0.0215, 0.1745, 0.0215, 0.07568, 0.61424, 0.07568, 0.633, 0.727811,
        0.633, 0.6,
    );
    pub const JADE: Self = Self::new_from_scalars(
        0.135, 0.2225, 0.1575, 0.54, 0.89, 0.63, 0.316228, 0.316228, 0.316228,
        0.1,
    );
    pub const OBSIDIAN: Self = Self::new_from_scalars(
        0.05375, 0.05, 0.06625, 0.18275, 0.17, 0.22525, 0.332741, 0.328634,
        0.346435, 0.3,
    );
    pub const PEARL: Self = Self::new_from_scalars(
        0.25, 0.20725, 0.20725, 1., 0.829, 0.829, 0.296648, 0.296648, 0.296648,
        0.088,
    );
    pub const RUBY: Self = Self::new_from_scalars(
        0.1745, 0.01175, 0.01175, 0.61424, 0.04136, 0.04136, 0.727811,
        0.626959, 0.626959, 0.6,
    );
    pub const TURQUOISE: Self = Self::new_from_scalars(
        0.1, 0.18725, 0.1745, 0.396, 0.74151, 0.69102, 0.297254, 0.30829,
        0.306678, 0.1,
    );
    pub const BRASS: Self = Self::new_from_scalars(
        0.329412, 0.223529, 0.027451, 0.780392, 0.568627, 0.113725, 0.992157,
        0.941176, 0.807843, 0.21794872,
    );
    pub const BRONZE: Self = Self::new_from_scalars(
        0.2125, 0.1275, 0.054, 0.714, 0.4284, 0.18144, 0.393548, 0.271906,
        0.166721, 0.2,
    );
    pub const CHROME: Self = Self::new_from_scalars(
        0.25, 0.25, 0.25, 0.4, 0.4, 0.4, 0.774597, 0.774597, 0.774597, 0.6,
    );
    pub const COPPER: Self = Self::new_from_scalars(
        0.19125, 0.0735, 0.0225, 0.7038, 0.27048, 0.0828, 0.256777, 0.137622,
        0.086014, 0.1,
    );
    pub const GOLD: Self = Self::new_from_scalars(
        0.24725, 0.1995, 0.0745, 0.75164, 0.60648, 0.22648, 0.628281, 0.555802,
        0.366065, 0.4,
    );
    pub const SILVER: Self = Self::new_from_scalars(
        0.19225, 0.19225, 0.19225, 0.50754, 0.50754, 0.50754, 0.508273,
        0.508273, 0.508273, 0.4,
    );
}

pub const ALL_MATERIALS_WITH_NAMES: [(&str, MaterialProperties); 12] = [
    ("emerald", MaterialProperties::EMERALD),
    ("jade", MaterialProperties::JADE),
    ("obsidian", MaterialProperties::OBSIDIAN),
    ("pearl", MaterialProperties::PEARL),
    ("ruby", MaterialProperties::RUBY),
    ("turquoise", MaterialProperties::TURQUOISE),
    ("brass", MaterialProperties::BRASS),
    ("bronze", MaterialProperties::BRONZE),
    ("chrome", MaterialProperties::CHROME),
    ("copper", MaterialProperties::COPPER),
    ("gold", MaterialProperties::GOLD),
    ("silver", MaterialProperties::SILVER),
];

pub struct Material<'a> {
    pub properties: MaterialProperties,
    pub shader_program: &'a ShaderProgram,
}

impl<'a> Material<'a> {
    pub fn new(
        properties: MaterialProperties,
        shader_program: &'a ShaderProgram,
    ) -> Self {
        Self {
            properties,
            shader_program,
        }
    }

    pub fn setup_shader(&self) {
        self.shader_program.use_program();

        self.shader_program
            .set_vec3("material.ambient", self.properties.ambient);
        self.shader_program
            .set_vec3("material.diffuse", self.properties.diffuse);
        self.shader_program
            .set_vec3("material.specular", self.properties.specular);
        self.shader_program
            .set_float("material.shininess", self.properties.shininess);
    }

    pub fn set_properties(&mut self, properties: MaterialProperties) {
        self.properties = properties;
    }
}
