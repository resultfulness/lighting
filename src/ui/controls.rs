use egui::{Button, CollapsingHeader, Slider, Ui};

use crate::giraffeics::{
    render::{
        light::Light,
        material::ALL_MATERIALS_WITH_NAMES,
        mesh::Mesh,
        mesh_gen::{gen_cube, gen_plane, gen_sphere},
        object::Object,
    },
    vao::VertexArrayObject,
};

pub fn presets(ui: &mut Ui, object: &mut Object) {
    CollapsingHeader::new("Presets").show(ui, |ui| {
        let column_count = 2;

        egui::Grid::new("material-grid")
            .spacing([4., 4.])
            .show(ui, |ui| {
                for (i, (name, mat)) in
                    ALL_MATERIALS_WITH_NAMES.iter().enumerate()
                {
                    let fill = [ui.available_width(), ui.available_height()];
                    if ui.add_sized(fill, Button::new(*name)).clicked() {
                        object.material.set_properties(*mat);
                    };

                    if i % column_count == column_count - 1 {
                        ui.end_row();
                    }
                }
            });
    });
}

pub fn custom_material(ui: &mut Ui, object: &mut Object) {
    let ambient = &mut object.material.properties.ambient;
    let diffuse = &mut object.material.properties.diffuse;
    let specular = &mut object.material.properties.specular;

    CollapsingHeader::new("Custom").show(ui, |ui| {
        ui.add(Slider::new(&mut ambient.x, 0.0..=1.0).text("ambient red"));
        ui.add(Slider::new(&mut ambient.y, 0.0..=1.0).text("ambient green"));
        ui.add(Slider::new(&mut ambient.z, 0.0..=1.0).text("ambient blue"));
        ui.add(Slider::new(&mut diffuse.x, 0.0..=1.0).text("diffuse red"));
        ui.add(Slider::new(&mut diffuse.y, 0.0..=1.0).text("diffuse green"));
        ui.add(Slider::new(&mut diffuse.z, 0.0..=1.0).text("diffuse blue"));
        ui.add(Slider::new(&mut specular.x, 0.0..=1.0).text("specular red"));
        ui.add(Slider::new(&mut specular.y, 0.0..=1.0).text("specular green"));
        ui.add(Slider::new(&mut specular.z, 0.0..=1.0).text("specular blue"));
        ui.add(
            Slider::new(&mut object.material.properties.shininess, 1.0..=128.0)
                .text("shininess"),
        );
    });
}

pub fn custom_light(ui: &mut Ui, light: &mut Light) {
    let ambient = &mut light.ambient;
    let diffuse = &mut light.diffuse;
    let specular = &mut light.specular;
    let pos = &mut light.position;

    CollapsingHeader::new("Light").show(ui, |ui| {
        ui.add(Slider::new(&mut ambient.x, 0.0..=1.0).text("ambient red"));
        ui.add(Slider::new(&mut ambient.y, 0.0..=1.0).text("ambient green"));
        ui.add(Slider::new(&mut ambient.z, 0.0..=1.0).text("ambient blue"));
        ui.add(Slider::new(&mut diffuse.x, 0.0..=1.0).text("diffuse red"));
        ui.add(Slider::new(&mut diffuse.y, 0.0..=1.0).text("diffuse green"));
        ui.add(Slider::new(&mut diffuse.z, 0.0..=1.0).text("diffuse blue"));
        ui.add(Slider::new(&mut specular.x, 0.0..=1.0).text("specular red"));
        ui.add(Slider::new(&mut specular.y, 0.0..=1.0).text("specular green"));
        ui.add(Slider::new(&mut specular.z, 0.0..=1.0).text("specular blue"));
        ui.add(Slider::new(&mut pos.x, -10.0..=10.0).text("position x"));
        ui.add(Slider::new(&mut pos.y, -10.0..=10.0).text("position y"));
        ui.add(Slider::new(&mut pos.z, -10.0..=10.0).text("position z"));
    });
}

pub fn meshes<'a>(
    ui: &mut Ui,
    object: &mut Object<'a>,
    vao: &'a VertexArrayObject,
) {
    CollapsingHeader::new("Mesh").show(ui, |ui| {
        egui::Grid::new("mesh-grid")
            .spacing([4., 4.])
            .show(ui, |ui| {
                let fill = [ui.available_width(), ui.available_height()];
                if ui.add_sized(fill, Button::new("sphere")).clicked() {
                    let (vertices, indices) = gen_sphere(1., 32, 32);
                    let mesh = Mesh::new(&vao, vertices, indices);
                    if let Ok(mesh) = mesh {
                        object.mesh = mesh;
                    }
                }
                ui.end_row();
                if ui.add_sized(fill, Button::new("cube")).clicked() {
                    let (vertices, indices) = gen_cube(1.);
                    let mesh = Mesh::new(&vao, vertices, indices);
                    if let Ok(mesh) = mesh {
                        object.mesh = mesh;
                    }
                }
                ui.end_row();
                if ui.add_sized(fill, Button::new("plane")).clicked() {
                    let (vertices, indices) = gen_plane(1.);
                    let mesh = Mesh::new(&vao, vertices, indices);
                    if let Ok(mesh) = mesh {
                        object.mesh = mesh;
                    }
                }
                ui.end_row();
            });
    });
}
