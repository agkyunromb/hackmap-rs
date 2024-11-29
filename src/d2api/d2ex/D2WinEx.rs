use crate::d2api::d2consts::*;
use crate::d2api::d2ex::common::*;

pub mod Text {
    use super::*;

    pub fn draw_text(s: PCWSTR, x: i32, y: i32, font: D2Font, color: D2StringColorCodes) {
        let old_font = D2Win::Text::SetFont(font);
        let (width, _) = D2Win::Text::GetTextDimensions(s);

        D2Win::Text::DrawText(s, x - (width / 2), y, color, TRUE);

        D2Win::Text::SetFont(old_font);
    }
}

pub(super) fn init(_modules: &D2Modules) -> Result<(), HookError> {
    Ok(())
}
