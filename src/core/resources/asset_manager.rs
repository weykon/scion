use std::collections::HashMap;
use crate::core::components::material::Material2D;
use std::marker::PhantomData;
use legion::storage::Component;

/// `AssetManager` is resource that will link assets to an asset ref to allow reusability of assets
#[derive(Default)]
pub struct AssetManager {
    materials: HashMap<usize, Material2D>
}

impl AssetManager {
    pub(crate) fn get_material_for_ref(&self, asset_ref: &AssetRef<Material2D>) -> Material2D{
        self.materials
            .get(&asset_ref.0)
            .expect("An asset has been requested but does not exist")
            .clone()
    }

    pub fn register_material(&mut self, material: Material2D) -> AssetRef<Material2D> {
        let next_ref = AssetRef(self.materials.keys().count(), PhantomData::default());
        self.materials.insert(next_ref.0, material);
        next_ref
    }
}

#[derive(Clone)]
pub struct AssetRef<T>(pub(crate) usize, PhantomData<T>) where T:Component;

#[cfg(test)]
mod tests {
    use crate::core::resources::asset_manager::AssetManager;
    use crate::core::components::material::Material2D;
    use crate::core::components::color::Color;

    #[test]
    fn register_material_test() {
        let mut manager = AssetManager::default();
        let asset_ref = manager.register_material(Material2D::Color(Color::new(1,1,1,1.)));
        assert_eq!(0, asset_ref.0);
        assert_eq!(1, manager.materials.len());

        let asset_ref = manager.register_material(Material2D::Color(Color::new(2,2,2,1.)));
        assert_eq!(1, asset_ref.0);
        assert_eq!(2, manager.materials.len());
    }
}