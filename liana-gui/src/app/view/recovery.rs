use std::collections::{HashMap, HashSet};

use iced::{
    widget::{checkbox, tooltip, Space},
    Alignment, Length,
};

use liana::miniscript::bitcoin::{
    bip32::{DerivationPath, Fingerprint},
    Amount,
};

use liana_ui::{
    component::{amount::*, button, form, text::*},
    theme,
    widget::*,
};

use crate::app::{
    cache::Cache,
    menu::Menu,
    view::{
        dashboard,
        message::{CreateSpendMessage, Message},
    },
    Error,
};

#[allow(clippy::too_many_arguments)]
pub fn recovery<'a>(
    cache: &'a Cache,
    recovery_paths: Vec<Element<'a, Message>>,
    selected_path: Option<usize>,
    feerate: &form::Value<String>,
    address: &'a form::Value<String>,
    warning: Option<&Error>,
) -> Element<'a, Message> {
    let no_recovery_paths = recovery_paths.is_empty();
    dashboard(
        &Menu::Recovery,
        cache,
        warning,
        Column::new()
            .push(Container::new(h3("Recovery")).width(Length::Fill))
            .push(Space::with_height(Length::Fixed(20.0)))
            .push(
                Row::new()
                    .spacing(20)
                    .align_y(Alignment::Center)
                    .push(text("Destination").bold())
                    .push(
                        Container::new(
                            form::Form::new_trimmed("Address", address, move |msg| {
                                Message::CreateSpend(CreateSpendMessage::RecipientEdited(
                                    0, "address", msg,
                                ))
                            })
                            .warning("Invalid Bitcoin address")
                            .size(P1_SIZE)
                            .padding(10),
                        )
                        .max_width(500)
                        .width(Length::Fill),
                    )
                    .push(text("Feerate").bold())
                    .push(
                        Container::new(
                            form::Form::new_trimmed("42 (sats/vbyte)", feerate, move |msg| {
                                Message::CreateSpend(CreateSpendMessage::FeerateEdited(msg))
                            })
                            .warning("Invalid feerate")
                            .size(P1_SIZE)
                            .padding(10),
                        )
                        .width(Length::Fixed(200.0)),
                    ),
            )
            .push(if no_recovery_paths {
                Container::new(text("No recovery path is currently available"))
            } else {
                Container::new(
                    Column::new()
                        .spacing(20)
                        .push(text(format!(
                            "{} recovery paths will be available at the next block, select one:",
                            recovery_paths.len()
                        )))
                        .push(Column::with_children(recovery_paths).spacing(20)),
                )
                .style(theme::card::simple)
                .padding(20)
            })
            .push_maybe(if no_recovery_paths {
                None
            } else {
                Some(
                    Row::new()
                        .push(Space::with_width(Length::Fill))
                        .push(
                            if feerate.valid
                                && !feerate.value.is_empty()
                                && address.valid
                                && !address.value.is_empty()
                                && selected_path.is_some()
                            {
                                button::secondary(None, "Next")
                                    .on_press(Message::Next)
                                    .width(Length::Fixed(200.0))
                            } else {
                                button::secondary(None, "Next").width(Length::Fixed(200.0))
                            },
                        )
                        .spacing(20)
                        .align_y(Alignment::Center),
                )
            })
            .spacing(20),
    )
}

pub fn recovery_path_view<'a>(
    index: usize,
    threshold: usize,
    origins: &'a [(Fingerprint, HashSet<DerivationPath>)],
    total_amount: Amount,
    number_of_coins: usize,
    key_aliases: &'a HashMap<Fingerprint, String>,
    selected: bool,
) -> Element<'a, Message> {
    Row::new()
        .push(
            checkbox("", selected)
                .on_toggle(move |_| Message::CreateSpend(CreateSpendMessage::SelectPath(index))),
        )
        .push(
            Column::new()
                .push(
                    Row::new()
                        .align_y(Alignment::Center)
                        .spacing(10)
                        .push(
                            text(format!(
                                "{} signature{} from",
                                threshold,
                                if threshold > 1 { "s" } else { "" }
                            ))
                            .bold(),
                        )
                        .push(origins.iter().fold(
                            Row::new().align_y(Alignment::Center).spacing(5),
                            |row, (fg, _)| {
                                row.push(if let Some(alias) = key_aliases.get(fg) {
                                    Container::new(
                                        tooltip::Tooltip::new(
                                            Container::new(text(alias))
                                                .padding(5)
                                                .style(theme::pill::simple),
                                            liana_ui::widget::Text::new(fg.to_string()),
                                            tooltip::Position::Bottom,
                                        )
                                        .style(theme::card::simple),
                                    )
                                } else {
                                    Container::new(text(fg.to_string()))
                                        .padding(5)
                                        .style(theme::pill::simple)
                                })
                            },
                        )),
                )
                .push(
                    Row::new()
                        .spacing(5)
                        .push(text("can recover"))
                        .push(text(format!(
                            "{} coin{} totalling",
                            number_of_coins,
                            if number_of_coins > 0 { "s" } else { "" }
                        )))
                        .push(amount(&total_amount)),
                )
                .spacing(5),
        )
        .width(Length::Fill)
        .align_y(Alignment::Center)
        .spacing(20)
        .into()
}
