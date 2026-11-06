//! G1-SLICE.13: hosts ADR-0002's native topologies so they can be measured. An experiment, not product code.
//!
//! `dom-canvas` and `webgpu-webview` load the spike's own page in the system webview, so those two rows run the same
//! code as the browser rows. `native-wgpu-composited` draws the corpus with wgpu in this process, with a small HTML
//! toolbar composed over it in a child webview, which is the composition seam that topology has to survive. It measures
//! what the page measures, by the same definitions.
use sc_viewport::{corpus, pick, snap, Corpus, Point, View, ZOOMS};
use std::io::Write as _;
use std::net::TcpStream;
use std::time::Instant;
use tao::dpi::LogicalSize;
use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::window::{Window, WindowBuilder};
use wry::dpi::{LogicalPosition, LogicalSize as WryLogicalSize};
use wry::{Rect, WebViewBuilder};

const WIDTH: u32 = 1600;
const HEIGHT: u32 = 1000;
const SEED: u64 = 7;
const FRAMES: usize = 600;
const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8Unorm;

const SHADER: &str = "
struct Uniforms { scale: f32, x: f32, y: f32, width: f32, height: f32 };
@group(0) @binding(0) var<uniform> u: Uniforms;
struct Out { @builtin(position) position: vec4f, @location(0) uv: vec2f };
fn place(pixel: vec2f) -> vec4f {
  return vec4f(pixel.x / u.width * 2.0 - 1.0, 1.0 - pixel.y / u.height * 2.0, 0.0, 1.0);
}
@vertex fn line(@location(0) world: vec2f) -> @builtin(position) vec4f {
  return place(vec2f(world.x * u.scale - u.x, world.y * u.scale - u.y));
}
@fragment fn ink() -> @location(0) vec4f { return vec4f(0.0, 0.0, 0.0, 1.0); }
@vertex fn glyph(@location(0) pixel: vec2f, @location(1) uv: vec2f) -> Out {
  return Out(place(pixel), uv);
}
@group(0) @binding(1) var atlas: texture_2d<f32>;
@group(0) @binding(2) var nearest: sampler;
@fragment fn sampled(input: Out) -> @location(0) vec4f {
  let alpha = textureSample(atlas, nearest, input.uv).r;
  return vec4f(0.0, 0.0, 0.0, alpha);
}";

// The digits and the two letters the labels use, 5 pixels wide and 7 tall, one byte per row.
const FONT: [(char, [u8; 7]); 12] = [
    ('0', [14, 17, 19, 21, 25, 17, 14]),
    ('1', [4, 12, 4, 4, 4, 4, 14]),
    ('2', [14, 17, 1, 2, 4, 8, 31]),
    ('3', [30, 1, 1, 14, 1, 1, 30]),
    ('4', [2, 6, 10, 18, 31, 2, 2]),
    ('5', [31, 16, 30, 1, 1, 17, 14]),
    ('6', [6, 8, 16, 30, 17, 17, 14]),
    ('7', [31, 1, 2, 4, 8, 8, 8]),
    ('8', [14, 17, 17, 14, 17, 17, 14]),
    ('9', [14, 17, 17, 15, 1, 2, 12]),
    ('P', [30, 17, 17, 30, 16, 16, 16]),
    ('.', [0, 0, 0, 0, 0, 12, 12]),
];

fn post(base: &str, body: &str) {
    let address = base.trim_start_matches("http://");
    if let Ok(mut stream) = TcpStream::connect(address) {
        let request = format!(
            "POST /result HTTP/1.1\r\nHost: {address}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        );
        let _ = stream.write_all(request.as_bytes());
    }
}

fn percentile(values: &[f64], share: f64) -> f64 {
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    let at = ((sorted.len() as f64) * share) as usize;
    sorted[at.min(sorted.len().saturating_sub(1))]
}

struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    multisample: wgpu::Texture,
    resolved: wgpu::Texture,
    readback: wgpu::Buffer,
    uniforms: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    line_pipeline: wgpu::RenderPipeline,
    fill_pipeline: wgpu::RenderPipeline,
    glyph_pipeline: wgpu::RenderPipeline,
    line_buffer: wgpu::Buffer,
    line_count: u32,
    marker_buffer: wgpu::Buffer,
    glyph_buffer: wgpu::Buffer,
    labels: Vec<(u32, f32, f32)>,
    glyph_data: Vec<f32>,
}

fn floats(bytes: &[f32]) -> Vec<u8> {
    bytes.iter().flat_map(|value| value.to_le_bytes()).collect()
}

impl Renderer {
    fn new(window: &'static Window, set: &Corpus) -> Self {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let surface = instance.create_surface(window).expect("a surface");
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .expect("an adapter");
        let (device, queue) =
            pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default()))
                .expect("a device");
        surface.configure(
            &device,
            &wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_DST,
                format: FORMAT,
                width: WIDTH,
                height: HEIGHT,
                present_mode: wgpu::PresentMode::AutoNoVsync,
                desired_maximum_frame_latency: 2,
                alpha_mode: wgpu::CompositeAlphaMode::Auto,
                view_formats: vec![],
            },
        );
        let extent = wgpu::Extent3d {
            width: WIDTH,
            height: HEIGHT,
            depth_or_array_layers: 1,
        };
        let texture = |samples: u32, usage: wgpu::TextureUsages| {
            device.create_texture(&wgpu::TextureDescriptor {
                label: None,
                size: extent,
                mip_level_count: 1,
                sample_count: samples,
                dimension: wgpu::TextureDimension::D2,
                format: FORMAT,
                usage,
                view_formats: &[],
            })
        };
        let multisample = texture(4, wgpu::TextureUsages::RENDER_ATTACHMENT);
        let resolved = texture(
            1,
            wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        );
        let readback = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: u64::from(WIDTH) * u64::from(HEIGHT) * 4,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        let uniforms = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 32,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Lines: every outline edge, and the annotations' short marks.
        let mut lines: Vec<f32> = Vec::new();
        for piece in &set.pieces {
            let count = piece.outline.len();
            for at in 0..count {
                let (a, b) = (piece.outline[at], piece.outline[(at + 1) % count]);
                lines.extend([a.x as f32, a.y as f32, b.x as f32, b.y as f32]);
            }
            for mark in piece.annotations.iter().skip(4) {
                lines.extend([
                    mark.x as f32,
                    mark.y as f32,
                    mark.x as f32 + 500.0,
                    mark.y as f32 + 1000.0,
                ]);
            }
        }
        let line_count = (lines.len() / 2) as u32;
        let line_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: (lines.len() * 4) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        queue.write_buffer(&line_buffer, 0, &floats(&lines));
        let marker_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 48,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // The label atlas: 64 cells of 64 by 16 pixels, drawn from the bitmap font.
        let (atlas_width, atlas_height) = (512u32, 256u32);
        let mut atlas_bytes = vec![0u8; (atlas_width * atlas_height) as usize];
        let mut labels = Vec::new();
        for (number, piece) in set.pieces.iter().enumerate() {
            for at in 0..4 {
                let cell = labels.len() as u32;
                let (cell_x, cell_y) = ((cell % 8) * 64, (cell / 8) * 16);
                for (index, character) in format!("P{number}.{at}").chars().enumerate() {
                    let rows = FONT
                        .iter()
                        .find(|(known, _)| *known == character)
                        .map_or([0; 7], |(_, rows)| *rows);
                    for (row, bits) in rows.iter().enumerate() {
                        for column in 0..5u32 {
                            if bits & (16 >> column) != 0 {
                                let x = cell_x + index as u32 * 6 + column;
                                let y = cell_y + 4 + row as u32;
                                atlas_bytes[(y * atlas_width + x) as usize] = 255;
                            }
                        }
                    }
                }
                let mark = piece.annotations[at];
                labels.push((cell, mark.x as f32, mark.y as f32));
            }
        }
        let atlas = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: atlas_width,
                height: atlas_height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        queue.write_texture(
            atlas.as_image_copy(),
            &atlas_bytes,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(atlas_width),
                rows_per_image: None,
            },
            wgpu::Extent3d {
                width: atlas_width,
                height: atlas_height,
                depth_or_array_layers: 1,
            },
        );
        let glyph_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: (labels.len() * 24 * 4) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        let atlas_view = atlas.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor::default());
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniforms.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&atlas_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&layout],
            push_constant_ranges: &[],
        });
        let position = [wgpu::VertexBufferLayout {
            array_stride: 8,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x2],
        }];
        let glyph_vertex = [wgpu::VertexBufferLayout {
            array_stride: 16,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2],
        }];
        let pipeline = |vertex: &str,
                        fragment: &str,
                        buffers: &[wgpu::VertexBufferLayout],
                        topology,
                        blend| {
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &module,
                    entry_point: Some(vertex),
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                    buffers,
                },
                primitive: wgpu::PrimitiveState {
                    topology,
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 4,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                fragment: Some(wgpu::FragmentState {
                    module: &module,
                    entry_point: Some(fragment),
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: FORMAT,
                        blend,
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                multiview: None,
                cache: None,
            })
        };
        let line_pipeline = pipeline(
            "line",
            "ink",
            &position,
            wgpu::PrimitiveTopology::LineList,
            None,
        );
        let fill_pipeline = pipeline(
            "line",
            "ink",
            &position,
            wgpu::PrimitiveTopology::TriangleList,
            None,
        );
        let glyph_pipeline = pipeline(
            "glyph",
            "sampled",
            &glyph_vertex,
            wgpu::PrimitiveTopology::TriangleList,
            Some(wgpu::BlendState::ALPHA_BLENDING),
        );
        Self {
            device,
            queue,
            surface,
            multisample,
            resolved,
            readback,
            uniforms,
            bind_group,
            line_pipeline,
            fill_pipeline,
            glyph_pipeline,
            line_buffer,
            line_count,
            marker_buffer,
            glyph_buffer,
            labels,
            glyph_data: Vec::new(),
        }
    }

    fn set_view(&self, view: (f64, f64, f64)) {
        let data = [
            view.0 as f32,
            view.1 as f32,
            view.2 as f32,
            WIDTH as f32,
            HEIGHT as f32,
            0.0,
            0.0,
            0.0,
        ];
        self.queue.write_buffer(&self.uniforms, 0, &floats(&data));
    }

    fn pass<'e>(
        &self,
        encoder: &'e mut wgpu::CommandEncoder,
        resolve: &'e wgpu::TextureView,
        target: &'e wgpu::TextureView,
    ) -> wgpu::RenderPass<'e> {
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: Some(resolve),
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                    store: wgpu::StoreOp::Discard,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        })
    }

    fn copy_for_reading(&self, encoder: &mut wgpu::CommandEncoder) {
        encoder.copy_texture_to_buffer(
            self.resolved.as_image_copy(),
            wgpu::TexelCopyBufferInfo {
                buffer: &self.readback,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(WIDTH * 4),
                    rows_per_image: None,
                },
            },
            wgpu::Extent3d {
                width: WIDTH,
                height: HEIGHT,
                depth_or_array_layers: 1,
            },
        );
    }

    fn read_pixels(&self) -> Vec<u8> {
        let slice = self.readback.slice(..);
        slice.map_async(wgpu::MapMode::Read, |_| {});
        let _ = self.device.poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: None,
        });
        let pixels = slice.get_mapped_range().to_vec();
        self.readback.unmap();
        pixels
    }

    // Returns the milliseconds of this frame's own work. Waiting for the display to take a drawable, and presenting it,
    // are the display's pace rather than the renderer's, and the page's definition does not include them either.
    fn frame(&mut self, view: (f64, f64, f64), finish: bool) -> f64 {
        let frame = self
            .surface
            .get_current_texture()
            .expect("a surface texture");
        let started = Instant::now();
        self.set_view(view);
        self.glyph_data.clear();
        for (cell, world_x, world_y) in &self.labels {
            let (cell_x, cell_y) = ((cell % 8) as f32 * 64.0, (cell / 8) as f32 * 16.0);
            let left = *world_x * view.0 as f32 - view.1 as f32;
            let top = *world_y * view.0 as f32 - view.2 as f32 - 12.0;
            let (right, bottom) = (left + 64.0, top + 16.0);
            let (u0, v0, u1, v1) = (
                cell_x / 512.0,
                cell_y / 256.0,
                (cell_x + 64.0) / 512.0,
                (cell_y + 16.0) / 256.0,
            );
            self.glyph_data.extend([
                left, top, u0, v0, right, top, u1, v0, left, bottom, u0, v1, left, bottom, u0, v1,
                right, top, u1, v0, right, bottom, u1, v1,
            ]);
        }
        self.queue
            .write_buffer(&self.glyph_buffer, 0, &floats(&self.glyph_data));
        let resolved = self
            .resolved
            .create_view(&wgpu::TextureViewDescriptor::default());
        let target = self
            .multisample
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut pass = self.pass(&mut encoder, &resolved, &target);
            pass.set_bind_group(0, &self.bind_group, &[]);
            pass.set_pipeline(&self.line_pipeline);
            pass.set_vertex_buffer(0, self.line_buffer.slice(..));
            pass.draw(0..self.line_count, 0..1);
            pass.set_pipeline(&self.glyph_pipeline);
            pass.set_vertex_buffer(0, self.glyph_buffer.slice(..));
            pass.draw(0..(self.labels.len() as u32 * 6), 0..1);
        }
        if finish {
            self.copy_for_reading(&mut encoder);
        }
        self.queue.submit([encoder.finish()]);
        if finish {
            let _ = self.read_pixels();
        }
        let work = started.elapsed().as_secs_f64() * 1000.0;
        // A separate submission, because a drawable's copy waits for the display and would be timed as renderer work.
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        encoder.copy_texture_to_texture(
            self.resolved.as_image_copy(),
            frame.texture.as_image_copy(),
            wgpu::Extent3d {
                width: WIDTH,
                height: HEIGHT,
                depth_or_array_layers: 1,
            },
        );
        self.queue.submit([encoder.finish()]);
        frame.present();
        work
    }

    fn marker(
        &self,
        view: (f64, f64, f64),
        world_x: f64,
        world_y: f64,
        radius: f64,
    ) -> Option<(f64, f64)> {
        self.set_view(view);
        let half = radius / view.0;
        let (l, r, t, b) = (
            (world_x - half) as f32,
            (world_x + half) as f32,
            (world_y - half) as f32,
            (world_y + half) as f32,
        );
        self.queue.write_buffer(
            &self.marker_buffer,
            0,
            &floats(&[l, t, r, t, l, b, l, b, r, t, r, b]),
        );
        let resolved = self
            .resolved
            .create_view(&wgpu::TextureViewDescriptor::default());
        let target = self
            .multisample
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut pass = self.pass(&mut encoder, &resolved, &target);
            pass.set_bind_group(0, &self.bind_group, &[]);
            pass.set_pipeline(&self.fill_pipeline);
            pass.set_vertex_buffer(0, self.marker_buffer.slice(..));
            pass.draw(0..6, 0..1);
        }
        self.copy_for_reading(&mut encoder);
        self.queue.submit([encoder.finish()]);
        let pixels = self.read_pixels();
        let (mut total, mut sum_x, mut sum_y) = (0.0, 0.0, 0.0);
        for y in 0..HEIGHT as usize {
            for x in 0..WIDTH as usize {
                let dark = 255.0 - f64::from(pixels[(y * WIDTH as usize + x) * 4 + 1]);
                if dark > 0.0 {
                    total += dark;
                    sum_x += dark * (x as f64 + 0.5);
                    sum_y += dark * (y as f64 + 0.5);
                }
            }
        }
        (total > 0.0).then(|| (sum_x / total, sum_y / total))
    }
}

fn pan_to(view: View, point: Point) -> View {
    let (x, y) = view.pixel_of(point).expect("in range");
    view.pan(
        (x - f64::from(WIDTH) / 2.0).round() as i64,
        (y - f64::from(HEIGHT) / 2.0).round() as i64,
    )
    .expect("in range")
}

/// Correctness and fidelity, measured the way the page measures them.
fn correctness(renderer: &Renderer, set: &Corpus) -> (usize, usize, f64, f64) {
    let close = View::new((40, 1)).expect("a zoom");
    let through_screen = |at: Point| {
        let (x, y) = close.pixel_of(at).expect("in range");
        close
            .point_of_pixel(x.round() as i64, y.round() as i64)
            .expect("in range")
    };
    let picks = set
        .picks
        .iter()
        .filter(|probe| {
            pick(&set.pieces, through_screen(probe.at), 200)
                .is_some_and(|hit| (hit.piece, hit.edge) == (probe.piece, probe.entity))
        })
        .count();
    let snaps = set
        .snaps
        .iter()
        .filter(|probe| {
            snap(&set.pieces, through_screen(probe.at))
                .is_some_and(|hit| (hit.piece, hit.edge) == (probe.piece, probe.entity))
        })
        .count();
    let mut worst = 0.0_f64;
    for zoom in ZOOMS {
        for probe in set.picks.iter().take(5) {
            let view = pan_to(View::new(zoom).expect("a zoom"), probe.at);
            let (want_x, want_y) = view.pixel_of(probe.at).expect("in range");
            let found = renderer
                .marker(view.transform(), probe.at.x as f64, probe.at.y as f64, 6.0)
                .expect("a marker on screen");
            worst = worst
                .max((found.0 - want_x).abs())
                .max((found.1 - want_y).abs());
        }
    }
    let probe = set.picks[0].at;
    let start = pan_to(View::new((1, 1)).expect("a zoom"), probe);
    let first = renderer
        .marker(start.transform(), probe.x as f64, probe.y as f64, 6.0)
        .expect("a marker on screen");
    let mut view = start;
    for _ in 0..1000 {
        for zoom in ZOOMS.iter().chain(ZOOMS.iter().rev()) {
            view = view.zoom_about(*zoom, 400, 300).expect("in range");
        }
        view = view.zoom_about((1, 1), 400, 300).expect("in range");
    }
    let last = renderer
        .marker(view.transform(), probe.x as f64, probe.y as f64, 6.0)
        .expect("a marker on screen");
    let (a, b) = (start.transform(), view.transform());
    let drift = (a.0 - b.0)
        .abs()
        .max((a.1 - b.1).abs())
        .max((a.2 - b.2).abs())
        .max((first.0 - last.0).abs())
        .max((first.1 - last.1).abs());
    (picks, snaps, worst, drift)
}

fn run_native(event_loop: EventLoop<()>, window: &'static Window, base: String, mode: String) {
    let begun = Instant::now();
    let set = corpus(SEED);
    let mut renderer = Renderer::new(window, &set);
    let mut view = View::new((1, 5)).expect("a zoom");
    renderer.frame(view.transform(), false);
    let cold_start = begun.elapsed().as_secs_f64() * 1000.0;
    let measured = (mode == "timing").then(|| correctness(&renderer, &set));
    let mut frame = 0;
    let mut times = Vec::with_capacity(FRAMES);
    view = View::new((1, 5)).expect("a zoom");
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        if let Event::WindowEvent { event: WindowEvent::CloseRequested, .. } = event {
            *control_flow = ControlFlow::Exit;
        }
        if !matches!(event, Event::MainEventsCleared) {
            return;
        }
        if frame == FRAMES {
            let body = match measured {
                Some((picks, snaps, worst, drift)) => format!(
                    "{{\"topology\":\"native-wgpu-composited\",\"capable\":\"yes\",\"pick_correct\":\"{}\",\"snap_exact\":\"{}\",\"pick_right\":{picks},\"snap_right\":{snaps},\"fidelity_max_px\":{worst},\"fidelity_drift_px\":{drift},\"frame_p95_ms\":{},\"frame_median_ms\":{},\"cold_start_ms\":{cold_start},\"adapter\":\"wgpu\"}}",
                    if picks == 200 { "yes" } else { "no" },
                    if snaps == 200 { "yes" } else { "no" },
                    percentile(&times, 0.95),
                    percentile(&times, 0.5),
                ),
                None => "{\"memory_done\":true}".to_owned(),
            };
            post(&base, &body);
            frame += 1;
            *control_flow = ControlFlow::Exit;
            return;
        }
        if frame > FRAMES {
            return;
        }
        let zoom = ZOOMS[(frame / 20) % ZOOMS.len()];
        view = view.zoom_about(zoom, 800, 500).expect("in range");
        view = view.pan((frame % 40) as i64 - 20, (frame % 30) as i64 - 15).expect("in range");
        times.push(renderer.frame(view.transform(), measured.is_some()));
        frame += 1;
    });
}

fn main() {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let [topology, base, mode] = arguments.as_slice() else {
        eprintln!("usage: canvas-host <topology> <base-url> <timing|memory>");
        std::process::exit(2);
    };
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("StitchCAD canvas spike")
        .with_inner_size(LogicalSize::new(f64::from(WIDTH), f64::from(HEIGHT)))
        .build(&event_loop)
        .expect("a window");
    let window: &'static Window = Box::leak(Box::new(window));
    if topology == "native-wgpu-composited" {
        // The HTML chrome this topology must compose with: a toolbar over the wgpu surface.
        let toolbar = WebViewBuilder::new()
            .with_transparent(true)
            .with_html("<html><body style='margin:0;background:rgba(240,240,240,.8);font:13px sans-serif;padding:8px'>StitchCAD toolbar</body></html>")
            .with_bounds(Rect {
                position: LogicalPosition::new(0.0, 0.0).into(),
                size: WryLogicalSize::new(f64::from(WIDTH), 36.0).into(),
            })
            .build_as_child(window)
            .expect("a child webview");
        Box::leak(Box::new(toolbar));
        run_native(event_loop, window, base.clone(), mode.clone());
    } else {
        let url = format!("{base}/page.html?topology={topology}&mode={mode}");
        let webview = WebViewBuilder::new()
            .with_url(url)
            .build(window)
            .expect("a webview");
        Box::leak(Box::new(webview));
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            if let Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } = event
            {
                *control_flow = ControlFlow::Exit;
            }
        });
    }
}
