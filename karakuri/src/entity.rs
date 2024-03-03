use crate::engine::RenderData;
use crate::math::Vector2;
use crate::utils::Color;

pub trait EntityTrait {
    fn on_start(&mut self);
    fn on_step(&mut self, delta_time: f64);
}

pub trait EntityAddon {
    fn on_start(&mut self);
    fn on_step(&mut self, delta_time: f64);
    fn render_data<'a>(&self, render_data: &'a mut RenderData) -> &'a mut RenderData;
}
pub struct SpatialAddon {
    pub position: Vector2,
    pub size: Vector2,
    pub color: Color,
}
impl EntityAddon for SpatialAddon {
    fn on_start(&mut self) {
        println!("SPATIAL START");
    }

    fn on_step(&mut self, _delta_time: f64) {
        println!("SPATIAL ON STEP");
    }

    fn render_data<'a>(&self, render_data: &'a mut RenderData) -> &'a mut RenderData {
        render_data.position = self.position;
        render_data.size = self.size;
        render_data.color = self.color;
        render_data
    }
}

pub struct EntityItem {
    entity: Box<dyn EntityTrait>,
    addons: Vec<Box<dyn EntityAddon>>,
}
impl EntityItem {
    pub fn init(mut entity: impl EntityTrait + 'static) -> Self {
        entity.on_start();
        Self {
            entity: Box::new(entity),
            addons: vec![],
        }
    }

    pub fn register_addon(mut self, mut addon: impl EntityAddon + 'static) -> Self {
        addon.on_start();
        self.addons.push(Box::new(addon));
        self
    }

    pub fn render_data(&self) -> RenderData {
        let mut render_data = RenderData::default();
        for addon in self.addons.iter() {
            addon.render_data(&mut render_data);
        }
        render_data
    }

    pub fn on_step(&mut self, delta_time: f64) {
        self.entity.on_step(delta_time);
        for addon in self.addons.iter_mut() {
            addon.on_step(delta_time);
        }
    }
}
