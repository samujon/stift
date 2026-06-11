use stift_core::{Brush, Canvas, StrokePoint};
use wgpu::{Device, Queue, RenderPass, TextureFormat, RenderPipeline};

pub trait Renderer {
    fn create_pipeline(device: &Device, format: TextureFormat) -> RenderPipeline;
    fn update_buffers(&self, device: &Device, queue: &Queue);
    fn draw<'a>(&'a self, render_pass: &mut RenderPass<'a>);
}

#[derive(Debug)]
pub struct GpuRenderer {
    pipeline: RenderPipeline,
}

impl GpuRenderer {
    pub fn new(device: &Device, format: TextureFormat) -> Self {
        let pipeline = Self::create_pipeline(device, format);

        Self { pipeline }
    }
}

impl Renderer for GpuRenderer {

    fn create_pipeline(_device: &Device, _format: TextureFormat) -> RenderPipeline {
        todo!("Implement pipeline creation")
    }

    fn update_buffers(&self, _device: &Device, _queue: &Queue) {
        todo!("Implement buffer updates")
    }

    fn draw<'a>(&'a self, _render_pass: &mut RenderPass<'a>) {
        RenderPass::set_pipeline(_render_pass, &self.pipeline);
    }


}
