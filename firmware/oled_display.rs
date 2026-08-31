use embedded_graphics::prelude::*;
use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::text::Text;
use embedded_graphics::pixelcolor::BinaryColor;

pub fn draw_dt_logo(display: &mut impl DrawTarget<BinaryColor>) {
    let style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);

    let text_dt = Text::new("DH", Point::new(30, 15), style);
    text_dt.draw(display).ok();

    let text_keeb = Text::new("KEEB", Point::new(20, 35), style);
    text_keeb.draw(display).ok();
}
