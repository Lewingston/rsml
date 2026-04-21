
use crate::drawable::texture::Texture;
use crate::error::Error;

use std::rc::Rc;


pub struct Font {

    font:     fontdue::Font,
    font_map: std::collections::HashMap<u32, FontTextureAtlas>
}


impl Font {

    /// # Errors
    ///
    /// Returns error if loading of font file failed.
    pub fn from_file(font_file_path: &str) -> Result<Self, Error> {

        let data = match std::fs::read(font_file_path) {
            Ok(data) => { data },
            Err(err) => { return Err(Error::FailedToOpenFontFile(err.to_string())); }
        };

        Font::from_bytes(&data)
    }


    /// # Errors
    ///
    /// Returns error if loading of font failed.
    pub fn from_bytes(data: &[u8]) -> Result<Self, Error> {

        let font = match fontdue::Font::from_bytes(data, fontdue::FontSettings::default()) {
            Ok(font) => { font },
            Err(err) => { return Err(Error::FailedToLoadFont(err.to_string())); }
        };

        Ok(Self {
            font,
            font_map: std::collections::HashMap::new()
        })
    }


    /// # Errors
    ///
    /// Returns an error if character can not be retrieved from font.
    /// The reason is most likely an issue with the generation of the
    /// image for the font texture atlas.
    pub fn get_char(&mut self, c: char, size: f32) -> Result<&CharParams, Error> {

        let size = size as u32;

        let atlas = self.font_map.entry(size).or_insert(
            FontTextureAtlas::new()?
        );

        let params = atlas.get_char_or_insert(c, &self.font, size)?;

        Ok(params)
    }


    /// # Errors
    ///
    /// Returns an error if the texture atlas for the font can not be
    /// retrieved. Most likely reason is an issue with the creation of
    /// the image for the font texture atlas.
    pub fn get_texture(&mut self, size: f32) -> Result<Rc<Texture>, Error> {

        let size = size as u32;

        let atlas = self.font_map.entry(size).or_insert(
            FontTextureAtlas::new()?
        );

        Ok(atlas.get_texture())
    }
}


#[derive(Copy, Clone)]
pub struct CharParams {

    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,

    pub metrics: fontdue::Metrics
}


struct FontTextureAtlas {

    char_map: std::collections::HashMap<char, CharParams>,
    image:    FontTextureImage,
    texture:  Option<Rc<Texture>>
}


impl FontTextureAtlas {

    pub fn new() -> Result<Self, Error> {

        let Some(image) = FontTextureImage::new(128, 128) else {
            return Err(
                 Error::FailedToCreateFontTextureImage("Failed to create font texture image".to_string())
            );
        };

        Ok(Self {
            char_map: std::collections::HashMap::new(),
            image,
            texture: None
        })
    }


    pub fn get_texture(&mut self) -> Rc<Texture> {

        if let Some(texture) = self.texture.as_ref() {
            texture.clone()
        } else {
            let texture = Rc::new(self.image.create_texture());
            self.texture = Some(texture.clone());
            texture
        }

    }


    pub fn get_char_or_insert(
        &mut self,
        c: char,
        font: &fontdue::Font,
        size: u32
    ) -> Result<&CharParams, Error> {

        if self.char_map.contains_key(&c) {

            let Some(params) = self.char_map.get(&c) else { panic!(); };
            Ok(params)

        } else {

            let params = self.add_char_to_image(c, font, size)?;
            let params = self.char_map.entry(c).or_insert(params);
            Ok(params)
        }
    }


    fn add_char_to_image(&mut self, c: char, font: &fontdue::Font, size: u32) -> Result<CharParams, Error> {

        // After inserting a new character the texture is no longer up to date
        self.texture = None;

        // Create the bitmap for the new character
        let (metrics, bitmap) = font.rasterize(c, size as f32);

        // Add character, resize the underling image if necessary
        let params = if let Some(params) = self.image.add_char(metrics, &bitmap) { params } else {

            self.resize_image(metrics.width as u32, metrics.height as u32);
            match self.image.add_char(metrics, &bitmap) {
                Some(params) => params,
                None => {
                    return Err(
                        Error::FailedToCreateFontTextureImage("Resizing did not work".to_string())
                    );
                }
            }
        };

        Ok(params)
    }


    fn resize_image(&mut self, width: u32, height: u32) {

        let width  = width  + self.image.get_width();
        let height = height + self.image.get_height();

        let Some(mut new_image) = FontTextureImage::new(width, height) else { return; };

        for param in self.char_map.values_mut() {

            *param = match new_image.copy_char(&self.image, param) {
                Some(param) => param,
                None => { return; }
            };
        }

        self.image = new_image;
    }
}


struct FontTextureImage {

    width:  u32,
    height: u32,
    image:  image::GrayImage,

    pos_x:       u32,
    pos_y:       u32,
    line_height: u32
}


impl FontTextureImage {


    pub fn new(width: u32, height: u32) -> Option<Self> {

        let image = image::GrayImage::from_raw(
            width,
            height,
            vec![0; (width * height) as usize]
        )?;

        Some(Self {
            width,
            height,
            image,
            pos_x: 0,
            pos_y: 0,
            line_height: 0
        })
    }


    #[must_use]
    pub fn get_width(&self) -> u32 { self.width }


    #[must_use]
    pub fn get_height(&self) -> u32 { self.height }


    #[must_use]
    pub fn create_texture(&self) -> Texture {

        Texture::from_gray_image(&self.image, self.width, self.height, Some("font texture atlas"))
    }


    #[must_use]
    pub fn add_char(
        &mut self,
        metrics: fontdue::Metrics,
        bitmap:  &[u8]
    ) -> Option<CharParams> {

        if self.has_space_in_current_line(metrics.width, metrics.height) {

            let pos_x = self.pos_x;
            let pos_y = self.pos_y;

            self.draw_char(pos_x, pos_y, &metrics, bitmap);

            Some(CharParams {
                x: pos_x,
                y: pos_y,
                w: metrics.width as u32,
                h: metrics.height as u32,
                metrics
            })
        }
        else if self.has_space_in_new_line(metrics.width, metrics.height) {

            self.pos_y += self.line_height;
            self.pos_x = 0;
            self.line_height = 0;

            let pos_x = self.pos_x;
            let pos_y = self.pos_y;

            self.draw_char(pos_x, pos_y, &metrics, bitmap);

            Some(CharParams {
                x: pos_x,
                y: pos_y,
                w: metrics.width as u32,
                h: metrics.height as u32,
                metrics
            })
        }
        else {

            None
        }
    }


    pub fn copy_char(
        &mut self,
        source: &FontTextureImage,
        params: &CharParams
    ) -> Option<CharParams> {

        if !self.has_space_in_current_line(params.w as usize, params.h as usize) {
            if self.has_space_in_new_line(params.w as usize, params.h as usize) {
                self.pos_x = 0;
                self.pos_y += self.line_height;
                self.line_height = 0;
            } else {
                return None;
            }
        }

        for x in 0..params.w {
            for y in 0..params.h {

                let luma = source.image.get_pixel(params.x + x, params.y + y);
                self.image.put_pixel(self.pos_x + x, self.pos_y + y, *luma);
            }
        }

        let new_param = CharParams {
            x:       self.pos_x,
            y:       self.pos_y,
            w:       params.w,
            h:       params.h,
            metrics: params.metrics
        };

        self.pos_x += params.w;
        self.line_height = std::cmp::max(self.line_height, params.h);

        Some(new_param)
    }


    fn has_space_in_current_line(&self, char_width: usize, char_height: usize) -> bool {

        let horizontal = char_width  as u32 <= self.width - self.pos_x;
        let vertical   = char_height as u32 <= self.height - self.pos_y;

        horizontal & vertical
    }


    fn has_space_in_new_line(&self, char_width: usize, char_height: usize) -> bool {

        let horizontal = char_width as u32 <= self.width;
        let vertical   = char_height as u32 <= self.height - (self.pos_y + self.line_height);

        horizontal & vertical
    }


    fn draw_char(
        &mut self,
        pos_x:   u32,
        pos_y:   u32,
        metrics: &fontdue::Metrics,
        bitmap:  &[u8])
    {
        use image::Pixel;

        let width  = metrics.width  as u32;
        let height = metrics.height as u32;

        for x in 0..width {
            for y in 0..height {

                let s_index = (y * width + x) as usize;

                let luma = image::Luma::<u8>::from_slice(&bitmap[s_index..=s_index]);

                self.image.put_pixel(pos_x + x, pos_y + y, *luma);
            }
        }

        self.pos_x = pos_x + width;
        self.line_height = std::cmp::max(self.line_height, height);
    }
}
