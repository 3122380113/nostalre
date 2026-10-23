pub mod colors;
pub mod dialog_container;
pub mod dropdown;
pub mod fallback;
pub mod format;
pub mod head_board;
pub mod scrollbar;
pub mod tab_control;
pub mod window_chrome;

use nostalre_ui::frame::CheckboxTextures;

pub const CHECKBOX: CheckboxTextures = CheckboxTextures {
    off: nostalre_resources::ui::CHECKBOX_0,
    on: nostalre_resources::ui::CHECKBOX_1,
};
