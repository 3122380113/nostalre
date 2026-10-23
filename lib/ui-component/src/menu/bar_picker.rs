//! Putting a skill or an item on the skill bar from the menu: the bar's four rows of nine drawn over the screen,
//! a slot picked with the arrows or a tap. What it sends is what a drop on the bar sends (`hotkey_bar::assign_*`).
use super::SLOT_BASE_ID;
use super::frame::{self, Hint, Key, Keys, Textures};
use crate::BuildCtx;
use crate::game::hotkey_bar::{
    DOCK_GAP, DOCK_ICON, DOCK_SLOT, assign_item, assign_skill, clear_slot, slot_key,
};
use crate::hud::style;
use nostalre_game::data_table::skill_name_table::format_skill_display_name;
use nostalre_game::event::GameEvent;
use nostalre_game::hotkey::{HOTKEY_COLS, HOTKEY_ROWS, HOTKEY_TOTAL, HotkeySlotContent};
use nostalre_game::skill::{SkillEnum, skill_icon_path};
use nostalre_ui::draw;
use nostalre_ui::frame::{UiFrame, WidgetId};
use nostalre_ui::rect::Rect;

const SHADE: [f32; 4] = style::shade(0.88);
/// A slot whose item is not in the bag or whose skill is not learnt: still shown, so it is not put over blind
const ABSENT: [f32; 4] = [1.0, 1.0, 1.0, 0.35];
const KEY_GAP: f32 = 4.0;
const ROW_GAP: f32 = 22.0;

/// What goes on the bar
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) enum ToBar {
    /// An inventory item, as long as that index still holds it
    Item {
        index: u16,
        item_id: u32,
    },
    Skill(SkillEnum),
}

pub(super) struct BarPicker {
    what: ToBar,
    slot: usize,
}

impl BarPicker {
    pub fn new(what: ToBar) -> Self {
        Self { what, slot: 0 }
    }

    /// What is going on the bar, by name; `None` once the item has gone from the bag
    fn name<'a>(&self, ctx: &'a BuildCtx) -> Option<&'a str> {
        match self.what {
            ToBar::Item { index, item_id } => ctx
                .character
                .inventory
                .get_item(index)
                .filter(|i| i.item_id == item_id)
                .map(|i| i.name.as_str()),
            ToBar::Skill(skill) => Some(format_skill_display_name(
                skill,
                ctx.data.skill_name.as_ref(),
            )),
        }
    }

    pub fn footer(&self, ctx: &BuildCtx) -> (String, Vec<Hint>) {
        (
            format!("Put {} on a slot.", self.name(ctx).unwrap_or_default()),
            vec![
                Hint::MOVE,
                Hint::tap("Enter", "put", Key::Enter),
                Hint::tap("Del", "empty slot", Key::Remove),
                Hint::CANCEL,
            ],
        )
    }

    /// Draws the bar over `area`; true once the skill or item is on it, or the item is gone
    pub fn build(
        &mut self,
        ui: &mut UiFrame,
        ctx: &mut BuildCtx,
        area: Rect,
        keys: &Keys,
        textures: &mut Textures,
        events: &mut Vec<GameEvent>,
    ) -> bool {
        if self.name(ctx).is_none() {
            return true;
        }
        style::fill(ui, area, SHADE);

        let (row, col) = (self.slot / HOTKEY_COLS, self.slot % HOTKEY_COLS);
        let row = if keys.up {
            (row + HOTKEY_ROWS - 1) % HOTKEY_ROWS
        } else if keys.down {
            (row + 1) % HOTKEY_ROWS
        } else {
            row
        };
        let col = if keys.left {
            (col + HOTKEY_COLS - 1) % HOTKEY_COLS
        } else if keys.right {
            (col + 1) % HOTKEY_COLS
        } else {
            col
        };
        self.slot = row * HOTKEY_COLS + col;

        let line = style::line(ui);
        let key_h = ui.atlas.line_height;
        let pitch = DOCK_SLOT + DOCK_GAP;
        let grid_w = HOTKEY_COLS as f32 * pitch - DOCK_GAP;
        let row_h = DOCK_SLOT + KEY_GAP + key_h + ROW_GAP;
        let grid_h = HOTKEY_ROWS as f32 * row_h - ROW_GAP;
        let x0 = area.x + ((area.w - grid_w) / 2.0).floor().max(0.0);
        let y0 = area.y + ((area.h - grid_h - line - 12.0) / 2.0).floor().max(0.0);
        style::heading_rule(ui, x0, y0, grid_w, "Skill bar");
        let top = y0 + line + 12.0;

        let mut put = keys.enter.then_some(self.slot);
        for slot in 0..HOTKEY_TOTAL {
            let (r, c) = (slot / HOTKEY_COLS, slot % HOTKEY_COLS);
            let rect = Rect::new(
                x0 + c as f32 * pitch,
                top + r as f32 * row_h,
                DOCK_SLOT,
                DOCK_SLOT,
            );
            let resp = ui.interact(WidgetId(SLOT_BASE_ID + slot as u32), rect);
            if slot == self.slot {
                style::push_white(
                    ui,
                    draw::rounded_rect(
                        rect.x - 2.0,
                        rect.y - 2.0,
                        rect.w + 4.0,
                        rect.h + 4.0,
                        7.0,
                        style::ACCENT,
                    ),
                );
            }
            style::slot(ui, rect, resp.hovered());
            if let Some((path, tint)) = slot_icon(ctx, ctx.character.hotkeys.get_slot(slot)) {
                textures.want(&path);
                let pad = ((DOCK_SLOT - DOCK_ICON) / 2.0).floor();
                let icon = Rect::new(rect.x + pad, rect.y + pad, DOCK_ICON, DOCK_ICON);
                frame::image(ui, icon, path, tint);
            }
            if let Some(code) = slot_key(slot) {
                let label = ui.ctx.key_labels.display(code);
                let w = ui.atlas.measure_text(&label);
                let x = rect.x + ((DOCK_SLOT - w) / 2.0).floor();
                let baseline = rect.y + DOCK_SLOT + KEY_GAP + ui.atlas.ascent;
                style::outlined(ui, x, baseline, &label, style::DIM);
            }
            if resp.clicked() {
                self.slot = slot;
                put = Some(slot);
            }
        }

        if keys.remove {
            events.push(clear_slot(ctx.character, self.slot));
        }
        let Some(slot) = put else {
            return false;
        };
        let character = &mut *ctx.character;
        let event = match self.what {
            ToBar::Item { index, .. } => assign_item(character, slot, index),
            ToBar::Skill(skill) => {
                assign_skill(character, ctx.data.skill_use_level.as_ref(), slot, skill)
            }
        };
        events.extend(event);
        true
    }
}

/// What a slot holds, as its icon: faint when the bar could not use it now (`HotkeyBarWindow` leaves those
/// out; here they show, so a slot is never put over unseen)
fn slot_icon(ctx: &BuildCtx, content: HotkeySlotContent) -> Option<(String, [f32; 4])> {
    let c = &*ctx.character;
    let tint = |there: bool| if there { [1.0; 4] } else { ABSENT };
    match content {
        HotkeySlotContent::Skill { skill, .. } => {
            Some((skill_icon_path(skill), tint(c.skills.knows(skill))))
        }
        HotkeySlotContent::Item { item_id } => {
            let held = c.inventory.find_by_item_id(item_id).is_some();
            Some((ctx.data.item_icon_path(item_id)?, tint(held)))
        }
        HotkeySlotContent::Empty => None,
    }
}
