use gl33::{global_loader::*, *};

pub struct Texture {
    pub id: u32,
}

impl Texture {
    pub fn new() -> Option<Self> {
        let mut texture_id: u32 = 0;
        unsafe {
            glGenTextures(1, &mut texture_id);
        }

        if texture_id != 0 {
            Some(Texture { id: texture_id })
        } else {
            None
        }
    }

    pub fn set_image(img_path: &str) {
        let img = image::open(img_path).expect("Failed to load texture image");

        unsafe {
            glTexImage2D(
                GL_TEXTURE_2D,
                0,
                GL_RGB.0 as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                GL_RGB,
                GL_UNSIGNED_BYTE,
                img.as_bytes().as_ptr().cast(),
            );
            glGenerateMipmap(GL_TEXTURE_2D);
        }
    }

    pub fn bind(&self) {
        unsafe {
            glBindTexture(GL_TEXTURE_2D, self.id);
        }
    }
}

pub fn setup_texture_opts() {
    unsafe {
        // set texture wrapping to mirrored repeat
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_MIRRORED_REPEAT.0 as i32);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_MIRRORED_REPEAT.0 as i32);

        // set texture filtering to nearest for minification and linear for magnification
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST.0 as i32);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR.0 as i32);

        // set texture filtering to use mipmaps
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR_MIPMAP_LINEAR.0 as i32);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR.0 as i32);
    }
}
