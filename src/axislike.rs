//! Tools for working with directional axis-like user inputs (gamesticks, D-Pads and emulated equivalents)
use crate::input_like::InputKind;
use crate::orientation::{Direction, Rotation};
use bevy::input::{gamepad::GamepadButtonType, keyboard::KeyCode};
use bevy::math::Vec2;
use bevy::reflect::{FromReflect, Reflect};
use bevy::utils::FloatOrd;
use serde::{Deserialize, Serialize};

/// A single directional axis with a configurable trigger zone.
///
/// These can be stored in a [`InputKind`] to create a virtual button.
///
/// # Warning
///
/// `positive_low` must be greater than or equal to `negative_low` for this type to be validly constructed.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SingleAxis {
    /// The axis that is being checked.
    pub axis_type: AxisType,
    /// Any axis value higher than this will trigger the input.
    pub positive_low: f32,
    /// Any axis value lower than this will trigger the input.
    pub negative_low: f32,
    /// The target value for this input, used for input mocking.
    ///
    /// WARNING: this field is ignored for the sake of [`Eq`] and [`Hash`](std::hash::Hash)
    pub value: Option<f32>,
}

impl SingleAxis {
    /// Creates a [`SingleAxis`] with both `positive_low` and `negative_low` set to `threshold`.
    #[must_use]
    pub fn symmetric(axis_type: impl Into<AxisType>, threshold: f32) -> SingleAxis {
        SingleAxis {
            axis_type: axis_type.into(),
            positive_low: threshold,
            negative_low: -threshold,
            value: None,
        }
    }

    /// Creates a [`SingleAxis`] with the specified `axis_type` and `value`.
    ///
    /// All thresholds are set to 0.0.
    /// Primarily useful for [input mocking](crate::input_mocking).
    #[must_use]
    pub fn from_value(axis_type: impl Into<AxisType>, value: f32) -> SingleAxis {
        SingleAxis {
            axis_type: axis_type.into(),
            positive_low: 0.0,
            negative_low: 0.0,
            value: Some(value),
        }
    }

    /// Creates a [`SingleAxis`] corresponding to horizontal [`MouseMotion`](bevy::input::mouse::MouseMotion) movement
    #[must_use]
    pub const fn mouse_motion_x() -> SingleAxis {
        SingleAxis {
            axis_type: AxisType::MouseMotion(MouseMotionAxisType::X),
            positive_low: 0.,
            negative_low: 0.,
            value: None,
        }
    }

    /// Creates a [`SingleAxis`] corresponding to vertical [`MouseMotion`](bevy::input::mouse::MouseMotion) movement
    #[must_use]
    pub const fn mouse_motion_y() -> SingleAxis {
        SingleAxis {
            axis_type: AxisType::MouseMotion(MouseMotionAxisType::Y),
            positive_low: 0.,
            negative_low: 0.,
            value: None,
        }
    }

    /// Creates a [`SingleAxis`] with the `axis_type` and `negative_low` set to `threshold`.
    ///
    /// Positive values will not trigger the input.
    pub fn negative_only(axis_type: impl Into<AxisType>, threshold: f32) -> SingleAxis {
        SingleAxis {
            axis_type: axis_type.into(),
            negative_low: threshold,
            positive_low: f32::MAX,
            value: None,
        }
    }

    /// Creates a [`SingleAxis`] with the `axis_type` and `positive_low` set to `threshold`.
    ///
    /// Negative values will not trigger the input.
    pub fn positive_only(axis_type: impl Into<AxisType>, threshold: f32) -> SingleAxis {
        SingleAxis {
            axis_type: axis_type.into(),
            negative_low: f32::MIN,
            positive_low: threshold,
            value: None,
        }
    }

    /// Returns this [`SingleAxis`] with the deadzone set to the specified value
    #[must_use]
    pub fn with_deadzone(mut self, deadzone: f32) -> SingleAxis {
        self.negative_low = -deadzone;
        self.positive_low = deadzone;
        self
    }
}

impl PartialEq for SingleAxis {
    fn eq(&self, other: &Self) -> bool {
        self.axis_type == other.axis_type
            && FloatOrd(self.positive_low) == FloatOrd(other.positive_low)
            && FloatOrd(self.negative_low) == FloatOrd(other.negative_low)
    }
}
impl Eq for SingleAxis {}
impl std::hash::Hash for SingleAxis {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.axis_type.hash(state);
        FloatOrd(self.positive_low).hash(state);
        FloatOrd(self.negative_low).hash(state);
    }
}

/// A virtual Axis that you can get a value between -1 and 1 from.
///
/// Typically, you don't want to store a [`SingleAxis`] in this type,
/// even though it can be stored as an [`InputKind`].
///
/// Instead, use it directly as [`InputKind::SingleAxis`]!
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VirtualAxis {
    /// The input that represents the negative direction of this virtual axis
    pub negative: InputKind,
    /// The input that represents the positive direction of this virtual axis
    pub positive: InputKind,
}

impl VirtualAxis {
    /// Generates a [`VirtualAxis`] corresponding to the horizontal arrow keyboard keycodes
    pub fn horizontal_arrow_keys() -> VirtualAxis {
        VirtualAxis {
            negative: InputKind::Keyboard(KeyCode::Left),
            positive: InputKind::Keyboard(KeyCode::Right),
        }
    }

    /// Generates a [`VirtualAxis`] corresponding to the horizontal arrow keyboard keycodes
    pub fn vertical_arrow_keys() -> VirtualAxis {
        VirtualAxis {
            negative: InputKind::Keyboard(KeyCode::Down),
            positive: InputKind::Keyboard(KeyCode::Up),
        }
    }

    /// Generates a [`VirtualAxis`] corresponding to the `AD` keyboard keycodes.
    pub fn ad() -> VirtualAxis {
        VirtualAxis {
            negative: InputKind::Keyboard(KeyCode::A),
            positive: InputKind::Keyboard(KeyCode::D),
        }
    }

    /// Generates a [`VirtualAxis`] corresponding to the `WS` keyboard keycodes.
    pub fn ws() -> VirtualAxis {
        VirtualAxis {
            negative: InputKind::Keyboard(KeyCode::S),
            positive: InputKind::Keyboard(KeyCode::W),
        }
    }

    #[allow(clippy::doc_markdown)]
    /// Generates a [`VirtualAxis`] corresponding to the horizontal DPad buttons on a gamepad.
    pub fn horizontal_dpad() -> VirtualAxis {
        VirtualAxis {
            negative: InputKind::GamepadButton(GamepadButtonType::DPadLeft),
            positive: InputKind::GamepadButton(GamepadButtonType::DPadRight),
        }
    }

    #[allow(clippy::doc_markdown)]
    /// Generates a [`VirtualAxis`] corresponding to the vertical DPad buttons on a gamepad.
    pub fn vertical_dpad() -> VirtualAxis {
        VirtualAxis {
            negative: InputKind::GamepadButton(GamepadButtonType::DPadDown),
            positive: InputKind::GamepadButton(GamepadButtonType::DPadUp),
        }
    }
}

/// The type of axis used by a [`UserInput`](crate::input_like::UserInput).
///
/// This is stored in either a [`SingleAxis`] or [`DualAxis`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AxisType {
    /// Input associated with movement of the mouse
    MouseMotion(MouseMotionAxisType),
}
/// The direction of motion of the mouse.
///
/// Stored in the [`AxisType`] enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MouseMotionAxisType {
    /// Horizontal movement.
    X,
    /// Vertical movement.
    Y,
}

impl From<MouseMotionAxisType> for AxisType {
    fn from(axis_type: MouseMotionAxisType) -> Self {
        AxisType::MouseMotion(axis_type)
    }
}

impl TryFrom<AxisType> for MouseMotionAxisType {
    type Error = AxisConversionError;

    fn try_from(axis_type: AxisType) -> Result<Self, AxisConversionError> {
        match axis_type {
            AxisType::MouseMotion(inner) => Ok(inner),
        }
    }
}

/// An [`AxisType`] could not be converted into a more specialized variant
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AxisConversionError;

/// A wrapped [`Vec2`] that represents the combination of two input axes.
///
/// The neutral origin is always at 0, 0.
/// When working with gamepad axes, both `x` and `y` values are bounded by [-1.0, 1.0].
/// For other input axes (such as mousewheel data), this may not be true!
///
/// This struct should store the processed form of your raw inputs in a device-agnostic fashion.
/// Any deadzone correction, rescaling or drift-correction should be done at an earlier level.
#[derive(Debug, Copy, Clone, PartialEq, Default, Deserialize, Serialize, Reflect, FromReflect)]
pub struct DualAxisData {
    xy: Vec2,
}

// Constructors
impl DualAxisData {
    /// Creates a new [`DualAxisData`] from the provided (x,y) coordinates
    pub fn new(x: f32, y: f32) -> DualAxisData {
        DualAxisData {
            xy: Vec2::new(x, y),
        }
    }

    /// Creates a new [`DualAxisData`] directly from a [`Vec2`]
    pub fn from_xy(xy: Vec2) -> DualAxisData {
        DualAxisData { xy }
    }

    /// Merge the state of this [`DualAxisData`] with another.
    ///
    /// This is useful if you have multiple sticks bound to the same game action,
    /// and you want to get their combined position.
    ///
    /// # Warning
    ///
    /// This method can result in values with a greater maximum magnitude than expected!
    /// Use [`DualAxisData::clamp_length`] to limit the resulting direction.
    pub fn merged_with(&self, other: DualAxisData) -> DualAxisData {
        DualAxisData::from_xy(self.xy() + other.xy())
    }
}

// Methods
impl DualAxisData {
    /// The value along the x-axis, typically ranging from -1 to 1
    #[must_use]
    #[inline]
    pub fn x(&self) -> f32 {
        self.xy.x
    }

    /// The value along the y-axis, typically ranging from -1 to 1
    #[must_use]
    #[inline]
    pub fn y(&self) -> f32 {
        self.xy.y
    }

    /// The (x, y) values, each typically ranging from -1 to 1
    #[must_use]
    #[inline]
    pub fn xy(&self) -> Vec2 {
        self.xy
    }

    /// The [`Direction`] that this axis is pointing towards, if any
    ///
    /// If the axis is neutral (x,y) = (0,0), a (0, 0) `None` will be returned
    #[must_use]
    #[inline]
    pub fn direction(&self) -> Option<Direction> {
        // TODO: replace this quick-n-dirty hack once Direction::new no longer panics
        if self.xy.length() > 0.00001 {
            return Some(Direction::new(self.xy));
        }
        None
    }

    /// The [`Rotation`] (measured clockwise from midnight) that this axis is pointing towards, if any
    ///
    /// If the axis is neutral (x,y) = (0,0), this will be `None`
    #[must_use]
    #[inline]
    pub fn rotation(&self) -> Option<Rotation> {
        match Rotation::from_xy(self.xy) {
            Ok(rotation) => Some(rotation),
            Err(_) => None,
        }
    }

    /// How far from the origin is this axis's position?
    ///
    /// Typically bounded by 0 and 1.
    ///
    /// If you only need to compare relative magnitudes, use `magnitude_squared` instead for faster computation.
    #[must_use]
    #[inline]
    pub fn length(&self) -> f32 {
        self.xy.length()
    }

    /// The square of the axis' magnitude
    ///
    /// Typically bounded by 0 and 1.
    ///
    /// This is faster than `magnitude`, as it avoids a square root, but will generally have less natural behavior.
    #[must_use]
    #[inline]
    pub fn length_squared(&self) -> f32 {
        self.xy.length_squared()
    }

    /// Clamps the magnitude of the axis
    pub fn clamp_length(&mut self, max: f32) {
        self.xy = self.xy.clamp_length_max(max);
    }
}

impl From<DualAxisData> for Vec2 {
    fn from(data: DualAxisData) -> Vec2 {
        data.xy
    }
}
