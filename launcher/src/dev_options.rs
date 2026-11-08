//! Legacy developer panel. Local profiles are a standard Corizon feature.
use super::*;

/// Clicks on the launcher's version that reveal the section.
const CLICKS: u8 = 7;

impl Launcher {
    /// A click on the sidebar's launcher version; the last one opens Settings on the section.
    pub(super) fn version_clicked(&mut self) {
        if self.dev_options {
            return;
        }
        self.dev_clicks += 1;
        if self.dev_clicks >= CLICKS {
            self.dev_options = true;
            self.go(Page::Settings);
        }
    }

    /// The section, when revealed or when one of its options is on.
    pub(super) fn dev_options_section(&self, _cx: &mut Context<Self>) -> Option<Div> {
        if !self.dev_options {
            return None;
        }
        let t = self.theme;
        Some(
            div()
                .flex()
                .flex_col()
                .child(
                    section_title("Developer options", "terminal", &t)
                        .mt(px(20.))
                        .mb(px(12.)),
                )
                .child(index_row(
                    0,
                    tr("Local profiles"),
                    Some(
                        div()
                            .child(tr(
                                "Local profiles are always available in Corizon. Prepare Minecraft resources from your own install.",
                            ))
                            .into_any_element(),
                    ),
                    None,
                    &t,
                )),
        )
    }
}
