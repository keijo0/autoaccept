use anyhow::{Context, Result};
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{self, ConnectionExt as XprotoExt, ImageFormat};
use x11rb::protocol::xtest::ConnectionExt as XTestExt;
use x11rb::rust_connection::RustConnection;

use crate::process_image::RawImage;

const BUTTON_PRESS:   u8 = 4;
const BUTTON_RELEASE: u8 = 5;

pub struct X11Context {
    conn:   RustConnection,
    root:   xproto::Window,
    width:  u16,
    height: u16,
}

impl X11Context {
    pub fn new() -> Result<Self> {
        let (conn, screen_num) =
            RustConnection::connect(None).context("Failed to connect to X11 server")?;

        let screen = &conn.setup().roots[screen_num];
        let root   = screen.root;
        let width  = screen.width_in_pixels;
        let height = screen.height_in_pixels;

        Ok(Self { conn, root, width, height })
    }

    pub fn take_screenshot(&self) -> Result<RawImage> {
        let reply = self
            .conn
            .get_image(
                ImageFormat::Z_PIXMAP,
                self.root,
                0,
                0,
                self.width,
                self.height,
                !0u32,
            )?
            .reply()
            .context("GetImage request failed")?;

        let w = self.width  as u32;
        let h = self.height as u32;
        let mut rgba = Vec::with_capacity((w * h * 4) as usize);

        for chunk in reply.data.chunks_exact(4) {
            let (b, g, r) = (chunk[0], chunk[1], chunk[2]);
            rgba.push(r);
            rgba.push(g);
            rgba.push(b);
            rgba.push(255);
        }

        Ok(RawImage { width: w, height: h, data: rgba })
    }

    pub fn set_mouse_pos(&self, x: i16, y: i16) -> Result<()> {
        self.conn
            .warp_pointer(x11rb::NONE, self.root, 0, 0, 0, 0, x, y)?
            .check()
            .context("WarpPointer failed")?;
        self.conn.flush().context("X11 flush failed")?;
        Ok(())
    }

    pub fn mouse_click(&self, depressed: bool) -> Result<()> {
        let event_type = if depressed { BUTTON_PRESS } else { BUTTON_RELEASE };

        self.conn
            .xtest_fake_input(event_type, 1, 0, self.root, 0, 0, 0)?
            .check()
            .context("XTestFakeInput failed")?;
        self.conn.flush().context("X11 flush failed")?;
        Ok(())
    }
}
