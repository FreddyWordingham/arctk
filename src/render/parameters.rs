//! Input configuration.

use palette::{Gradient, LinSrgba};
use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf};

use crate::{
    dom::{Surface, SurfaceBuilder, Tree, TreeBuilder},
    geom::Mesh,
    parse::{json, wavefront},
    render::{Attribute, AttributeBuilder, GradientBuilder, Settings, Shader, ShaderBuilder},
    rt::{Camera, CameraBuilder},
};

/// Input configuration.
#[derive(Deserialize)]
pub struct Parameters {
    /// Path to the top level resource directory.
    resources_dir: PathBuf,
    /// Oct-tree settings.
    tree: TreeBuilder,
    /// Technical settings.
    settings: Settings,
    /// Aesthetic settings.
    shader: ShaderBuilder,
    /// Main camera.
    camera: CameraBuilder,
    /// Surfaces.
    surfaces: Vec<SurfaceBuilder>,
}

impl Parameters {
    /// Get the names of the `Gradient`s used.
    #[inline]
    #[must_use]
    pub fn used_gradient_names(&self) -> Vec<String> {
        let mut gradient_names = Vec::new();

        gradient_names.extend(self.shader.used_gradient_names());
        gradient_names.extend(&mut self.used_attribute_names().iter().flat_map(|n| {
            json::load::<AttributeBuilder>(
                &self
                    .resources_dir
                    .join("attributes")
                    .join(n)
                    .with_extension("json"),
            )
            .used_gradient_names()
        }));

        gradient_names.sort();
        gradient_names.dedup();

        gradient_names
    }

    /// Get the names of the `Attribute`s used.
    #[inline]
    #[must_use]
    pub fn used_attribute_names(&self) -> Vec<String> {
        let mut names = Vec::new();

        for surf in &self.surfaces {
            names.push(surf.1.clone());
        }

        names.sort();
        names.dedup();

        names
    }

    /// Get the names of the `Meshes`s used.
    #[inline]
    #[must_use]
    pub fn used_mesh_names(&self) -> Vec<String> {
        let mut names = Vec::new();

        for surf in &self.surfaces {
            names.push(surf.0.clone());
        }

        names.sort();
        names.dedup();

        names
    }

    /// Load the dictionary of `Gradients`.
    #[inline]
    #[must_use]
    pub fn load_gradients(&self) -> HashMap<String, Gradient<LinSrgba>> {
        let mut grads = HashMap::new();

        for name in self.used_gradient_names() {
            let grad = json::load::<GradientBuilder>(
                &self
                    .resources_dir
                    .join("gradients")
                    .join(name.clone())
                    .with_extension("json"),
            )
            .build();
            grads.insert(name, grad);
        }

        grads
    }

    /// Load the dictionary of `Attributes`.
    #[inline]
    #[must_use]
    pub fn load_attributes<'a>(
        &self,
        grads: &'a HashMap<String, Gradient<LinSrgba>>,
    ) -> HashMap<String, Attribute<'a>> {
        let mut attrs = HashMap::new();

        for name in self.used_attribute_names() {
            let attr = json::load::<AttributeBuilder>(
                &self
                    .resources_dir
                    .join("attributes")
                    .join(name.clone())
                    .with_extension("json"),
            )
            .build(grads);
            attrs.insert(name, attr);
        }

        attrs
    }

    /// Load the dictionary of `Meshes`.
    #[inline]
    #[must_use]
    pub fn load_meshes(&self) -> HashMap<String, Mesh> {
        let mut meshes = HashMap::new();

        for name in self.used_mesh_names() {
            let mesh = wavefront::load(
                &self
                    .resources_dir
                    .join("meshes")
                    .join(name.clone())
                    .with_extension("obj"),
            );
            meshes.insert(name, mesh);
        }

        meshes
    }

    /// Load the `Surface`s.
    #[inline]
    #[must_use]
    pub fn load_surfaces<'a>(
        &self,
        meshes: &HashMap<String, Mesh>,
        attributes: &'a HashMap<String, Attribute>,
    ) -> Vec<Surface<Attribute<'a>>> {
        self.surfaces
            .iter()
            .map(|s| s.clone().build(meshes, attributes))
            .collect()
    }

    /// Build the `Shader`.
    #[inline]
    #[must_use]
    pub fn build_shader<'a>(&self, grads: &'a HashMap<String, Gradient<LinSrgba>>) -> Shader<'a> {
        self.shader.build(grads)
    }

    /// Build the `Settings`.
    #[inline]
    #[must_use]
    pub fn build_settings(&self) -> Settings {
        self.settings.clone()
    }

    /// Build the `Camera`s.
    #[inline]
    #[must_use]
    pub fn build_camera(&self) -> Camera {
        self.camera.clone().build()
    }

    /// Build the `Tree`.
    #[inline]
    #[must_use]
    pub fn build_tree<'a, T>(&self, surfs: &'a [Surface<T>]) -> Tree<'a, T> {
        self.tree.build(surfs)
    }
}
