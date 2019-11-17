use std::sync::{Mutex, Arc};
use crate::core::{
    math::vec2::Vec2,
    pool::Handle
};
use crate::{
    gui::{
        node::UINode,
        Draw,
        draw::{DrawingContext, CommandKind, CommandTexture},
        widget::{Widget, AsWidget},
        Layout,
        UserInterface,
        Update,
        widget::WidgetBuilder
    },
    resource::texture::Texture,
};

pub struct Image {
    widget: Widget,
    texture: Option<Arc<Mutex<Texture>>>,
}

impl AsWidget for Image {
    fn widget(&self) -> &Widget {
        &self.widget
    }

    fn widget_mut(&mut self) -> &mut Widget {
        &mut self.widget
    }
}

impl Draw for Image {
    fn draw(&mut self, drawing_context: &mut DrawingContext) {
        let bounds = self.widget.get_screen_bounds();
        drawing_context.push_rect_filled(&bounds, None, self.widget.color());
        let texture = self.texture.as_ref().map_or(CommandTexture::None,
                                          |t| { CommandTexture::Texture(t.clone()) });
        drawing_context.commit(CommandKind::Geometry, texture);
    }
}

impl Layout for Image {
    fn measure_override(&self, ui: &UserInterface, available_size: Vec2) -> Vec2 {
        self.widget.measure_override(ui, available_size)
    }

    fn arrange_override(&self, ui: &UserInterface, final_size: Vec2) -> Vec2 {
        self.widget.arrange_override(ui, final_size)
    }
}

impl Update for Image {
    fn update(&mut self, dt: f32) {
        self.widget.update(dt)
    }
}

pub struct ImageBuilder {
    widget_builder: WidgetBuilder,
    texture: Option<Arc<Mutex<Texture>>>,
}

impl ImageBuilder {
    pub fn new(widget_builder: WidgetBuilder) -> Self {
        Self {
            widget_builder,
            texture: None,
        }
    }

    pub fn with_texture(mut self, texture: Arc<Mutex<Texture>>) -> Self {
        self.texture = Some(texture);
        self
    }

    pub fn with_opt_texture(mut self, texture: Option<Arc<Mutex<Texture>>>) -> Self {
        self.texture = texture;
        self
    }

    pub fn build(self, ui: &mut UserInterface) -> Handle<UINode> {
        let image = UINode::Image(Image {
            widget: self.widget_builder.build(),
            texture: self.texture,
        });

        ui.add_node(image)
    }
}