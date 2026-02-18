//! Named folder color presets with target HSL colors.
//!
//! Each [`FolderColor`] variant maps to a target HSL color. When applied,
//! the renderer computes the necessary deltas from the base icon's surface
//! color to produce the target appearance.
//!
//! # Usage
//!
//! ```
//! use folco_core::color::FolderColor;
//!
//! let color = FolderColor::Red;
//! let settings = color.to_hsl_mutation_settings();
//! // settings can be embedded in a CustomizationProfile
//! ```
//!
//! The full list of available colors, including their target HSL values,
//! can be serialized to JSON via [`FolderColor::all_with_metadata`] so that
//! a frontend can present a color picker.

use serde::{Deserialize, Serialize};

use folco_renderer::HslMutationSettings;

/// A named folder color preset.
///
/// Each variant maps to a target HSL color that recolors a standard
/// system folder icon to the desired hue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FolderColor {
    Red,
    Pink,
    Purple,
    DeepPurple,
    Indigo,
    Blue,
    LightBlue,
    Cyan,
    Teal,
    Green,
    LightGreen,
    Lime,
    Yellow,
    Amber,
    Orange,
    DeepOrange,
    Brown,
    Grey,
    BlueGrey,
    White,
    Black,
}

impl FolderColor {
    /// Returns all available folder color presets.
    pub fn all() -> &'static [FolderColor] {
        &[
            FolderColor::Red,
            FolderColor::Pink,
            FolderColor::Purple,
            FolderColor::DeepPurple,
            FolderColor::Indigo,
            FolderColor::Blue,
            FolderColor::LightBlue,
            FolderColor::Cyan,
            FolderColor::Teal,
            FolderColor::Green,
            FolderColor::LightGreen,
            FolderColor::Lime,
            FolderColor::Yellow,
            FolderColor::Amber,
            FolderColor::Orange,
            FolderColor::DeepOrange,
            FolderColor::Brown,
            FolderColor::Grey,
            FolderColor::BlueGrey,
            FolderColor::White,
            FolderColor::Black,
        ]
    }

    /// Human-readable display name.
    pub fn display_name(&self) -> &'static str {
        match self {
            FolderColor::Red => "Red",
            FolderColor::Pink => "Pink",
            FolderColor::Purple => "Purple",
            FolderColor::DeepPurple => "Deep Purple",
            FolderColor::Indigo => "Indigo",
            FolderColor::Blue => "Blue",
            FolderColor::LightBlue => "Light Blue",
            FolderColor::Cyan => "Cyan",
            FolderColor::Teal => "Teal",
            FolderColor::Green => "Green",
            FolderColor::LightGreen => "Light Green",
            FolderColor::Lime => "Lime",
            FolderColor::Yellow => "Yellow",
            FolderColor::Amber => "Amber",
            FolderColor::Orange => "Orange",
            FolderColor::DeepOrange => "Deep Orange",
            FolderColor::Brown => "Brown",
            FolderColor::Grey => "Grey",
            FolderColor::BlueGrey => "Blue Grey",
            FolderColor::White => "White",
            FolderColor::Black => "Black",
        }
    }

    /// Converts this color preset to HSL mutation settings.
    ///
    /// The returned settings contain the **target HSL** color, ready to
    /// embed in a [`CustomizationProfile`](folco_renderer::CustomizationProfile).
    /// The renderer computes the necessary deltas from the base icon's
    /// surface color.
    pub fn to_hsl_mutation_settings(&self) -> HslMutationSettings {
        let (target_hue, target_saturation, target_lightness) = self.target_hsl();
        HslMutationSettings {
            target_hue,
            target_saturation,
            target_lightness,
            enabled: true,
        }
    }

    /// Returns the target `(hue, saturation, lightness)` tuple.
    ///
    /// - Hue is in degrees (0–360).
    /// - Saturation and lightness are fractions (0.0–1.0).
    pub fn target_hsl(&self) -> (f32, f32, f32) {
        match self {
            //                                    hue       sat      light
            FolderColor::Red =>              (  4.11,   0.8962,   0.5843),
            FolderColor::Pink =>             (339.61,   0.8219,   0.5157),
            FolderColor::Purple =>           (291.24,   0.6372,   0.4216),
            FolderColor::DeepPurple =>       (261.60,   0.5187,   0.4725),
            FolderColor::Indigo =>           (230.85,   0.4836,   0.4784),
            FolderColor::Blue =>             (206.57,   0.8974,   0.5412),
            FolderColor::LightBlue =>        (198.67,   0.9757,   0.4843),
            FolderColor::Cyan =>             (186.79,   1.0000,   0.4157),
            FolderColor::Teal =>             (174.40,   1.0000,   0.2941),
            FolderColor::Green =>            (122.42,   0.3944,   0.4922),
            FolderColor::LightGreen =>       ( 87.77,   0.5021,   0.5275),
            FolderColor::Lime =>             ( 65.52,   0.6996,   0.5431),
            FolderColor::Yellow =>           ( 53.88,   1.0000,   0.6157),
            FolderColor::Amber =>            ( 45.00,   1.0000,   0.5137),
            FolderColor::Orange =>           ( 35.76,   1.0000,   0.5000),
            FolderColor::DeepOrange =>       ( 14.39,   1.0000,   0.5667),
            FolderColor::Brown =>            ( 15.92,   0.2539,   0.3784),
            FolderColor::Grey =>             (  0.00,   0.0000,   0.6196),
            FolderColor::BlueGrey =>         (199.53,   0.1830,   0.4608),
            FolderColor::White =>            (  0.00,   0.0000,   0.9333),
            FolderColor::Black =>            (  0.00,   0.0000,   0.2588),
        }
    }

    /// Returns all color presets with their metadata, suitable for
    /// serializing to JSON and sending to a frontend.
    pub fn all_with_metadata() -> Vec<FolderColorMetadata> {
        Self::all()
            .iter()
            .map(|c| {
                let (target_hue, target_saturation, target_lightness) = c.target_hsl();
                FolderColorMetadata {
                    id: *c,
                    display_name: c.display_name().to_string(),
                    target_hue,
                    target_saturation,
                    target_lightness,
                }
            })
            .collect()
    }

    /// Serializes all color presets (with metadata) to a JSON string.
    pub fn all_metadata_json() -> Result<String, serde_json::Error> {
        serde_json::to_string(&Self::all_with_metadata())
    }

    /// Pretty-printed variant of [`all_metadata_json`](Self::all_metadata_json).
    pub fn all_metadata_json_pretty() -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&Self::all_with_metadata())
    }
}

/// Metadata for a single folder color preset, including its target HSL values.
///
/// Serialized to JSON as:
/// ```json
/// {
///   "id": "red",
///   "displayName": "Red",
///   "targetHue": 4.11,
///   "targetSaturation": 0.8962,
///   "targetLightness": 0.5843
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FolderColorMetadata {
    /// Machine-readable color identifier (kebab-case).
    pub id: FolderColor,
    /// Human-readable display name.
    pub display_name: String,
    /// Target hue in degrees (0–360).
    pub target_hue: f32,
    /// Target saturation (0.0–1.0).
    pub target_saturation: f32,
    /// Target lightness (0.0–1.0).
    pub target_lightness: f32,
}

#[cfg(feature = "clap")]
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    use palette::{FromColor, Hsl, Srgb};

    let hsl = Hsl::new(h, s, l);
    let rgb = Srgb::from_color(hsl);
    let (r, g, b) = rgb.into_components();
    (
        (r * 255.0).round() as u8,
        (g * 255.0).round() as u8,
        (b * 255.0).round() as u8,
    )
}

#[cfg(feature = "clap")]
impl clap::ValueEnum for FolderColor {
    fn value_variants<'a>() -> &'a [Self] {
        Self::all()
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        let name = match self {
            FolderColor::Red => "red",
            FolderColor::Pink => "pink",
            FolderColor::Purple => "purple",
            FolderColor::DeepPurple => "deep-purple",
            FolderColor::Indigo => "indigo",
            FolderColor::Blue => "blue",
            FolderColor::LightBlue => "light-blue",
            FolderColor::Cyan => "cyan",
            FolderColor::Teal => "teal",
            FolderColor::Green => "green",
            FolderColor::LightGreen => "light-green",
            FolderColor::Lime => "lime",
            FolderColor::Yellow => "yellow",
            FolderColor::Amber => "amber",
            FolderColor::Orange => "orange",
            FolderColor::DeepOrange => "deep-orange",
            FolderColor::Brown => "brown",
            FolderColor::Grey => "grey",
            FolderColor::BlueGrey => "blue-grey",
            FolderColor::White => "white",
            FolderColor::Black => "black",
        };

        let (h, s, l) = self.target_hsl();
        let (r, g, b) = hsl_to_rgb(h, s, l);
        let help = format!(
            "\x1b[48;2;{r};{g};{b}m  \x1b[0m {}",
            self.display_name()
        );

        Some(clap::builder::PossibleValue::new(name).help(help))
    }
}

impl std::fmt::Display for FolderColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.display_name())
    }
}

impl std::str::FromStr for FolderColor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().replace(' ', "").replace('-', "").replace('_', "").as_str() {
            "red" => Ok(FolderColor::Red),
            "pink" => Ok(FolderColor::Pink),
            "purple" => Ok(FolderColor::Purple),
            "deeppurple" => Ok(FolderColor::DeepPurple),
            "indigo" => Ok(FolderColor::Indigo),
            "blue" => Ok(FolderColor::Blue),
            "lightblue" => Ok(FolderColor::LightBlue),
            "cyan" => Ok(FolderColor::Cyan),
            "teal" => Ok(FolderColor::Teal),
            "green" => Ok(FolderColor::Green),
            "lightgreen" => Ok(FolderColor::LightGreen),
            "lime" => Ok(FolderColor::Lime),
            "yellow" => Ok(FolderColor::Yellow),
            "amber" => Ok(FolderColor::Amber),
            "orange" => Ok(FolderColor::Orange),
            "deeporange" => Ok(FolderColor::DeepOrange),
            "brown" => Ok(FolderColor::Brown),
            "grey" | "gray" => Ok(FolderColor::Grey),
            "bluegrey" | "bluegray" => Ok(FolderColor::BlueGrey),
            "white" => Ok(FolderColor::White),
            "black" => Ok(FolderColor::Black),
            _ => Err(format!("Unknown folder color: '{}'", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_colors_have_valid_params() {
        for color in FolderColor::all() {
            let (hue, sat, light) = color.target_hsl();
            assert!(
                (0.0..360.0).contains(&hue),
                "{:?} hue {} out of range",
                color,
                hue
            );
            assert!(
                (0.0..=1.0).contains(&sat),
                "{:?} saturation {} out of range",
                color,
                sat
            );
            assert!(
                (0.0..=1.0).contains(&light),
                "{:?} lightness {} out of range",
                color,
                light
            );
        }
    }

    #[test]
    fn metadata_json_roundtrip() {
        let json = FolderColor::all_metadata_json().unwrap();
        let parsed: Vec<FolderColorMetadata> = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.len(), FolderColor::all().len());
    }

    #[test]
    fn parse_color_names() {
        assert_eq!("red".parse::<FolderColor>().unwrap(), FolderColor::Red);
        assert_eq!(
            "deep-purple".parse::<FolderColor>().unwrap(),
            FolderColor::DeepPurple
        );
        assert_eq!(
            "BlueGrey".parse::<FolderColor>().unwrap(),
            FolderColor::BlueGrey
        );
        assert_eq!(
            "light_blue".parse::<FolderColor>().unwrap(),
            FolderColor::LightBlue
        );
        assert!("invalid".parse::<FolderColor>().is_err());
    }

    #[test]
    fn to_hsl_mutation_settings() {
        let settings = FolderColor::Red.to_hsl_mutation_settings();
        assert!(settings.enabled);
        assert!((settings.target_hue - 4.11).abs() < 0.01);
        assert!((settings.target_saturation - 0.8962).abs() < 0.001);
        assert!((settings.target_lightness - 0.5843).abs() < 0.001);
    }
}
