
use crate::renderer::Renderer;
use crate::error::Error;

use image::GenericImageView;


pub struct Texture {

    texture: wgpu::Texture,
    view:    wgpu::TextureView,
    sampler: wgpu::Sampler
}


impl Texture {


    #[must_use]
    pub fn get_texture(&self) -> &wgpu::Texture { &self.texture }


    #[must_use]
    pub fn get_view(&self) -> &wgpu::TextureView { &self.view }


    #[must_use]
    pub fn get_sampler(&self) -> &wgpu::Sampler { &self.sampler }


    /// # Errors
    ///
    /// Returns error if loading of image failed
    pub fn from_bytes(
        renderer: &Renderer,
        bytes:    &[u8],
        label:    Option<&str>
    ) -> Result<Self, Error> {

        let image = match image::load_from_memory(bytes) {
            Ok(image) => image,
            Err(err)  => return Err(Error::FailedToLoadImage(err.to_string()))
        };

        Ok(Self::from_image(renderer, &image, label))
    }


    #[must_use]
    pub fn from_image(
        renderer: &Renderer,
        image:    &image::DynamicImage,
        label:    Option<&str>
    ) -> Self {

        let texture = Self::create_texture(renderer, image, label);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = Self::create_sampler(renderer.get_device());

        Self { texture, view, sampler }
    }


    fn create_texture(
        renderer: &Renderer,
        image:    &image::DynamicImage,
        label:    Option<&str>
    ) -> wgpu::Texture {

        let rgba_image = image.to_rgba8();
        let dimensions = image.dimensions();

        let size = wgpu::Extent3d {
            width:                 dimensions.0,
            height:                dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = renderer.get_device().create_texture(
            &wgpu::TextureDescriptor {
                label,
                size,
                mip_level_count: 1,
                sample_count:    1,
                dimension:       wgpu::TextureDimension::D2,
                format:          wgpu::TextureFormat::Rgba8UnormSrgb,
                usage:           wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats:    &[]
            }
        );

        renderer.get_queue().write_texture(
            wgpu::TexelCopyTextureInfo {
                aspect:    wgpu::TextureAspect::All,
                texture:   &texture,
                mip_level: 0,
                origin:    wgpu::Origin3d::ZERO
            },
            &rgba_image,
            wgpu::TexelCopyBufferLayout {
                offset:         0,
                bytes_per_row:  Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1)
            },
            size
        );

        texture
    }


    fn create_sampler(
        device: &wgpu::Device
    ) -> wgpu::Sampler {

        device.create_sampler(
            &wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter:     wgpu::FilterMode::Linear,
                min_filter:     wgpu::FilterMode::Nearest,
                mipmap_filter:  wgpu::MipmapFilterMode::Nearest,
                ..Default::default()
            }
        )
    }


    #[must_use]
    pub fn get_default_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {

        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding:    0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty:         wgpu::BindingType::Texture {
                        multisampled:   false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type:    wgpu::TextureSampleType::Float { filterable: true }
                    },
                    count: None
                },
                wgpu::BindGroupLayoutEntry {
                    binding:    1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty:         wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count:      None
                }
            ],
            label: Some("Default texture bind group layout")
        })
    }
}
