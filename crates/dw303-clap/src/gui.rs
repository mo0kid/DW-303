//! macOS-only embedded GUI; FFI parent attachment stays isolated here.
use super::*;
use clack_extensions::gui::*;
use ravet_surface::{AttachmentError, ParameterAttachment, ParameterControl};
use ravet_webview::{Callbacks, WebView};
use serde_json::{Value, json};
use std::ptr::NonNull;

const SIZE: GuiSize = GuiSize {
    width: 1227,
    height: 589,
};
const HTML: &str = include_str!("../../../editor/index.html");
pub(super) struct GuiState<'a> {
    view: WebView<'a>,
    attached: bool,
    size: GuiSize,
}
struct Dw303Callbacks<'a> {
    shared: &'a Dw303Shared,
    host: HostMainThreadHandle<'a>,
    params: Option<HostParams>,
    error: Option<&'static str>,
    attachments: [ParameterAttachment<'static>; params::COUNT],
}
fn parse_edit(
    json: &str,
) -> Result<(&'static str, ravet_foundation::ParameterId, f32), &'static str> {
    if json.len() > 4096 {
        return Err("Editor message too large");
    }
    let value: Value = serde_json::from_str(json).map_err(|_| "Invalid editor message")?;
    let object = value.as_object().ok_or("Invalid editor message")?;
    if object.len() != 3 {
        return Err("Unknown editor message");
    }
    let kind = match value["type"].as_str() {
        Some("edit") => "edit",
        Some("begin") => "begin",
        Some("value") => "value",
        Some("end") => "end",
        _ => return Err("Unknown editor message"),
    };
    let id = u32::try_from(value["id"].as_u64().ok_or("Invalid parameter ID")?)
        .map_err(|_| "Invalid parameter ID")?;
    if index_of(ClapId::new(id)).is_none() {
        return Err("Unknown parameter");
    }
    let value = value["value"].as_f64().ok_or("Invalid parameter value")? as f32;
    if !value.is_finite() {
        return Err("Invalid parameter value");
    }
    Ok((kind, ravet_foundation::ParameterId(id), value))
}
/// Bounded placeholder protocol for future sequencer editing.
fn parse_panel(text: &str) -> Result<bool, &'static str> {
    if text.len() > 4096 {
        return Err("Editor message too large");
    }
    let value: Value = serde_json::from_str(text).map_err(|_| "Invalid editor message")?;
    if value["type"] != "panel" {
        return Ok(false);
    }
    let object = value.as_object().ok_or("Invalid panel message")?;
    if object.len() < 2 || object.len() > 4 {
        return Err("Invalid panel message");
    }
    let action = value["action"].as_str().ok_or("Invalid panel action")?;
    if !["pitch", "function", "pattern-group", "mode"].contains(&action) {
        return Err("Unknown panel action");
    }
    for (key, v) in object {
        match key.as_str() {
            "type" | "action" => (),
            "key"
                if v.as_str().is_some_and(|s| {
                    s.len() <= 32 && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
                }) => {}
            "value" if v.as_u64().is_some_and(|n| n <= 16) => (),
            _ => return Err("Invalid panel field"),
        }
    }
    Ok(true)
}
fn panel_size(size: GuiSize) -> GuiSize {
    let rounded_height = |width: u32| (width as f64 * 785.0 / 1636.0).round() as u32;
    let width = if size.height == rounded_height(size.width) {
        size.width
    } else {
        size.width
            .min((size.height as f64 * 1636.0 / 785.0).round() as u32)
    }
    .clamp(818, 2045);
    GuiSize {
        width,
        height: rounded_height(width),
    }
}
struct GuiControl<'a> {
    shared: &'a Dw303Shared,
    host: HostMainThreadHandle<'a>,
    params: Option<HostParams>,
}
impl GuiControl<'_> {
    fn notify(&self, result: Result<(), EditError>) -> Result<(), EditError> {
        result?;
        if let Some(params) = self.params {
            params.request_flush(&self.host);
        } else {
            self.host.request_process();
        }
        Ok(())
    }
}
impl ParameterControl for GuiControl<'_> {
    type Error = EditError;
    fn commit(&self, id: ravet_foundation::ParameterId, physical: f32) -> Result<(), EditError> {
        self.notify(queue_edit(self.shared, id, physical))
    }
    fn begin(&self, id: ravet_foundation::ParameterId) -> Result<(), EditError> {
        self.notify(self.shared.edits.begin(
            ClapId::new(id.0),
            self.shared.generation.load(Ordering::Acquire),
        ))
    }
    fn update(&self, id: ravet_foundation::ParameterId, physical: f32) -> Result<(), EditError> {
        self.notify(self.shared.edits.update(ClapId::new(id.0), physical))
    }
    fn end(&self, _: ravet_foundation::ParameterId) -> Result<(), EditError> {
        self.notify(self.shared.edits.end())
    }
}
impl Callbacks for Dw303Callbacks<'_> {
    fn message(&mut self, json: &str) {
        let result = (|| {
            if parse_panel(json)? {
                return Ok("panel");
            }
            let (kind, id, value) = parse_edit(json)?;
            let index = params::schema()
                .map_err(|_| "Invalid schema")?
                .index(id)
                .ok_or("Unknown parameter")?;
            let control = GuiControl {
                shared: self.shared,
                host: self.host,
                params: self.params,
            };
            let attachment = &mut self.attachments[index];
            let result = match kind {
                "edit" => attachment.commit(&control, value),
                "begin" => attachment.begin(&control),
                "value" => attachment.update(&control, value),
                _ => attachment.end(&control),
            };
            result.map_err(|error| match error {
                AttachmentError::InvalidValue => "Invalid parameter value",
                AttachmentError::GestureState => "Invalid gesture sequence",
                AttachmentError::Transport(EditError::GestureActive) => {
                    "Finish the current gesture first"
                }
                AttachmentError::Transport(EditError::NoGesture) => "No active gesture",
                AttachmentError::Transport(EditError::Full) => "Host edit queue full; try again",
                AttachmentError::Transport(EditError::UnknownParameter) => "Unknown parameter",
                AttachmentError::Transport(EditError::InvalidValue) => "Invalid parameter value",
            })?;
            Ok(kind)
        })();
        self.error = match result {
            Ok("end") => self.error,
            Ok(_) => None,
            Err(error) => Some(error),
        };
    }
    fn snapshot(&mut self) -> Option<String> {
        let mut values = serde_json::Map::new();
        for (i, attachment) in self.attachments.iter_mut().enumerate() {
            attachment.refresh(self.shared.value(i));
            values.insert(
                params::DESCRIPTORS[i].id.0.to_string(),
                json!(attachment.physical()),
            );
        }
        Some(json!({"params":values,"leds":{"run":u8::from(self.shared.value(11)==1.0)},"error":self.error,"pending":self.shared.edits.pending()}).to_string())
    }
}
impl Drop for Dw303Callbacks<'_> {
    fn drop(&mut self) {
        let control = GuiControl {
            shared: self.shared,
            host: self.host,
            params: self.params,
        };
        for attachment in &mut self.attachments {
            if attachment.gesture_active() {
                let _ = attachment.end(&control);
            }
        }
    }
}
impl PluginGuiImpl for Dw303Main<'_> {
    fn is_api_supported(&self, configuration: GuiConfiguration) -> bool {
        configuration.api_type == GuiApiType::COCOA && !configuration.is_floating
    }
    fn get_preferred_api(&self) -> Option<GuiConfiguration<'_>> {
        Some(GuiConfiguration {
            api_type: GuiApiType::COCOA,
            is_floating: false,
        })
    }
    fn create(&self, configuration: GuiConfiguration) -> Result<(), PluginError> {
        if !self.is_api_supported(configuration) || self.gui.borrow().is_some() {
            return Err(PluginError::Message(
                "unsupported or already created editor",
            ));
        }
        let callbacks = Dw303Callbacks {
            shared: self.shared,
            host: self.host,
            params: self.host.get_extension::<HostParams>(),
            error: None,
            attachments: core::array::from_fn(|i| {
                ParameterAttachment::new(
                    params::schema().expect("valid static schema"),
                    params::DESCRIPTORS[i].id,
                )
                .expect("known parameter")
            }),
        };
        let html = ravet_surface::embed_web_controls(HTML)
            .ok_or(PluginError::Message("invalid local control page"))?;
        let view = WebView::from_html(&html, SIZE.width as f64, SIZE.height as f64, callbacks)
            .map_err(|_| PluginError::Message("native editor creation failed or wrong thread"))?;
        view.set_background(23.0 / 255.0, 29.0 / 255.0, 36.0 / 255.0);
        view.set_visible(false);
        *self.gui.borrow_mut() = Some(GuiState {
            view,
            attached: false,
            size: SIZE,
        });
        Ok(())
    }
    fn destroy(&self) {
        let editor = self.gui.borrow_mut().take();
        drop(editor);
    }
    fn set_scale(&self, scale: f64) -> Result<(), PluginError> {
        if scale.is_finite() && scale > 0.0 {
            Ok(())
        } else {
            Err(PluginError::Message("invalid editor scale"))
        }
    }
    fn get_size(&self) -> Option<GuiSize> {
        self.gui.borrow().as_ref().map(|state| state.size)
    }
    fn can_resize(&self) -> bool {
        true
    }
    fn get_resize_hints(&self) -> Option<GuiResizeHints> {
        Some(GuiResizeHints {
            can_resize_horizontally: true,
            can_resize_vertically: true,
            strategy: AspectRatioStrategy::Preserve {
                width: 1636,
                height: 785,
            },
        })
    }
    fn adjust_size(&self, size: GuiSize) -> Option<GuiSize> {
        Some(panel_size(size))
    }
    fn set_size(&self, size: GuiSize) -> Result<(), PluginError> {
        if self.adjust_size(size) != Some(size) {
            return Err(PluginError::Message(
                "editor size outside panel aspect/range",
            ));
        }
        let mut state = self.gui.borrow_mut();
        let state = state
            .as_mut()
            .ok_or(PluginError::Message("editor not created"))?;
        if !state.view.set_size(size.width as f64, size.height as f64) {
            return Err(PluginError::Message("native resize failed"));
        }
        state.size = size;
        Ok(())
    }
    fn set_parent(&self, window: Window) -> Result<(), PluginError> {
        let parent = window
            .as_cocoa_nsview()
            .and_then(NonNull::new)
            .ok_or(PluginError::Message("editor requires Cocoa NSView parent"))?;
        let mut state = self.gui.borrow_mut();
        let state = state
            .as_mut()
            .ok_or(PluginError::Message("editor not created"))?;
        // SAFETY: CLAP host must supply a live NSView on the Cocoa main thread
        // and retain it until detach/destroy, as required by its GUI contract.
        unsafe {
            state.view.attach(parent);
        }
        state.attached = true;
        Ok(())
    }
    fn set_transient(&self, _: Window) -> Result<(), PluginError> {
        Err(PluginError::Message("floating editor unsupported"))
    }
    fn show(&self) -> Result<(), PluginError> {
        let state = self.gui.borrow();
        let state = state
            .as_ref()
            .filter(|s| s.attached)
            .ok_or(PluginError::Message("editor has no parent"))?;
        state.view.set_visible(true);
        Ok(())
    }
    fn hide(&self) -> Result<(), PluginError> {
        let state = self.gui.borrow();
        let state = state
            .as_ref()
            .ok_or(PluginError::Message("editor not created"))?;
        state.view.set_visible(false);
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn resize_is_bounded_and_idempotent() {
        for size in [
            SIZE,
            GuiSize {
                width: 818,
                height: 393,
            },
            GuiSize {
                width: 2045,
                height: 981,
            },
            GuiSize {
                width: 0,
                height: 0,
            },
            GuiSize {
                width: u32::MAX,
                height: u32::MAX,
            },
        ] {
            let adjusted = panel_size(size);
            assert_eq!(panel_size(adjusted), adjusted);
            assert!((818..=2045).contains(&adjusted.width));
            assert!((adjusted.height as f64 - adjusted.width as f64 * 785.0 / 1636.0).abs() <= 0.5);
        }
    }
    #[test]
    fn protocol_bounds_and_panel_validation() {
        for message in [
            " ".repeat(4097),
            format!(r#"{{"type":"edit","id":1,"value":{}}}"#, "9".repeat(4096)),
        ] {
            assert!(parse_edit(&message).is_err());
            assert!(parse_panel(&message).is_err());
        }
        for text in [
            r#"{"type":"edit","id":13,"value":0}"#,
            r#"{"type":"edit","id":1,"value":"NaN"}"#,
            r#"{"type":"edit","id":1,"value":1e309}"#,
        ] {
            assert!(parse_edit(text).is_err());
        }
        for text in [
            r#"{"type":"panel","action":"unknown"}"#,
            r#"{"type":"panel","action":"mode","value":17}"#,
            r#"{"type":"panel","action":"pitch","key":[]}"#,
            r#"{"type":"panel","action":"pitch","value":1e100}"#,
            r#"{"type":"panel","action":"mode","extra":0}"#,
        ] {
            assert!(parse_panel(text).is_err());
        }
        assert_eq!(
            parse_panel(r#"{"type":"panel","action":"pitch","key":"key-c"}"#),
            Ok(true)
        );
        assert_eq!(
            parse_panel(r#"{"type":"panel","action":"mode","value":5}"#),
            Ok(true)
        );
    }
    #[test]
    fn inline_page_fits_and_contains_all_controls() {
        let page = ravet_surface::embed_web_controls(HTML).unwrap();
        assert!(page.len() < 1_048_576);
        assert!(!page.contains("<filter"));
        for id in 1..=8 {
            assert!(page.contains(&format!("id=\"param-{id}\"")));
        }
    }
    #[test]
    fn editor_protocol_rejects_malformed_and_nonfinite_messages() {
        assert_eq!(
            parse_edit(r#"{"type":"edit","id":1,"value":0.5}"#),
            Ok(("edit", ParameterId(1), 0.5))
        );
        for text in [
            "null",
            "[]",
            "{",
            r#"{"type":"edit","id":-1,"value":1}"#,
            r#"{"type":"edit","id":1,"value":1e100}"#,
            r#"{"type":"edit","id":1,"value":1,"extra":1}"#,
            r#"{"type":"other","id":1,"value":1}"#,
        ] {
            assert!(parse_edit(text).is_err());
        }
    }
}
