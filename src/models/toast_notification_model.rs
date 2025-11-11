use std::{cell::Cell, collections::HashMap};

use chrono::{Local, NaiveTime};
use i18nrs::{I18n, I18nConfig};
use shared::utils::normalize::NormalizeDelay;

use crate::errors::{
    form_error::{FormError, FormErrorReason},
    setting_error::{SettingError, SettingErrorReason},
};

#[derive(Clone, Debug, Default)]
pub struct ToastNotificationModel {
    pub id: usize,
    pub title: String,
    pub message: String,
    pub created_at: NaiveTime,
}

impl ToastNotificationModel {
    pub fn incorrect_field(error: FormError, language: &str) -> Self {
        let translations = HashMap::from([
            match &*NormalizeDelay::normalize_color(language.to_string()) {
                "en" => ("en", include_str!("../i18n/en/base.json")),
                "fr" => ("fr", include_str!("../i18n/fr/base.json")),
                "it" => ("it", include_str!("../i18n/it/base.json")),
                _ => ("en", include_str!("../i18n/en/base.json")),
            },
        ]);

        let config = I18nConfig {
            translations: translations.clone(),
        };
        let i18n = I18n::new(config, translations).unwrap();
        let id = next_id();
        ToastNotificationModel {
            id: id,
            title: format!(
                "{} {}",
                i18n.t("form_incorrect_field"),
                i18n.t(&error.field)
            ),
            message: match error.error {
                FormErrorReason::Empty => {
                    format!("{} {}", i18n.t("empty_form_field"), i18n.t(&error.field))
                }
                FormErrorReason::Past => i18n.t("past_datetime"),
                FormErrorReason::Format(format) => {
                    format!("{} {} {}", error.field, i18n.t("incorrect_format"), format)
                }
                FormErrorReason::Fallback(value) => {
                    format!("{} {}", i18n.t("form_fallback"), value)
                }
            },
            created_at: Local::now().time(),
        }
    }

    pub fn incorrect_setting(error: SettingError, language: &str) -> Self {
        let translations = HashMap::from([
            match &*NormalizeDelay::normalize_color(language.to_string()) {
                "en" => ("en", include_str!("../i18n/en/base.json")),
                "fr" => ("fr", include_str!("../i18n/fr/base.json")),
                "it" => ("it", include_str!("../i18n/it/base.json")),
                _ => ("en", include_str!("../i18n/en/base.json")),
            },
        ]);

        let config = I18nConfig {
            translations: translations.clone(),
        };
        let i18n = I18n::new(config, translations).unwrap();
        let id = next_id();
        ToastNotificationModel {
            id: id,
            title: format!(
                "{} {}",
                i18n.t("setting_incorrect_field"),
                i18n.t(&error.field)
            ),
            message: match error.error {
                SettingErrorReason::Empty => {
                    format!("{} {}", i18n.t("empty_setting_field"), i18n.t(&error.field))
                }
                SettingErrorReason::Format(format) => {
                    format!("{} {} {}", error.field, i18n.t("incorrect_format"), format)
                }
                SettingErrorReason::NonExistent => i18n.t("invalid_setting"),
            },
            created_at: Local::now().time(),
        }
    }
}

impl PartialEq for ToastNotificationModel {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }

    fn ne(&self, other: &Self) -> bool {
        self.id != other.id
    }
}

thread_local! {
    static ID: Cell<usize> = Cell::new(0);
}

fn next_id() -> usize {
    ID.set(ID.get() + 1);

    if ID.get() == 1 {
        0
    } else {
        ID.get() - 1
    }
}
