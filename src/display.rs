use crate::report::parsed::{ArrayValueItem, Field, ParsedReport, VarItem};
use crate::report_descriptor::data::{Size, SizedPayload};
use crate::report_descriptor::{
    Collection, Data, DataFieldOptions, GlobalType, ItemType, Linear, LocalType, MainType,
    Mutability, NullState, ReportDescriptor, ReportDescriptorItem, State, Structure, Value,
    Volatile, Wrap,
};
use crate::usage_table::consumer::ConsumerUsage;
use crate::usage_table::fido::FIDOAllianceUsage;
use crate::usage_table::generic_desktop::GenericDesktopControlsUsage;
use crate::usage_table::keyboard::KeyboardUsage;
use crate::usage_table::Usage;
use crate::usage_table::UsagePage;
use std::fmt;
use std::fmt::Formatter;

impl fmt::Display for GlobalType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GlobalType::UsagePage => f.write_str("Usage Page"),
            GlobalType::LogicalMinimum => f.write_str("Logical Minimum"),
            GlobalType::LogicalMaximum => f.write_str("Logical Maximum"),
            GlobalType::PhysicalMinimum => f.write_str("Physical Minimum"),
            GlobalType::PhysicalMaximum => f.write_str("Physical Maximum"),
            GlobalType::UnitExponent => f.write_str("Unit Exponent"),
            GlobalType::Unit => f.write_str("Unit"),
            GlobalType::ReportSize => f.write_str("Report Size"),
            GlobalType::ReportID => f.write_str("Report ID"),
            GlobalType::ReportCount => f.write_str("Report Count"),
            GlobalType::Push => f.write_str("Push"),
            GlobalType::Pop => f.write_str("Pop"),
        }
    }
}

impl fmt::Display for MainType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MainType::Input => f.write_str("Input"),
            MainType::Output => f.write_str("Output"),
            MainType::Feature => f.write_str("Feature"),
            MainType::Collection => f.write_str("Collection"),
            MainType::EndCollection => f.write_str("End Collection"),
        }
    }
}

impl fmt::Display for Collection {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Collection::Physical => write!(f, "Physical"),
            Collection::Application => write!(f, "Application"),
            Collection::Logical => write!(f, "Logical"),
            Collection::Report => write!(f, "Report"),
            Collection::NamedArray => write!(f, "Named Array"),
            Collection::UsageSwitch => write!(f, "Usage Switch"),
            Collection::UsageModifier => write!(f, "Usage Modifier"),
            Collection::Reserved(i) => write!(f, "Reserved ({})", i),
            Collection::VendorDefined(i) => write!(f, "Vendor defined ({})", i),
        }
    }
}

impl fmt::Display for DataFieldOptions {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {}, {}",
            self.mutability(),
            self.structure(),
            self.value()
        )
    }
}

impl fmt::Display for Mutability {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Mutability::Data => write!(f, "Data"),
            Mutability::Constant => write!(f, "Const"),
        }
    }
}

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Structure::Array => write!(f, "Arr"),
            Structure::Variable => write!(f, "Var"),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Absolute => write!(f, "Abs"),
            Value::Relative => write!(f, "Rel"),
        }
    }
}

impl fmt::Display for Wrap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Wrap::NoWrap => write!(f, "No Wrap"),
            Wrap::Wrap => write!(f, "Wrap"),
        }
    }
}

impl fmt::Display for Linear {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Linear::Linear => write!(f, "Linear"),
            Linear::NonLinear => write!(f, "Non-linear"),
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            State::Preferred => write!(f, "Preferred State"),
            State::NoPreferred => write!(f, "No Preferred State"),
        }
    }
}

impl fmt::Display for NullState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            NullState::NoNullPosition => write!(f, "No Null Position"),
            NullState::NullState => write!(f, "Null State"),
        }
    }
}

impl fmt::Display for Volatile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Volatile::NonVolatile => write!(f, "Non Volatile"),
            Volatile::Volatile => write!(f, "Volatile"),
        }
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Data::BitField => write!(f, "Bit Field"),
            Data::BufferedBytes => write!(f, "Buffered Bytes"),
        }
    }
}

impl fmt::Display for LocalType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            LocalType::Usage => f.write_str("Usage"),
            LocalType::UsageMinimum => f.write_str("Usage Minimum"),
            LocalType::UsageMaximum => f.write_str("Usage Maximum"),
            LocalType::DesignatorIndex => f.write_str("Designator Index"),
            LocalType::DesignatorMinimum => f.write_str("Designator Minimum"),
            LocalType::DesignatorMaximum => f.write_str("Designator Maximum"),
            LocalType::StringIndex => f.write_str("String Index"),
            LocalType::StringMinimum => f.write_str("String Minimum"),
            LocalType::StringMaximum => f.write_str("String Maximum"),
            LocalType::Delimiter => f.write_str("Delimiter"),
        }
    }
}

impl fmt::Display for UsagePage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            UsagePage::Undefined => f.write_str("Undefined"),
            UsagePage::GenericDesktopControls => f.write_str("Generic Desktop Controls"),
            UsagePage::SimulationControls => f.write_str("Simulation Controls"),
            UsagePage::VRControls => f.write_str("VR Controls"),
            UsagePage::SportControls => f.write_str("Sport Controls"),
            UsagePage::GameControls => f.write_str("Game Controls"),
            UsagePage::GenericDeviceControls => f.write_str("Generic DeviceControls"),
            UsagePage::Keyboard => f.write_str("Keyboard"),
            UsagePage::LED => f.write_str("LED"),
            UsagePage::Button => f.write_str("Button"),
            UsagePage::Ordinal => f.write_str("Ordinal"),
            UsagePage::Telephony => f.write_str("Telephony"),
            UsagePage::Consumer => f.write_str("Consumer"),
            UsagePage::Digitizer => f.write_str("Digitizer"),
            UsagePage::Haptics => f.write_str("Haptics"),
            UsagePage::PhysicalInputDevice => f.write_str("Physical Input Device"),
            UsagePage::Unicode => f.write_str("Unicode"),
            UsagePage::EyeAndHeadTracker => f.write_str("Eye and Head Tracker"),
            UsagePage::AuxiliaryDisplay => f.write_str("Auxiliary Display"),
            UsagePage::Sensor => f.write_str("Sensor"),
            UsagePage::MedicalInstruments => f.write_str("Medical Instruments"),
            UsagePage::BrailleDisplay => f.write_str("Braille Display"),
            UsagePage::LightningAndIllumination => f.write_str("Lightning And Illumination"),
            UsagePage::Monitor => f.write_str("Monitor"),
            UsagePage::MonitorEnumerated => f.write_str("Monitor Enumerated"),
            UsagePage::VESAVirtualControls => f.write_str("VESA VirtualControls"),
            UsagePage::Power => f.write_str("Power"),
            UsagePage::BatterySystem => f.write_str("Battery System"),
            UsagePage::BarcodeScanner => f.write_str("Barcode Scanner"),
            UsagePage::Scale => f.write_str("Scale"),
            UsagePage::MagneticStripeReader => f.write_str("Magnetic Stripe Reader"),
            UsagePage::CameraControl => f.write_str("Camera Control"),
            UsagePage::Arcade => f.write_str("Arcade"),
            UsagePage::GamingDevice => f.write_str("Gaming Device"),
            UsagePage::FIDOAlliance => f.write_str("FIDO Alliance"),
            UsagePage::VendorDefined(i) => write!(f, "Vendor Defined (0x{:02x})", i),
            UsagePage::Reserved(i) => write!(f, "Reserved (0x{:02x})", i),
        }
    }
}

impl fmt::Display for ItemType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ItemType::Main(item) => write!(f, "{}", item),
            ItemType::Global(item) => write!(f, "{}", item),
            ItemType::Local(item) => write!(f, "{}", item),
        }
    }
}

/// Implement Display for HidReportDescriptorItem
///
/// Example output:
/// `Usage Page (Generic Desktop Controls)`
impl fmt::Display for ReportDescriptorItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.is_usage_page() {
            return match self.usage_page() {
                Some(u) => write!(f, "{} ({})", self.kind, u),
                None => write!(f, "{} (!!ERROR UNKNOWN!!)", self.kind),
            };
        }

        if self.is_collection() {
            return match self.collection() {
                Some(collection) => write!(f, "{} ({})", self.kind, collection),
                None => write!(f, "{} (!!ERROR UNKNOWN!!)", self.kind),
            };
        }

        if self.is_input_output_or_feature() {
            return match self.data_field_options() {
                Some(options) => write!(f, "{} ({})", self.kind, options),
                None => write!(f, "{} (!!ERROR UNKNOWN!!)", self.kind),
            };
        }

        match self.payload_size {
            Size::Empty => write!(f, "{}", self.kind),
            _ => write!(f, "{} ({})", self.kind, self.raw_payload()),
        }
    }
}

/// Implement Display for Payload
///
/// Will display the output as i32
impl fmt::Display for SizedPayload {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.size() != Size::Empty {
            write!(f, "{}", i32::from(self.clone()))?;
        }

        Ok(())
    }
}

/// Implement the Display trait for ReportDescriptorItemList. Will display an
/// user-friendly list of items.
///
/// Example output:
/// ```text
/// Usage Page (Generic Desktop Controls)
/// Usage (Keyboard)
/// Logical Minimum (0)
/// Logical Maximum (1)
/// Report Count (152)
/// Report Size (1)
/// Input (2)
///```
impl fmt::Display for ReportDescriptor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut usage_page = UsagePage::default();
        let mut indentations: usize = 0;

        for item in self.items() {
            // Display raw bytes
            let len = item.raw.len();
            write!(
                f,
                "{:02x?}{:>width$}",
                item.raw,
                "",
                width = 20 - len * 2 - 2 - 2 * (len - 1)
            )?;

            // Display indentation
            if item.is_end_collection() {
                indentations = indentations.saturating_sub(1);
            }
            write!(f, "{:>width$}", "", width = indentations * 4)?;

            // Display item
            if item.is_usage() || item.is_usage_minimum() || item.is_usage_maximum() {
                let usage = Usage::try_from((&usage_page, item.payload_u16().unwrap_or(0)))
                    .unwrap_or_default();
                writeln!(f, "{} ({})", item.kind, usage)?;
            } else {
                writeln!(f, "{}", item)?;
            }

            // Extract info that we need for an user-friendly Display implementation
            if item.is_usage_page() {
                usage_page = item.usage_page().unwrap_or_default();
            }

            if item.is_collection() {
                indentations = indentations.saturating_add(1);
            }
        }

        Ok(())
    }
}

impl fmt::Display for Usage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Usage::Undefined => f.write_str("Undefined"),
            Usage::GenericDesktopControls(i) => write!(f, "{}", i),
            Usage::SimulationControls(i) => write!(f, "{}", i),
            Usage::VRControls(i) => write!(f, "{}", i),
            Usage::SportControls(i) => write!(f, "{}", i),
            Usage::GameControls(i) => write!(f, "{}", i),
            Usage::GenericDeviceControls(i) => write!(f, "{}", i),
            Usage::Keyboard(i) => write!(f, "{}", i),
            Usage::LED(i) => write!(f, "{}", i),
            Usage::Button(i) => write!(f, "{}", i),
            Usage::Ordinal(i) => write!(f, "{}", i),
            Usage::Telephony(i) => write!(f, "{}", i),
            Usage::Consumer(i) => write!(f, "{}", i),
            Usage::Digitizer(i) => write!(f, "{}", i),
            Usage::Haptics(i) => write!(f, "{}", i),
            Usage::PhysicalInputDevice(i) => write!(f, "{}", i),
            Usage::Unicode(i) => write!(f, "{}", i),
            Usage::EyeAndHeadTracker(i) => write!(f, "{}", i),
            Usage::AuxiliaryDisplay(i) => write!(f, "{}", i),
            Usage::Sensor(i) => write!(f, "{}", i),
            Usage::MedicalInstruments(i) => write!(f, "{}", i),
            Usage::BrailleDisplay(i) => write!(f, "{}", i),
            Usage::LightningAndIllumination(i) => write!(f, "{}", i),
            Usage::Monitor(i) => write!(f, "{}", i),
            Usage::MonitorEnumerated(i) => write!(f, "{}", i),
            Usage::VESAVirtualControls(i) => write!(f, "{}", i),
            Usage::Power(i) => write!(f, "{}", i),
            Usage::BatterySystem(i) => write!(f, "{}", i),
            Usage::BarcodeScanner(i) => write!(f, "{}", i),
            Usage::Scale(i) => write!(f, "{}", i),
            Usage::MagneticStripeReader(i) => write!(f, "{}", i),
            Usage::CameraControl(i) => write!(f, "{}", i),
            Usage::Arcade(i) => write!(f, "{}", i),
            Usage::GamingDevice(i) => write!(f, "{}", i),
            Usage::FIDOAlliance(i) => write!(f, "{}", i),
            Usage::VendorDefined(i) => write!(f, "{}", i),
        }
    }
}

impl fmt::Display for GenericDesktopControlsUsage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Undefined => f.write_str("Undefined"),
            Self::Pointer => f.write_str("Pointer"),
            Self::Mouse => f.write_str("Mouse"),
            Self::Joystick => f.write_str("Joystick"),
            Self::Gamepad => f.write_str("Gamepad"),
            Self::Keyboard => f.write_str("Keyboard"),
            Self::Keypad => f.write_str("Keypad"),
            Self::MultiaxisController => f.write_str("Multi-axis Controller"),
            Self::TabletPCSystemControls => f.write_str("Tablet PC System Controls"),
            Self::WaterCoolingDevice => f.write_str("Water Cooling Device"),
            Self::ComputerChassisDevice => f.write_str("Computer Chassis Device"),
            Self::WirelessRadioControls => f.write_str("Wireless Radio Controls"),
            Self::PortableDeviceControl => f.write_str("Portable Device Control"),
            Self::SystemMultiAxisController => f.write_str("System Multi Axis Controller"),
            Self::SpatialController => f.write_str("Spatial Controller"),
            Self::AssistiveControl => f.write_str("Assistive Control"),
            Self::DeviceDock => f.write_str("Device Dock"),
            Self::DockableDevice => f.write_str("Dockable Device"),
            Self::CallStateManagementControl => f.write_str("Call State Management Control"),
            Self::X => f.write_str("X"),
            Self::Y => f.write_str("Y"),
            Self::Z => f.write_str("Z"),
            Self::Rx => f.write_str("Rx"),
            Self::Ry => f.write_str("Ry"),
            Self::Rz => f.write_str("Rz"),
            Self::Slider => f.write_str("Slider"),
            Self::Dial => f.write_str("Dial"),
            Self::Wheel => f.write_str("Wheel"),
            Self::HatSwitch => f.write_str("Hat Switch"),
            Self::CountedBuffer => f.write_str("Counted Buffer"),
            Self::ByteCount => f.write_str("Byte Count"),
            Self::MotionWakeup => f.write_str("Motion Wakeup"),
            Self::Start => f.write_str("Start"),
            Self::Select => f.write_str("Select"),
            Self::Vx => f.write_str("Vx"),
            Self::Vy => f.write_str("Vy"),
            Self::Vz => f.write_str("Vz"),
            Self::Vbrx => f.write_str("Vbrx"),
            Self::Vbry => f.write_str("Vbry"),
            Self::Vbrz => f.write_str("Vbrz"),
            Self::Vno => f.write_str("Vno"),
            Self::FeatureNotification => f.write_str("Feature Notification"),
            Self::ResolutionMultiplier => f.write_str("Resolution Multiplier"),
            Self::Qx => f.write_str("Qx"),
            Self::Qy => f.write_str("Qy"),
            Self::Qz => f.write_str("Qz"),
            Self::Qw => f.write_str("Qw"),
            Self::SystemControl => f.write_str("System Control"),
            Self::SystemPowerDown => f.write_str("System PowerDown"),
            Self::SystemSleep => f.write_str("System Sleep"),
            Self::SystemWakeUp => f.write_str("System WakeUp"),
            Self::SystemContextMenu => f.write_str("System Context Menu"),
            Self::SystemMainMenu => f.write_str("System Main Menu"),
            Self::SystemAppMenu => f.write_str("System App Menu"),
            Self::SystemMenuHelp => f.write_str("System Menu Help"),
            Self::SystemMenuExit => f.write_str("System Menu Exit"),
            Self::SystemMenuSelect => f.write_str("System Menu Select"),
            Self::SystemMenuRight => f.write_str("System Menu Right"),
            Self::SystemMenuLeft => f.write_str("System Menu Left"),
            Self::SystemMenuUp => f.write_str("System Menu Up"),
            Self::SystemMenuDown => f.write_str("System Menu Down"),
            Self::SystemColdRestart => f.write_str("System Cold Restart"),
            Self::SystemWarmRestart => f.write_str("System Warm Restart"),
            Self::DpadUp => f.write_str("D-pad Up"),
            Self::DpadDown => f.write_str("D-pad Down"),
            Self::DpadRight => f.write_str("D-pad Right"),
            Self::DpadLeft => f.write_str("D-padL eft"),
            Self::IndexTrigger => f.write_str("Index Trigger"),
            Self::PalmTrigger => f.write_str("Palm Trigger"),
            Self::Thumbstick => f.write_str("Thumbstick"),
            Self::SystemFunctionShift => f.write_str("System Function Shift"),
            Self::SystemFunctionShiftLock => f.write_str("System Function Shift Lock"),
            Self::SystemFunctionShiftLockIndicator => {
                f.write_str("System Function Shift Lock Indicator")
            }
            Self::SystemDismissNotification => f.write_str("System Dismiss Notification"),
            Self::SystemDoNotDisturb => f.write_str("System Do Not Disturb"),
            Self::SystemDock => f.write_str("System Dock"),
            Self::SystemUndock => f.write_str("System Undock"),
            Self::SystemSetup => f.write_str("System Setup"),
            Self::SystemBreak => f.write_str("System Break"),
            Self::SystemDebuggerBreak => f.write_str("System Debugger Break"),
            Self::ApplicationBreak => f.write_str("Application Break"),
            Self::ApplicationDebuggerBreak => f.write_str("Application Debugger Break"),
            Self::SystemSpeakerMute => f.write_str("SystemSpeaker Mute"),
            Self::SystemHibernate => f.write_str("System Hibernate"),
            Self::SystemDisplayInvert => f.write_str("System Display Invert"),
            Self::SystemDisplayInternal => f.write_str("System Display Internal"),
            Self::SystemDisplayExternal => f.write_str("System Display External"),
            Self::SystemDisplayBoth => f.write_str("System Display Both"),
            Self::SystemDisplayDual => f.write_str("System Display Dual"),
            Self::SystemDisplayToggleIntExtMode => {
                f.write_str("System Display Toggle Int Ext Mode")
            }
            Self::SystemDisplaySwapPrimarySecondary => {
                f.write_str("System Display Swap Primary/Secondary")
            }
            Self::SystemDisplayToggleLCDAutoscale => {
                f.write_str("System Display Toggle LCD Autoscale")
            }
            Self::SensorZone => f.write_str("Sensor Zone"),
            Self::RPM => f.write_str("RPM"),
            Self::CoolantLevel => f.write_str("Coolant Level"),
            Self::CoolantCriticalLevel => f.write_str("Coolant Critical Level"),
            Self::CoolantPump => f.write_str("Coolant Pump"),
            Self::ChassisEnclosure => f.write_str("Chassis Enclosure"),
            Self::WirelessRadioButton => f.write_str("Wireless Radio Button"),
            Self::WirelessRadioLED => f.write_str("Wireless Radio LED"),
            Self::WirelessRadioSliderSwitch => f.write_str("Wireless Radio Slider Switch"),
            Self::SystemDisplayRotationLockButton => {
                f.write_str("System Display Rotation Lock Button")
            }
            Self::SystemDisplayRotationLockSliderSwitch => {
                f.write_str("System Display Rotation Lock Slider Switch")
            }
            Self::ControlEnable => f.write_str("Control Enable"),
            Self::DockableDeviceUniqueID => f.write_str("Dockable Device Unique ID"),
            Self::DockableDeviceVendorID => f.write_str("Dockable Device Vendor ID"),
            Self::DockableDevicePrimaryUsagePage => {
                f.write_str("Dockable Device Primary UsagePage")
            }
            Self::DockableDevicePrimaryUsageID => f.write_str("Dockable Device Primary Usage ID"),
            Self::DockableDeviceDockingState => f.write_str("Dockable Device Docking State"),
            Self::DockableDeviceDisplayOcclusion => {
                f.write_str("Dockable Device Display Occlusion")
            }
            Self::DockableDeviceObjectType => f.write_str("Dockable Device Object Type"),
            Self::CallActiveLED => f.write_str("Call Active LED"),
            Self::CallMuteToggle => f.write_str("Call Mute Toggle"),
            Self::CallMuteLED => f.write_str("Call Mute LED"),
            Self::Reserved(i) => write!(f, "Reserved ({})", i),
        }
    }
}

impl fmt::Display for ConsumerUsage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ConsumerUsage::Undefined => f.write_str("Undefined"),
            ConsumerUsage::ConsumerControl => f.write_str("Consumer Control"),
            ConsumerUsage::NumericKeyPad => f.write_str("Numeric Key Pad"),
            ConsumerUsage::ProgrammableButtons => f.write_str("Programmable Buttons"),
            ConsumerUsage::Microphone => f.write_str("Microphone"),
            ConsumerUsage::Headphone => f.write_str("Headphone"),
            ConsumerUsage::GraphicEqualizer => f.write_str("Graphic Equalizer"),
            ConsumerUsage::Plus10 => f.write_str("Plus 10"),
            ConsumerUsage::Plus100 => f.write_str("Plus 100"),
            ConsumerUsage::AmPm => f.write_str("AM/PM"),
            ConsumerUsage::Power => f.write_str("Power"),
            ConsumerUsage::Reset => f.write_str("Reset"),
            ConsumerUsage::Sleep => f.write_str("Sleep"),
            ConsumerUsage::SleepAfter => f.write_str("Sleep After"),
            ConsumerUsage::SleepMode => f.write_str("Sleep Mode"),
            ConsumerUsage::Illumination => f.write_str("Illumination"),
            ConsumerUsage::FunctionButtons => f.write_str("Function Buttons"),
            ConsumerUsage::Menu => f.write_str("Menu"),
            ConsumerUsage::MenuPick => f.write_str("Menu Pick"),
            ConsumerUsage::MenuUp => f.write_str("Menu Up"),
            ConsumerUsage::MenuDown => f.write_str("Menu Down"),
            ConsumerUsage::MenuLeft => f.write_str("Menu Left"),
            ConsumerUsage::MenuRight => f.write_str("Menu Right"),
            ConsumerUsage::MenuEscape => f.write_str("Menu Escape"),
            ConsumerUsage::MenuValueIncrease => f.write_str("Menu Value Increase"),
            ConsumerUsage::MenuValueDecrease => f.write_str("Menu Value Decrease"),
            ConsumerUsage::DataOnScreen => f.write_str("Data On Screen"),
            ConsumerUsage::ClosedCaption => f.write_str("Closed Caption"),
            ConsumerUsage::ClosedCaptionSelect => f.write_str("Closed Caption Select"),
            ConsumerUsage::VcrTv => f.write_str("Vcr Tv"),
            ConsumerUsage::BroadcastMode => f.write_str("Broadcast Mode"),
            ConsumerUsage::Snapshot => f.write_str("Snapshot"),
            ConsumerUsage::Still => f.write_str("Still"),
            ConsumerUsage::PictureInPictureToggle => f.write_str("Picture In Picture Toggle"),
            ConsumerUsage::PictureInPictureSwap => f.write_str("Picture In Picture Swap"),
            ConsumerUsage::RedMenuButton => f.write_str("Red Menu Button"),
            ConsumerUsage::GreenMenuButton => f.write_str("Green Menu Button"),
            ConsumerUsage::BlueMenuButton => f.write_str("Blue Menu Button"),
            ConsumerUsage::YellowMenuButton => f.write_str("Yellow Menu Button"),
            ConsumerUsage::Aspect => f.write_str("Aspect"),
            ConsumerUsage::Mode3DSelect => f.write_str("Mode3DSelect"),
            ConsumerUsage::DisplayBrightnessIncrement => {
                f.write_str("Display Brightness Increment")
            }
            ConsumerUsage::DisplayBrightnessDecrement => {
                f.write_str("Display Brightness Decrement")
            }
            ConsumerUsage::DisplayBrightness => f.write_str("Display Brightness"),
            ConsumerUsage::DisplayBacklightToggle => f.write_str("Display Backlight Toggle"),
            ConsumerUsage::DisplaySetBrightnessToMinimum => {
                f.write_str("Display Set Brightness To Minimum")
            }
            ConsumerUsage::DisplaySetBrightnessToMaximum => {
                f.write_str("Display Set Brightness To Maximum")
            }
            ConsumerUsage::DisplaySetAutoBrightness => f.write_str("Display Set Auto Brightness"),
            ConsumerUsage::CameraAccessEnabled => f.write_str("Camera Access Enabled"),
            ConsumerUsage::CameraAccessDisabled => f.write_str("Camera Access Disabled"),
            ConsumerUsage::CameraAccessToggle => f.write_str("Camera Access Toggle"),
            ConsumerUsage::KeyboardBrightnessIncrement => {
                f.write_str("Keyboard Brightness Increment")
            }
            ConsumerUsage::KeyboardBrightnessDecrement => {
                f.write_str("Keyboard Brightness Decrement")
            }
            ConsumerUsage::KeyboardBacklightSetLevel => f.write_str("Keyboard Backlight Set Level"),
            ConsumerUsage::KeyboardBacklightOOC => f.write_str("Keyboard Backlight OOC"),
            ConsumerUsage::KeyboardBacklightSetMinimum => {
                f.write_str("Keyboard Backlight Set Minimum")
            }
            ConsumerUsage::KeyboardBacklightSetMaximum => {
                f.write_str("Keyboard Backlight Set Maximum")
            }
            ConsumerUsage::KeyboardBacklightAuto => f.write_str("Keyboard Backlight Auto"),
            ConsumerUsage::Selection => f.write_str("Selection"),
            ConsumerUsage::AssignSelection => f.write_str("Assign Selection"),
            ConsumerUsage::ModeStep => f.write_str("Mode Step"),
            ConsumerUsage::RecallLast => f.write_str("Recall Last"),
            ConsumerUsage::EnterChannel => f.write_str("Enter Channel"),
            ConsumerUsage::OrderMovie => f.write_str("Order Movie"),
            ConsumerUsage::Channel => f.write_str("Channel"),
            ConsumerUsage::MediaSelection => f.write_str("Media Selection"),
            ConsumerUsage::MediaSelectComputer => f.write_str("Media Select Computer"),
            ConsumerUsage::MediaSelectTV => f.write_str("Media Select TV"),
            ConsumerUsage::MediaSelectWWW => f.write_str("Media Select WWW"),
            ConsumerUsage::MediaSelectDVD => f.write_str("Media Select DVD"),
            ConsumerUsage::MediaSelectTelephone => f.write_str("Media Select Telephone"),
            ConsumerUsage::MediaSelectProgramGuide => f.write_str("Media Select Program Guide"),
            ConsumerUsage::MediaSelectVideoPhone => f.write_str("Media Select Video Phone"),
            ConsumerUsage::MediaSelectGames => f.write_str("Media Select Games"),
            ConsumerUsage::MediaSelectMessages => f.write_str("Media Select Messages"),
            ConsumerUsage::MediaSelectCD => f.write_str("Media Select CD"),
            ConsumerUsage::MediaSelectVCR => f.write_str("Media Select VCR"),
            ConsumerUsage::MediaSelectTuner => f.write_str("Media Select Tuner"),
            ConsumerUsage::Quit => f.write_str("Quit"),
            ConsumerUsage::Help => f.write_str("Help"),
            ConsumerUsage::MediaSelectTape => f.write_str("Media Select Tape"),
            ConsumerUsage::MediaSelectCable => f.write_str("Media Select Cable"),
            ConsumerUsage::MediaSelectSatellite => f.write_str("Media Select Satellite"),
            ConsumerUsage::MediaSelectSecurity => f.write_str("Media Select Security"),
            ConsumerUsage::MediaSelectHome => f.write_str("Media Select Home"),
            ConsumerUsage::MediaSelectCall => f.write_str("Media Select Call"),
            ConsumerUsage::ChannelIncrement => f.write_str("Channel Increment"),
            ConsumerUsage::ChannelDecrement => f.write_str("Channel Decrement"),
            ConsumerUsage::MediaSelectSAP => f.write_str("Media Select SAP"),
            ConsumerUsage::VCRPlus => f.write_str("VCRPlus"),
            ConsumerUsage::Once => f.write_str("Once"),
            ConsumerUsage::Daily => f.write_str("Daily"),
            ConsumerUsage::Weekly => f.write_str("Weekly"),
            ConsumerUsage::Monthly => f.write_str("Monthly"),
            ConsumerUsage::Play => f.write_str("Play"),
            ConsumerUsage::Pause => f.write_str("Pause"),
            ConsumerUsage::Record => f.write_str("Record"),
            ConsumerUsage::FastForward => f.write_str("Fast Forward"),
            ConsumerUsage::Rewind => f.write_str("Rewind"),
            ConsumerUsage::ScanNextTrack => f.write_str("Scan Next Track"),
            ConsumerUsage::ScanPreviousTrack => f.write_str("Scan Previous Track"),
            ConsumerUsage::Stop => f.write_str("Stop"),
            ConsumerUsage::Eject => f.write_str("Eject"),
            ConsumerUsage::RandomPlay => f.write_str("Random Play"),
            ConsumerUsage::SelectDisc => f.write_str("Select Disc"),
            ConsumerUsage::EnterDisc => f.write_str("Enter Disc"),
            ConsumerUsage::Repeat => f.write_str("Repeat"),
            ConsumerUsage::Tracking => f.write_str("Tracking"),
            ConsumerUsage::TrackNormal => f.write_str("Track Normal"),
            ConsumerUsage::SlowTracking => f.write_str("Slow Tracking"),
            ConsumerUsage::FrameForward => f.write_str("Frame Forward"),
            ConsumerUsage::FrameBack => f.write_str("Frame Back"),
            ConsumerUsage::Mark => f.write_str("Mark"),
            ConsumerUsage::ClearMark => f.write_str("Clear Mark"),
            ConsumerUsage::RepeatFromMark => f.write_str("Repeat From Mark"),
            ConsumerUsage::ReturnToMark => f.write_str("Return To Mark"),
            ConsumerUsage::SearchMarkForward => f.write_str("Search Mark Forward"),
            ConsumerUsage::SearchMarkBackwards => f.write_str("Search Mark Backwards"),
            ConsumerUsage::CounterReset => f.write_str("Counter Reset"),
            ConsumerUsage::ShowCounter => f.write_str("Show Counter"),
            ConsumerUsage::TrackingIncrement => f.write_str("Tracking Increment"),
            ConsumerUsage::TrackingDecrement => f.write_str("Tracking Decrement"),
            ConsumerUsage::StopEject => f.write_str("Stop Eject"),
            ConsumerUsage::PlayPause => f.write_str("Play Pause"),
            ConsumerUsage::PlaySkip => f.write_str("Play Skip"),
            ConsumerUsage::VoiceCommand => f.write_str("Voice Command"),
            ConsumerUsage::InvokeCaptureInterface => f.write_str("Invoke Capture Interface"),
            ConsumerUsage::StartOrStopGameRecording => f.write_str("Start Or Stop Game Recording"),
            ConsumerUsage::HistoricalGameCapture => f.write_str("Historical Game Capture"),
            ConsumerUsage::CaptureGameScreenshot => f.write_str("Capture Game Screenshot"),
            ConsumerUsage::ShowOrHideRecordingIndicator => {
                f.write_str("Show Or Hide Recording Indicator")
            }
            ConsumerUsage::StartOrStopMicrophoneCapture => {
                f.write_str("Start Or Stop Microphone Capture")
            }
            ConsumerUsage::StartOrStopCameraCapture => f.write_str("Start Or Stop Camera Capture"),
            ConsumerUsage::StartOrStopGameBroadcast => f.write_str("Start Or Stop Game Broadcast"),
            ConsumerUsage::StartOrStopVoiceDictationSession => {
                f.write_str("Start Or Stop Voice Dictation Session")
            }
            ConsumerUsage::InvokeDismissEmojiPicker => f.write_str("Invoke Dismiss Emoji Picker"),
            ConsumerUsage::Volume => f.write_str("Volume"),
            ConsumerUsage::Balance => f.write_str("Balance"),
            ConsumerUsage::Mute => f.write_str("Mute"),
            ConsumerUsage::Bass => f.write_str("Bass"),
            ConsumerUsage::Treble => f.write_str("Treble"),
            ConsumerUsage::BassBoost => f.write_str("Bass Boost"),
            ConsumerUsage::SurroundMode => f.write_str("Surround Mode"),
            ConsumerUsage::Loudness => f.write_str("Loudness"),
            ConsumerUsage::MPX => f.write_str("MPX"),
            ConsumerUsage::VolumeIncrement => f.write_str("Volume Increment"),
            ConsumerUsage::VolumeDecrement => f.write_str("Volume Decrement"),
            ConsumerUsage::SpeedSelect => f.write_str("Speed Select"),
            ConsumerUsage::PlaybackSpeed => f.write_str("Playback Speed"),
            ConsumerUsage::StandardPlay => f.write_str("Standard Play"),
            ConsumerUsage::LongPlay => f.write_str("Long Play"),
            ConsumerUsage::ExtendedPlay => f.write_str("Extended Play"),
            ConsumerUsage::Slow => f.write_str("Slow"),
            ConsumerUsage::FanEnable => f.write_str("Fan Enable"),
            ConsumerUsage::FanSpeed => f.write_str("Fan Speed"),
            ConsumerUsage::LightEnable => f.write_str("Light Enable"),
            ConsumerUsage::LightIlluminationLevel => f.write_str("Light Illumination Level"),
            ConsumerUsage::ClimateControlEnable => f.write_str("Climate Control Enable"),
            ConsumerUsage::RoomTemperature => f.write_str("Room Temperature"),
            ConsumerUsage::SecurityEnable => f.write_str("Security Enable"),
            ConsumerUsage::FireAlarm => f.write_str("Fire Alarm"),
            ConsumerUsage::PoliceAlarm => f.write_str("Police Alarm"),
            ConsumerUsage::Proximity => f.write_str("Proximity"),
            ConsumerUsage::Motion => f.write_str("Motion"),
            ConsumerUsage::DuressAlarm => f.write_str("Duress Alarm"),
            ConsumerUsage::HoldupAlarm => f.write_str("Holdup Alarm"),
            ConsumerUsage::MedicalAlarm => f.write_str("Medical Alarm"),
            ConsumerUsage::BalanceRight => f.write_str("Balance Right"),
            ConsumerUsage::BalanceLeft => f.write_str("Balance Left"),
            ConsumerUsage::BassIncrement => f.write_str("Bass Increment"),
            ConsumerUsage::BassDecrement => f.write_str("Bass Decrement"),
            ConsumerUsage::TrebleIncrement => f.write_str("Treble Increment"),
            ConsumerUsage::TrebleDecrement => f.write_str("Treble Decrement"),
            ConsumerUsage::SpeakerSystem => f.write_str("Speaker System"),
            ConsumerUsage::ChannelLeft => f.write_str("Channel Left"),
            ConsumerUsage::ChannelRight => f.write_str("Channel Right"),
            ConsumerUsage::ChannelCenter => f.write_str("Channel Center"),
            ConsumerUsage::ChannelFront => f.write_str("Channel Front"),
            ConsumerUsage::ChannelCenterFront => f.write_str("Channel Center Front"),
            ConsumerUsage::ChannelSide => f.write_str("Channel Side"),
            ConsumerUsage::ChannelSurround => f.write_str("Channel Surround"),
            ConsumerUsage::ChannelLowFrequencyEnhancement => {
                f.write_str("Channel Low Frequency Enhancement")
            }
            ConsumerUsage::ChannelTop => f.write_str("Channel Top"),
            ConsumerUsage::ChannelUnknown => f.write_str("Channel Unknown"),
            ConsumerUsage::SubChannel => f.write_str("Sub Channel"),
            ConsumerUsage::SubChannelIncrement => f.write_str("Sub Channel Increment"),
            ConsumerUsage::SubChannelDecrement => f.write_str("Sub Channel Decrement"),
            ConsumerUsage::AlternateAudioIncrement => f.write_str("Alternate Audio Increment"),
            ConsumerUsage::AlternateAudioDecrement => f.write_str("Alternate Audio Decrement"),
            ConsumerUsage::ApplicationLaunchButtons => f.write_str("Application Launch Buttons"),
            ConsumerUsage::ALLaunchButtonConfigurationTool => {
                f.write_str("AL Launch Button Configuration Tool")
            }
            ConsumerUsage::ALProgrammableButtonConfiguration => {
                f.write_str("AL Programmable Button Configuration")
            }
            ConsumerUsage::ALConsumerControlConfiguration => {
                f.write_str("AL Consumer Control Configuration")
            }
            ConsumerUsage::ALWordProcessor => f.write_str("AL Word Processor"),
            ConsumerUsage::ALTextEditor => f.write_str("AL Text Editor"),
            ConsumerUsage::ALSpreadsheet => f.write_str("AL Spreadsheet"),
            ConsumerUsage::ALGraphicsEditor => f.write_str("AL Graphics Editor"),
            ConsumerUsage::ALPresentationApp => f.write_str("AL Presentation App"),
            ConsumerUsage::ALDatabaseApp => f.write_str("AL Database App"),
            ConsumerUsage::ALEmailReader => f.write_str("AL Email Reader"),
            ConsumerUsage::ALNewsreader => f.write_str("AL Newsreader"),
            ConsumerUsage::ALVoicemail => f.write_str("AL Voicemail"),
            ConsumerUsage::ALContactsAddressBook => f.write_str("AL Contacts Address Book"),
            ConsumerUsage::ALCalendarSchedule => f.write_str("AL Calendar Schedule"),
            ConsumerUsage::ALTaskProjectManager => f.write_str("AL Task Project Manager"),
            ConsumerUsage::ALLogJournalTimecard => f.write_str("AL Log Journal Timecard"),
            ConsumerUsage::ALCheckbookFinance => f.write_str("AL Checkbook Finance"),
            ConsumerUsage::ALCalculator => f.write_str("AL Calculator"),
            ConsumerUsage::ALAVCapturePlayback => f.write_str("AL AV Capture Playback"),
            ConsumerUsage::ALLocalMachineBrowser => f.write_str("AL Local Machine Browser"),
            ConsumerUsage::ALLANWANBrowser => f.write_str("AL LAN WAN Browser"),
            ConsumerUsage::ALInternetBrowser => f.write_str("AL Internet Browser"),
            ConsumerUsage::ALRemoteNetworkingISPConnect => {
                f.write_str("AL Remote Networking ISP Connect")
            }
            ConsumerUsage::ALNetworkConference => f.write_str("AL Network Conference"),
            ConsumerUsage::ALNetworkChat => f.write_str("AL Network Chat"),
            ConsumerUsage::ALTelephonyDialer => f.write_str("AL Telephony Dialer"),
            ConsumerUsage::ALLogon => f.write_str("AL Logon"),
            ConsumerUsage::ALLogoff => f.write_str("AL Logoff"),
            ConsumerUsage::ALLogonLogoff => f.write_str("AL Logon Logoff"),
            ConsumerUsage::ALTerminalLockScreensaver => f.write_str("AL Terminal Lock Screensaver"),
            ConsumerUsage::ALControlPanel => f.write_str("AL Control Panel"),
            ConsumerUsage::ALCommandLineProcessorRun => {
                f.write_str("AL Command Line Processor Run")
            }
            ConsumerUsage::ALProcessTaskManager => f.write_str("AL Process Task Manager"),
            ConsumerUsage::ALSelectTaskApplication => f.write_str("AL Select Task Application"),
            ConsumerUsage::ALNextTaskApplication => f.write_str("AL Next Task Application"),
            ConsumerUsage::ALPreviousTaskApplication => f.write_str("AL Previous Task Application"),
            ConsumerUsage::ALPreemptiveHaltTaskApplication => {
                f.write_str("AL Preemptive Halt Task Application")
            }
            ConsumerUsage::ALIntegratedHelpCenter => f.write_str("AL Integrated Help Center"),
            ConsumerUsage::ALDocuments => f.write_str("AL Documents"),
            ConsumerUsage::ALThesaurus => f.write_str("AL Thesaurus"),
            ConsumerUsage::ALDictionary => f.write_str("AL Dictionary"),
            ConsumerUsage::ALDesktop => f.write_str("AL Desktop"),
            ConsumerUsage::ALSpellCheck => f.write_str("AL Spell Check"),
            ConsumerUsage::ALGrammarCheck => f.write_str("ALGrammar Check"),
            ConsumerUsage::ALWirelessStatus => f.write_str("AL Wireless Status"),
            ConsumerUsage::ALKeyboardLayout => f.write_str("AL Keyboard Layout"),
            ConsumerUsage::ALVirusProtection => f.write_str("AL Virus Protection"),
            ConsumerUsage::ALEncryption => f.write_str("AL Encryption"),
            ConsumerUsage::ALScreenSaver => f.write_str("AL Screen Saver"),
            ConsumerUsage::ALAlarms => f.write_str("AL Alarms"),
            ConsumerUsage::ALClock => f.write_str("AL Clock"),
            ConsumerUsage::ALFileBrowser => f.write_str("AL File Browser"),
            ConsumerUsage::ALPowerStatus => f.write_str("AL Power Status"),
            ConsumerUsage::ALImageBrowser => f.write_str("AL Image Browser"),
            ConsumerUsage::ALAudioBrowser => f.write_str("AL Audio Browser"),
            ConsumerUsage::ALMovieBrowser => f.write_str("AL Movie Browser"),
            ConsumerUsage::ALDigitalRightsManager => f.write_str("AL Digital Rights Manager"),
            ConsumerUsage::ALDigitalWallet => f.write_str("AL Digital Wallet"),
            ConsumerUsage::ALInstantMessaging => f.write_str("AL Instant Messaging"),
            ConsumerUsage::ALOEMFeaturesTipsTutorialBrowser => {
                f.write_str("AL OEM Features Tips Tutorial Browser")
            }
            ConsumerUsage::ALOEMHelp => f.write_str("AL OEM Help"),
            ConsumerUsage::ALOnlineCommunity => f.write_str("AL Online Community"),
            ConsumerUsage::ALEntertainmentContentBrowser => {
                f.write_str("AL Entertainment Content Browser")
            }
            ConsumerUsage::ALOnlineShoppingBrowser => f.write_str("AL Online Shopping Browser"),
            ConsumerUsage::ALSmartCardInformationHelp => {
                f.write_str("AL Smart Card Information Help")
            }
            ConsumerUsage::ALMarketMonitorFinanceBrowser => {
                f.write_str("AL Market Monitor Finance Browser")
            }
            ConsumerUsage::ALCustomizedCorporateNewsBrowser => {
                f.write_str("AL Customized Corporate News Browser")
            }
            ConsumerUsage::ALOnlineActivityBrowser => f.write_str("AL Online Activity Browser"),
            ConsumerUsage::ALResearchSearchBrowser => f.write_str("AL Research Search Browser"),
            ConsumerUsage::ALAudioPlayer => f.write_str("AL Audio Player"),
            ConsumerUsage::ALMessageStatus => f.write_str("AL Message Status"),
            ConsumerUsage::ALContactSync => f.write_str("AL Contact Sync"),
            ConsumerUsage::ALNavigation => f.write_str("AL Navigation"),
            ConsumerUsage::ALContextawareDesktopAssistant => {
                f.write_str("AL Contextaware Desktop Assistant")
            }
            ConsumerUsage::GenericGUIApplicationControls => {
                f.write_str("Generic GUI Application Controls")
            }
            ConsumerUsage::ACNew => f.write_str("AC New"),
            ConsumerUsage::ACOpen => f.write_str("AC Open"),
            ConsumerUsage::ACClose => f.write_str("AC Close"),
            ConsumerUsage::ACExit => f.write_str("AC Exit"),
            ConsumerUsage::ACMaximize => f.write_str("AC Maximize"),
            ConsumerUsage::ACMinimize => f.write_str("AC Minimize"),
            ConsumerUsage::ACSave => f.write_str("AC Save"),
            ConsumerUsage::ACPrint => f.write_str("AC Print"),
            ConsumerUsage::ACProperties => f.write_str("AC Properties"),
            ConsumerUsage::ACUndo => f.write_str("AC Undo"),
            ConsumerUsage::ACCopy => f.write_str("AC Copy"),
            ConsumerUsage::ACCut => f.write_str("AC Cut"),
            ConsumerUsage::ACPaste => f.write_str("AC Paste"),
            ConsumerUsage::ACSelectAll => f.write_str("AC Select All"),
            ConsumerUsage::ACFind => f.write_str("AC Find"),
            ConsumerUsage::ACFindAndReplace => f.write_str("AC Find And Replace"),
            ConsumerUsage::ACSearch => f.write_str("AC Search"),
            ConsumerUsage::ACGoTo => f.write_str("AC Go To"),
            ConsumerUsage::ACHome => f.write_str("AC Home"),
            ConsumerUsage::ACBack => f.write_str("AC Back"),
            ConsumerUsage::ACForward => f.write_str("AC Forward"),
            ConsumerUsage::ACStop => f.write_str("AC Stop"),
            ConsumerUsage::ACRefresh => f.write_str("AC Refresh"),
            ConsumerUsage::ACPreviousLink => f.write_str("AC Previous Link"),
            ConsumerUsage::ACNextLink => f.write_str("AC Next Link"),
            ConsumerUsage::ACBookmarks => f.write_str("AC Bookmarks"),
            ConsumerUsage::ACHistory => f.write_str("AC History"),
            ConsumerUsage::ACSubscriptions => f.write_str("AC Subscriptions"),
            ConsumerUsage::ACZoomIn => f.write_str("AC Zoom In"),
            ConsumerUsage::ACZoomOut => f.write_str("AC Zoom Out"),
            ConsumerUsage::ACZoom => f.write_str("AC Zoom"),
            ConsumerUsage::ACFullScreenView => f.write_str("AC Full Screen View"),
            ConsumerUsage::ACNormalView => f.write_str("AC Normal View"),
            ConsumerUsage::ACViewToggle => f.write_str("AC View Toggle"),
            ConsumerUsage::ACScrollUp => f.write_str("AC Scroll Up"),
            ConsumerUsage::ACScrollDown => f.write_str("AC Scroll Down"),
            ConsumerUsage::ACScroll => f.write_str("AC Scroll"),
            ConsumerUsage::ACPanLeft => f.write_str("AC Pan Left"),
            ConsumerUsage::ACPanRight => f.write_str("AC Pan Right"),
            ConsumerUsage::ACPan => f.write_str("AC Pan"),
            ConsumerUsage::ACNewWindow => f.write_str("AC New Window"),
            ConsumerUsage::ACTileHorizontally => f.write_str("AC Tile Horizontally"),
            ConsumerUsage::ACTileVertically => f.write_str("AC Tile Vertically"),
            ConsumerUsage::ACFormat => f.write_str("AC Format"),
            ConsumerUsage::ACEdit => f.write_str("AC Edit"),
            ConsumerUsage::ACBold => f.write_str("AC Bold"),
            ConsumerUsage::ACItalics => f.write_str("AC Italics"),
            ConsumerUsage::ACUnderline => f.write_str("AC Underline"),
            ConsumerUsage::ACStrikethrough => f.write_str("AC Strikethrough"),
            ConsumerUsage::ACSubscript => f.write_str("AC Subscript"),
            ConsumerUsage::ACSuperscript => f.write_str("AC Superscript"),
            ConsumerUsage::ACAllCaps => f.write_str("AC All Caps"),
            ConsumerUsage::ACRotate => f.write_str("AC Rotate"),
            ConsumerUsage::ACResize => f.write_str("AC Resize"),
            ConsumerUsage::ACFlipHorizontal => f.write_str("AC Flip Horizontal"),
            ConsumerUsage::ACFlipVertical => f.write_str("AC Flip Vertical"),
            ConsumerUsage::ACMirrorHorizontal => f.write_str("AC Mirror Horizontal"),
            ConsumerUsage::ACMirrorVertical => f.write_str("AC Mirror Vertical"),
            ConsumerUsage::ACFontSelect => f.write_str("AC Font Select"),
            ConsumerUsage::ACFontColor => f.write_str("AC Font Color"),
            ConsumerUsage::ACFontSize => f.write_str("AC Font Size"),
            ConsumerUsage::ACJustifyLeft => f.write_str("AC Justify Left"),
            ConsumerUsage::ACJustifyCenterH => f.write_str("AC Justify CenterH"),
            ConsumerUsage::ACJustifyRight => f.write_str("AC Justify Right"),
            ConsumerUsage::ACJustifyBlockH => f.write_str("AC Justify BlockH"),
            ConsumerUsage::ACJustifyTop => f.write_str("AC Justify Top"),
            ConsumerUsage::ACJustifyCenterV => f.write_str("AC Justify CenterV"),
            ConsumerUsage::ACJustifyBottom => f.write_str("AC Justify Bottom"),
            ConsumerUsage::ACJustifyBlockV => f.write_str("AC Justify BlockV"),
            ConsumerUsage::ACIndentDecrease => f.write_str("AC Indent Decrease"),
            ConsumerUsage::ACIndentIncrease => f.write_str("AC Indent Increase"),
            ConsumerUsage::ACNumberedList => f.write_str("AC Numbered List"),
            ConsumerUsage::ACRestartNumbering => f.write_str("AC Restart Numbering"),
            ConsumerUsage::ACBulletedList => f.write_str("AC Bulleted List"),
            ConsumerUsage::ACPromote => f.write_str("AC Promote"),
            ConsumerUsage::ACDemote => f.write_str("AC Demote"),
            ConsumerUsage::ACYes => f.write_str("AC Yes"),
            ConsumerUsage::ACNo => f.write_str("AC No"),
            ConsumerUsage::ACCancel => f.write_str("AC Cancel"),
            ConsumerUsage::ACCatalog => f.write_str("AC Catalog"),
            ConsumerUsage::ACBuyCheckout => f.write_str("AC Buy Checkout"),
            ConsumerUsage::ACAddToCart => f.write_str("AC Add To Cart"),
            ConsumerUsage::ACExpand => f.write_str("AC Expand"),
            ConsumerUsage::ACExpandAll => f.write_str("AC Expand All"),
            ConsumerUsage::ACCollapse => f.write_str("AC Collapse"),
            ConsumerUsage::ACCollapseAll => f.write_str("AC Collapse All"),
            ConsumerUsage::ACPrintPreview => f.write_str("AC Print Preview"),
            ConsumerUsage::ACPasteSpecial => f.write_str("AC Paste Special"),
            ConsumerUsage::ACInsertMode => f.write_str("AC Insert Mode"),
            ConsumerUsage::ACDelete => f.write_str("AC Delete"),
            ConsumerUsage::ACLock => f.write_str("AC Lock"),
            ConsumerUsage::ACUnlock => f.write_str("AC Unlock"),
            ConsumerUsage::ACProtect => f.write_str("AC Protect"),
            ConsumerUsage::ACUnprotect => f.write_str("AC Unprotect"),
            ConsumerUsage::ACAttachComment => f.write_str("AC Attach Comment"),
            ConsumerUsage::ACDeleteComment => f.write_str("AC Delete Comment"),
            ConsumerUsage::ACViewComment => f.write_str("AC View Comment"),
            ConsumerUsage::ACSelectWord => f.write_str("AC Select Word"),
            ConsumerUsage::ACSelectSentence => f.write_str("AC Select Sentence"),
            ConsumerUsage::ACSelectParagraph => f.write_str("AC Select Paragraph"),
            ConsumerUsage::ACSelectColumn => f.write_str("AC Select Column"),
            ConsumerUsage::ACSelectRow => f.write_str("AC Select Row"),
            ConsumerUsage::ACSelectTable => f.write_str("AC Select Table"),
            ConsumerUsage::ACSelectObject => f.write_str("AC Select Object"),
            ConsumerUsage::ACRedoRepeat => f.write_str("AC Redo Repeat"),
            ConsumerUsage::ACSort => f.write_str("AC Sort"),
            ConsumerUsage::ACSortAscending => f.write_str("AC Sort Ascending"),
            ConsumerUsage::ACSortDescending => f.write_str("AC Sort Descending"),
            ConsumerUsage::ACFilter => f.write_str("AC Filter"),
            ConsumerUsage::ACSetClock => f.write_str("AC Set Clock"),
            ConsumerUsage::ACViewClock => f.write_str("AC View Clock"),
            ConsumerUsage::ACSelectTimeZone => f.write_str("AC Select Time Zone"),
            ConsumerUsage::ACEditTimeZones => f.write_str("AC Edit Time Zones"),
            ConsumerUsage::ACSetAlarm => f.write_str("AC Set Alarm"),
            ConsumerUsage::ACClearAlarm => f.write_str("AC Clear Alarm"),
            ConsumerUsage::ACSnoozeAlarm => f.write_str("AC Snooze Alarm"),
            ConsumerUsage::ACResetAlarm => f.write_str("AC Reset Alarm"),
            ConsumerUsage::ACSynchronize => f.write_str("AC Synchronize"),
            ConsumerUsage::ACSendReceive => f.write_str("AC Send Receive"),
            ConsumerUsage::ACSendTo => f.write_str("AC Send To"),
            ConsumerUsage::ACReply => f.write_str("AC Reply"),
            ConsumerUsage::ACReplyAll => f.write_str("AC Reply All"),
            ConsumerUsage::ACForwardMsg => f.write_str("AC Forward Msg"),
            ConsumerUsage::ACSend => f.write_str("AC Send"),
            ConsumerUsage::ACAttachFile => f.write_str("AC Attach File"),
            ConsumerUsage::ACUpload => f.write_str("AC Upload"),
            ConsumerUsage::ACDownloadSaveTargetAs => f.write_str("AC Download Save Target As"),
            ConsumerUsage::ACSetBorders => f.write_str("AC Set Borders"),
            ConsumerUsage::ACInsertRow => f.write_str("AC Insert Row"),
            ConsumerUsage::ACInsertColumn => f.write_str("AC Insert Column"),
            ConsumerUsage::ACInsertFile => f.write_str("AC Insert File"),
            ConsumerUsage::ACInsertPicture => f.write_str("AC Insert Picture"),
            ConsumerUsage::ACInsertObject => f.write_str("AC Insert Object"),
            ConsumerUsage::ACInsertSymbol => f.write_str("AC Insert Symbol"),
            ConsumerUsage::ACSaveAndClose => f.write_str("AC Save And Close"),
            ConsumerUsage::ACRename => f.write_str("AC Rename"),
            ConsumerUsage::ACMerge => f.write_str("AC Merge"),
            ConsumerUsage::ACSplit => f.write_str("AC Split"),
            ConsumerUsage::ACDistributeHorizontally => f.write_str("AC Distribute Horizontally"),
            ConsumerUsage::ACDistributeVertically => f.write_str("AC Distribute Vertically"),
            ConsumerUsage::ACNextKeyboardLayoutSelect => {
                f.write_str("AC Next Keyboard Layout Select")
            }
            ConsumerUsage::ACNavigationGuidance => f.write_str("AC Navigation Guidance"),
            ConsumerUsage::ACDesktopShowAllWindows => f.write_str("AC Desktop Show All Windows"),
            ConsumerUsage::ACSoftKeyLeft => f.write_str("AC Soft Key Left"),
            ConsumerUsage::ACSoftKeyRight => f.write_str("AC Soft Key Right"),
            ConsumerUsage::ACDesktopShowAllApplications => {
                f.write_str("AC Desktop Show All Applications")
            }
            ConsumerUsage::ACIdleKeepAlive => f.write_str("AC Idle Keep Alive"),
            ConsumerUsage::ExtendedKeyboardAttributesCollection => {
                f.write_str("Extended Keyboard Attributes Collection")
            }
            ConsumerUsage::KeyboardFormFactor => f.write_str("Keyboard Form Factor"),
            ConsumerUsage::KeyboardKeyType => f.write_str("Keyboard Key Type"),
            ConsumerUsage::KeyboardPhysicalLayout => f.write_str("Keyboard Physical Layout"),
            ConsumerUsage::VendorSpecificKeyboardPhysicalLayout => {
                f.write_str("Vendor Specific Keyboard Physical Layout")
            }
            ConsumerUsage::KeyboardIETFLanguageTagIndex => {
                f.write_str("Keyboard IETFLanguage Tag Index")
            }
            ConsumerUsage::ImplementedKeyboardInputAssistControls => {
                f.write_str("Implemented Keyboard Input Assist Controls")
            }
            ConsumerUsage::KeyboardInputAssistPrevious => {
                f.write_str("Keyboard Input Assist Previous")
            }
            ConsumerUsage::KeyboardInputAssistNext => f.write_str("Keyboard Input Assist Next"),
            ConsumerUsage::KeyboardInputAssistPreviousGroup => {
                f.write_str("Keyboard Input Assist Previous Group")
            }
            ConsumerUsage::KeyboardInputAssistNextGroup => {
                f.write_str("Keyboard Input Assist Next Group")
            }
            ConsumerUsage::KeyboardInputAssistAccept => f.write_str("Keyboard Input Assist Accept"),
            ConsumerUsage::KeyboardInputAssistCancel => f.write_str("Keyboard Input Assist Cancel"),
            ConsumerUsage::PrivacyScreenToggle => f.write_str("Privacy Screen Toggle"),
            ConsumerUsage::PrivacyScreenLevelDecrement => {
                f.write_str("Privacy Screen Level Decrement")
            }
            ConsumerUsage::PrivacyScreenLevelIncrement => {
                f.write_str("Privacy Screen Level Increment")
            }
            ConsumerUsage::PrivacyScreenLevelMinimum => f.write_str("Privacy Screen Level Minimum"),
            ConsumerUsage::PrivacyScreenLevelMaximum => f.write_str("Privacy Screen Level Maximum"),
            ConsumerUsage::ContactEdited => f.write_str("Contact Edited"),
            ConsumerUsage::ContactAdded => f.write_str("Contact Added"),
            ConsumerUsage::ContactRecordActive => f.write_str("Contact Record Active"),
            ConsumerUsage::ContactIndex => f.write_str("Contact Index"),
            ConsumerUsage::ContactNickname => f.write_str("Contact Nickname"),
            ConsumerUsage::ContactFirstName => f.write_str("Contact First Name"),
            ConsumerUsage::ContactLastName => f.write_str("Contact Last Name"),
            ConsumerUsage::ContactFullName => f.write_str("Contact Full Name"),
            ConsumerUsage::ContactPhoneNumberPersonal => {
                f.write_str("Contact Phone Number Personal")
            }
            ConsumerUsage::ContactPhoneNumberBusiness => {
                f.write_str("Contact Phone Number Business")
            }
            ConsumerUsage::ContactPhoneNumberMobile => f.write_str("Contact Phone Number Mobile"),
            ConsumerUsage::ContactPhoneNumberPager => f.write_str("Contact Phone Number Pager"),
            ConsumerUsage::ContactPhoneNumberFax => f.write_str("Contact Phone Number Fax"),
            ConsumerUsage::ContactPhoneNumberOther => f.write_str("Contact Phone Number Other"),
            ConsumerUsage::ContactEmailPersonal => f.write_str("Contact Email Personal"),
            ConsumerUsage::ContactEmailBusiness => f.write_str("Contact Email Business"),
            ConsumerUsage::ContactEmailOther => f.write_str("Contact Email Other"),
            ConsumerUsage::ContactEmailMain => f.write_str("Contact Email Main"),
            ConsumerUsage::ContactSpeedDialNumber => f.write_str("Contact Speed Dial Number"),
            ConsumerUsage::ContactStatusFlag => f.write_str("Contact Status Flag"),
            ConsumerUsage::ContactMisc => f.write_str("Contact Misc"),
            ConsumerUsage::Reserved(i) => write!(f, "Reserved({})", i),
        }
    }
}

impl fmt::Display for KeyboardUsage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            KeyboardUsage::KeyboardErrorRollOver => f.write_str("Keyboard Error Roll Over"),
            KeyboardUsage::KeyboardPOSTFail => f.write_str("Keyboard POST Fail"),
            KeyboardUsage::KeyboardErrorUndefined => f.write_str("Keyboard Error Undefined"),
            KeyboardUsage::KeyboardaandA => f.write_str("Keyboard a and A"),
            KeyboardUsage::KeyboardbandB => f.write_str("Keyboard b and B"),
            KeyboardUsage::KeyboardcandC => f.write_str("Keyboard c and C"),
            KeyboardUsage::KeyboarddandD => f.write_str("Keyboard d and D"),
            KeyboardUsage::KeyboardeandE => f.write_str("Keyboard e and E"),
            KeyboardUsage::KeyboardfandF => f.write_str("Keyboard f and F"),
            KeyboardUsage::KeyboardgandG => f.write_str("Keyboard g and G"),
            KeyboardUsage::KeyboardhandH => f.write_str("Keyboard h and H"),
            KeyboardUsage::KeyboardiandI => f.write_str("Keyboard i and I"),
            KeyboardUsage::KeyboardjandJ => f.write_str("Keyboard j and J"),
            KeyboardUsage::KeyboardkandK => f.write_str("Keyboard k and K"),
            KeyboardUsage::KeyboardlandL => f.write_str("Keyboard l and L"),
            KeyboardUsage::KeyboardmandM => f.write_str("Keyboard m and M"),
            KeyboardUsage::KeyboardnandN => f.write_str("Keyboard n and N"),
            KeyboardUsage::KeyboardoandO => f.write_str("Keyboard o and O"),
            KeyboardUsage::KeyboardpandP => f.write_str("Keyboard p and P"),
            KeyboardUsage::KeyboardqandQ => f.write_str("Keyboard q and Q"),
            KeyboardUsage::KeyboardrandR => f.write_str("Keyboard r and R"),
            KeyboardUsage::KeyboardsandS => f.write_str("Keyboard s and S"),
            KeyboardUsage::KeyboardtandT => f.write_str("Keyboard t and T"),
            KeyboardUsage::KeyboarduandU => f.write_str("Keyboard u and U"),
            KeyboardUsage::KeyboardvandV => f.write_str("Keyboard v and V"),
            KeyboardUsage::KeyboardwandW => f.write_str("Keyboard w and W"),
            KeyboardUsage::KeyboardxandX => f.write_str("Keyboard x and X"),
            KeyboardUsage::KeyboardyandY => f.write_str("Keyboard y and Y"),
            KeyboardUsage::KeyboardzandZ => f.write_str("Keyboard z and Z"),
            KeyboardUsage::Keyboard1andExclamation => f.write_str("Keyboard 1 and !"),
            KeyboardUsage::Keyboard2andAt => f.write_str("Keyboard 2 and @"),
            KeyboardUsage::Keyboard3andHash => f.write_str("Keyboard 3 and #"),
            KeyboardUsage::Keyboard4andDollar => f.write_str("Keyboard 4 and $"),
            KeyboardUsage::Keyboard5andPercent => f.write_str("Keyboard 5 and %"),
            KeyboardUsage::Keyboard6andPower => f.write_str("Keyboard 6 and ^"),
            KeyboardUsage::Keyboard7andAmpersand => f.write_str("Keyboard 7 and &"),
            KeyboardUsage::Keyboard8andStar => f.write_str("Keyboard 8 and *"),
            KeyboardUsage::Keyboard9andLeftParentheses => f.write_str("Keyboard 9 and ("),
            KeyboardUsage::Keyboard0andRightParentheses => f.write_str("Keyboard 0 and ("),
            KeyboardUsage::KeyboardReturnEnter => f.write_str("Keyboard Return (Enter)"),
            KeyboardUsage::KeyboardESCAPE => f.write_str("Keyboard ESCAPE"),
            KeyboardUsage::KeyboardDELETE => f.write_str("Keyboard DELETE"),
            KeyboardUsage::KeyboardTab => f.write_str("Keyboard Tab"),
            KeyboardUsage::KeyboardSpacebar => f.write_str("Keyboard Spacebar"),
            KeyboardUsage::KeyboardDashandUnderscore => f.write_str("Keyboard - and _"),
            KeyboardUsage::KeyboardEqualandPlus => f.write_str("KeyboardEqualandPlus"),
            KeyboardUsage::KeyboardLeftSquareBracketandLeftCurlyBracket => {
                f.write_str("Keyboard [ and {")
            }
            KeyboardUsage::KeyboardRightSquareBracketandRightCurlyBracket => {
                f.write_str("Keyboard ] and }")
            }
            KeyboardUsage::KeyboardBackslashandPipe => f.write_str("Keyboard \\ and |"),
            KeyboardUsage::KeyboardNonUSHashandTilde => {
                f.write_str("Keyboard Non-US Hash and Tilde")
            }
            KeyboardUsage::KeyboardSemicolonandColon => f.write_str("Keyboard ; and :"),
            KeyboardUsage::KeyboardQuoteandDoubleQuote => f.write_str("Keyboard ' and \""),
            KeyboardUsage::KeyboardGraveAccentandTilde => f.write_str("Keyboard ` and ~"),
            KeyboardUsage::KeyboardCommaandLessThan => f.write_str("Keyboard , and <"),
            KeyboardUsage::KeyboardDotandGreatherThan => f.write_str("Keyboard . and >"),
            KeyboardUsage::KeyboardSlashandQuestion => f.write_str("Keyboard / and ?"),
            KeyboardUsage::KeyboardCapsLock => f.write_str("Keyboard Caps Lock"),
            KeyboardUsage::KeyboardF1 => f.write_str("Keyboard F1"),
            KeyboardUsage::KeyboardF2 => f.write_str("Keyboard F2"),
            KeyboardUsage::KeyboardF3 => f.write_str("Keyboard F3"),
            KeyboardUsage::KeyboardF4 => f.write_str("Keyboard F4"),
            KeyboardUsage::KeyboardF5 => f.write_str("Keyboard F5"),
            KeyboardUsage::KeyboardF6 => f.write_str("Keyboard F6"),
            KeyboardUsage::KeyboardF7 => f.write_str("Keyboard F7"),
            KeyboardUsage::KeyboardF8 => f.write_str("Keyboard F8"),
            KeyboardUsage::KeyboardF9 => f.write_str("Keyboard F9"),
            KeyboardUsage::KeyboardF10 => f.write_str("Keyboard F10"),
            KeyboardUsage::KeyboardF11 => f.write_str("Keyboard F11"),
            KeyboardUsage::KeyboardF12 => f.write_str("Keyboard F12"),
            KeyboardUsage::KeyboardPrintScreen => f.write_str("Keyboard Print Screen"),
            KeyboardUsage::KeyboardScrollLock => f.write_str("Keyboard ScrollLock"),
            KeyboardUsage::KeyboardPause => f.write_str("Keyboard Pause"),
            KeyboardUsage::KeyboardInsert => f.write_str("Keyboard Insert"),
            KeyboardUsage::KeyboardHome => f.write_str("Keyboard Home"),
            KeyboardUsage::KeyboardPageUp => f.write_str("Keyboard Page Up"),
            KeyboardUsage::KeyboardDeleteForward => f.write_str("Keyboard Delete Forward"),
            KeyboardUsage::KeyboardEnd => f.write_str("Keyboard End"),
            KeyboardUsage::KeyboardPageDown => f.write_str("Keyboard Page Down"),
            KeyboardUsage::KeyboardRightArrow => f.write_str("Keyboard Right Arrow"),
            KeyboardUsage::KeyboardLeftArrow => f.write_str("Keyboard Left Arrow"),
            KeyboardUsage::KeyboardDownArrow => f.write_str("Keyboard Down Arrow"),
            KeyboardUsage::KeyboardUpArrow => f.write_str("Keyboard Up Arrow"),
            KeyboardUsage::KeypadNumLockandClear => f.write_str("Keypad Num Lock and Clear"),
            KeyboardUsage::KeypadSlash => f.write_str("Keypad /"),
            KeyboardUsage::KeypadStar => f.write_str("Keypad *"),
            KeyboardUsage::KeypadDash => f.write_str("Keypad -"),
            KeyboardUsage::KeypadPlus => f.write_str("Keypad +"),
            KeyboardUsage::KeypadENTER => f.write_str("Keypad ENTER"),
            KeyboardUsage::Keypad1andEnd => f.write_str("Keypad 1 and End"),
            KeyboardUsage::Keypad2andDownArrow => f.write_str("Keypad 2and Down Arrow"),
            KeyboardUsage::Keypad3andPageDn => f.write_str("Keypad 3 and PageDn"),
            KeyboardUsage::Keypad4andLeftArrow => f.write_str("Keypad 4 and Left Arrow"),
            KeyboardUsage::Keypad => f.write_str("Keypad"),
            KeyboardUsage::Keypad6andRightArrow => f.write_str("Keypad 6 and Right Arrow"),
            KeyboardUsage::Keypad7andHome => f.write_str("Keypad 7 and Home"),
            KeyboardUsage::Keypad8andUpArrow => f.write_str("Keypad 8 and Up Arrow"),
            KeyboardUsage::Keypad9andPageUp => f.write_str("Keypad 9 and PageUp"),
            KeyboardUsage::Keypad0andInsert => f.write_str("Keypad 0 and Insert"),
            KeyboardUsage::KeypadDotandDelete => f.write_str("Keypad . and Delete"),
            KeyboardUsage::KeyboardNonUSBackslashandPipe => f.write_str("Keyboard Non-US \\ and |"),
            KeyboardUsage::KeyboardApplication1 => f.write_str("Keyboard Application 1"),
            KeyboardUsage::KeyboardPower => f.write_str("Keyboard Power"),
            KeyboardUsage::KeypadEqual => f.write_str("Keypad ="),
            KeyboardUsage::KeyboardF13 => f.write_str("Keyboard F13"),
            KeyboardUsage::KeyboardF14 => f.write_str("Keyboard F14"),
            KeyboardUsage::KeyboardF15 => f.write_str("Keyboard F15"),
            KeyboardUsage::KeyboardF16 => f.write_str("Keyboard F16"),
            KeyboardUsage::KeyboardF17 => f.write_str("Keyboard F17"),
            KeyboardUsage::KeyboardF18 => f.write_str("Keyboard F18"),
            KeyboardUsage::KeyboardF19 => f.write_str("Keyboard F19"),
            KeyboardUsage::KeyboardF20 => f.write_str("Keyboard F20"),
            KeyboardUsage::KeyboardF21 => f.write_str("Keyboard F21"),
            KeyboardUsage::KeyboardF22 => f.write_str("Keyboard F22"),
            KeyboardUsage::KeyboardF23 => f.write_str("Keyboard F23"),
            KeyboardUsage::KeyboardF24 => f.write_str("Keyboard F24"),
            KeyboardUsage::KeyboardExecute => f.write_str("Keyboard Execute"),
            KeyboardUsage::KeyboardHelp => f.write_str("Keyboard Help"),
            KeyboardUsage::KeyboardMenu => f.write_str("Keyboard Menu"),
            KeyboardUsage::KeyboardSelect => f.write_str("Keyboard Select"),
            KeyboardUsage::KeyboardStop => f.write_str("Keyboard Stop"),
            KeyboardUsage::KeyboardAgain => f.write_str("Keyboard Again"),
            KeyboardUsage::KeyboardUndo => f.write_str("Keyboard Undo"),
            KeyboardUsage::KeyboardCut => f.write_str("Keyboard Cut"),
            KeyboardUsage::KeyboardCopy => f.write_str("Keyboard Copy"),
            KeyboardUsage::KeyboardPaste => f.write_str("Keyboard Paste"),
            KeyboardUsage::KeyboardFind => f.write_str("Keyboard Find"),
            KeyboardUsage::KeyboardMute => f.write_str("Keyboard Mute"),
            KeyboardUsage::KeyboardVolumeUp => f.write_str("Keyboard Volume Up"),
            KeyboardUsage::KeyboardVolumeDown => f.write_str("Keyboard Volume Down"),
            KeyboardUsage::KeyboardLockingCapsLock => f.write_str("Keyboard Locking CapsLock"),
            KeyboardUsage::KeyboardLockingNumLock => f.write_str("Keyboard Locking Num Lock"),
            KeyboardUsage::KeyboardLockingScrollLock => f.write_str("Keyboard Locking Scroll Lock"),
            KeyboardUsage::KeypadComma => f.write_str("Keypad Comma"),
            KeyboardUsage::KeypadEqualSign => f.write_str("Keypad Equal Sign"),
            KeyboardUsage::KeyboardInternational1 => f.write_str("Keyboard International 1"),
            KeyboardUsage::KeyboardInternational2 => f.write_str("Keyboard International 2"),
            KeyboardUsage::KeyboardInternational3 => f.write_str("Keyboard International 3"),
            KeyboardUsage::KeyboardInternational4 => f.write_str("Keyboard International 4"),
            KeyboardUsage::KeyboardInternational5 => f.write_str("Keyboard International 5"),
            KeyboardUsage::KeyboardInternational6 => f.write_str("Keyboard International 6"),
            KeyboardUsage::KeyboardInternational7 => f.write_str("Keyboard International 7"),
            KeyboardUsage::KeyboardInternational8 => f.write_str("Keyboard International 8"),
            KeyboardUsage::KeyboardInternational9 => f.write_str("Keyboard International 9"),
            KeyboardUsage::KeyboardLANG1 => f.write_str("Keyboard LANG 1"),
            KeyboardUsage::KeyboardLANG2 => f.write_str("Keyboard LANG 2"),
            KeyboardUsage::KeyboardLANG3 => f.write_str("Keyboard LANG 3"),
            KeyboardUsage::KeyboardLANG4 => f.write_str("Keyboard LANG 4"),
            KeyboardUsage::KeyboardLANG5 => f.write_str("Keyboard LANG 5"),
            KeyboardUsage::KeyboardLANG6 => f.write_str("Keyboard LANG 6"),
            KeyboardUsage::KeyboardLANG7 => f.write_str("Keyboard LANG 7"),
            KeyboardUsage::KeyboardLANG8 => f.write_str("Keyboard LANG 8"),
            KeyboardUsage::KeyboardLANG9 => f.write_str("Keyboard LANG 9"),
            KeyboardUsage::KeyboardAlternateErase => f.write_str("Keyboard Alternate Erase"),
            KeyboardUsage::KeyboardSysReqAttention => f.write_str("Keyboard Sys/Req Attention"),
            KeyboardUsage::KeyboardCancel => f.write_str("Keyboard Cancel"),
            KeyboardUsage::KeyboardClear => f.write_str("Keyboard Clear"),
            KeyboardUsage::KeyboardPrior => f.write_str("Keyboard Prior"),
            KeyboardUsage::KeyboardReturn => f.write_str("Keyboard Return"),
            KeyboardUsage::KeyboardSeparator => f.write_str("Keyboard Separator"),
            KeyboardUsage::KeyboardOut => f.write_str("Keyboard Out"),
            KeyboardUsage::KeyboardOper => f.write_str("Keyboard Oper"),
            KeyboardUsage::KeyboardClearAgain => f.write_str("Keyboard Clear Again"),
            KeyboardUsage::KeyboardCrSelProps => f.write_str("Keyboard Cr/Sel Props"),
            KeyboardUsage::KeyboardExSel => f.write_str("Keyboard Ex Sel"),
            KeyboardUsage::Keypad00 => f.write_str("Keypad 00"),
            KeyboardUsage::Keypad000 => f.write_str("Keypad 000"),
            KeyboardUsage::ThousandsSeparator => f.write_str("Thousands Separator"),
            KeyboardUsage::DecimalSeparator => f.write_str("Decimal Separator"),
            KeyboardUsage::CurrencyUnit => f.write_str("Currency Unit"),
            KeyboardUsage::CurrencySubunit => f.write_str("Currency Subunit"),
            KeyboardUsage::KeypadLeftParentheses => f.write_str("Keypad ("),
            KeyboardUsage::KeypadRightParentheses => f.write_str("Keypad )"),
            KeyboardUsage::KeypadLeftCurlyBracket => f.write_str("Keypad {"),
            KeyboardUsage::KeypadRightCurlyBracket => f.write_str("Keypad }"),
            KeyboardUsage::KeypadTab => f.write_str("Keypad Tab"),
            KeyboardUsage::KeypadBackspace => f.write_str("Keypad Backspace"),
            KeyboardUsage::KeypadA => f.write_str("Keypad A"),
            KeyboardUsage::KeypadB => f.write_str("Keypad B"),
            KeyboardUsage::KeypadC => f.write_str("Keypad C"),
            KeyboardUsage::KeypadD => f.write_str("Keypad D"),
            KeyboardUsage::KeypadE => f.write_str("Keypad E"),
            KeyboardUsage::KeypadF => f.write_str("Keypad F"),
            KeyboardUsage::KeypadXOR => f.write_str("Keypad XOR"),
            KeyboardUsage::KeypadPower => f.write_str("Keypad ^"),
            KeyboardUsage::KeypadPercent => f.write_str("Keypad %"),
            KeyboardUsage::KeypadLessThan => f.write_str("Keypad <"),
            KeyboardUsage::KeypadGreaterThan => f.write_str("Keypad >"),
            KeyboardUsage::KeypadAmpersand => f.write_str("Keypad &"),
            KeyboardUsage::KeypadDoubleAmpersand => f.write_str("Keypad &&"),
            KeyboardUsage::KeypadPipe => f.write_str("Keypad |"),
            KeyboardUsage::KeypadDoublePipe => f.write_str("Keypad ||"),
            KeyboardUsage::KeypadColon => f.write_str("Keypad :"),
            KeyboardUsage::KeypadHash => f.write_str("Keypad #"),
            KeyboardUsage::KeypadSpace => f.write_str("Keypad Space"),
            KeyboardUsage::KeypadAt => f.write_str("Keypad @"),
            KeyboardUsage::KeypadExclamation => f.write_str("Keypad !"),
            KeyboardUsage::KeypadMemoryStore => f.write_str("Keypad Memory Store"),
            KeyboardUsage::KeypadMemoryRecall => f.write_str("Keypad Memory Recall"),
            KeyboardUsage::KeypadMemoryClear => f.write_str("Keypad Memory Clear"),
            KeyboardUsage::KeypadMemoryAdd => f.write_str("Keypad Memory Add"),
            KeyboardUsage::KeypadMemorySubtract => f.write_str("Keypad Memory Subtract"),
            KeyboardUsage::KeypadMemoryMultiply => f.write_str("Keypad Memory Multiply"),
            KeyboardUsage::KeypadMemoryDivide => f.write_str("Keypad Memory Divide"),
            KeyboardUsage::KeypadPlusMinus => f.write_str("Keypad +/-"),
            KeyboardUsage::KeypadClear => f.write_str("Keypad Clear"),
            KeyboardUsage::KeypadClearEntry => f.write_str("Keypad Clear Entry"),
            KeyboardUsage::KeypadBinary => f.write_str("Keypad Binary"),
            KeyboardUsage::KeypadOctal => f.write_str("Keypad Octal"),
            KeyboardUsage::KeypadDecimal => f.write_str("Keypad Decimal"),
            KeyboardUsage::KeypadHexadecimal => f.write_str("Keypad Hexadecimal"),
            KeyboardUsage::KeyboardLeftControl => f.write_str("Keyboard Left Control"),
            KeyboardUsage::KeyboardLeftShift => f.write_str("Keyboard Left Shift"),
            KeyboardUsage::KeyboardLeftAlt => f.write_str("Keyboard Left Alt"),
            KeyboardUsage::KeyboardLeftGUI => f.write_str("Keyboard Left GUI"),
            KeyboardUsage::KeyboardRightControl => f.write_str("Keyboard Right Control"),
            KeyboardUsage::KeyboardRightShift => f.write_str("Keyboard Right Shift"),
            KeyboardUsage::KeyboardRightAlt => f.write_str("Keyboard Right Alt"),
            KeyboardUsage::KeyboardRightGUI => f.write_str("Keyboard Right GUI"),
            KeyboardUsage::Reserved(i) => write!(f, "Reserved ({})", i),
        }
    }
}

impl fmt::Display for FIDOAllianceUsage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FIDOAllianceUsage::Undefined => f.write_str("Undefined"),
            FIDOAllianceUsage::U2FAuthenticatorDevice => f.write_str("U2F Authenticator Device"),
            FIDOAllianceUsage::InputReportData => f.write_str("Input Report Data"),
            FIDOAllianceUsage::OutputReportData => f.write_str("Output Report Data"),
            FIDOAllianceUsage::Reserved(i) => write!(f, "Reserved ({})", i),
        }
    }
}

impl fmt::Display for ParsedReport {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for field in &self.fields {
            match field {
                Field::ArrayZeroValue(_) => {} // ignore zero value
                _ => writeln!(f, "{}", field)?,
            }
        }

        Ok(())
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Field::ReportId(id) => write!(f, "ReportId({})", id),
            Field::Constant(c) => write!(f, "Constant({})", c),
            Field::Variable(item) => write!(f, "{}", item),
            Field::ArrayValue(item) => {
                write!(f, "{}", item)
            }
            Field::ArrayZeroValue(_) => {
                write!(f, "")
            }
        }
    }
}

impl fmt::Display for VarItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}({})", self.usage_page, self.usage, self.value)
    }
}

impl fmt::Display for ArrayValueItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.usage_page, self.usage)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::report_descriptor::data::Size;

    #[test]
    fn usage_page_item() {
        let item = ReportDescriptorItem {
            kind: ItemType::Global(GlobalType::UsagePage),
            payload_size: Size::One,
            raw: vec![0x05, 0x01],
        };

        assert_eq!(format!("{}", item), "Usage Page (Generic Desktop Controls)");
    }

    #[test]
    fn payload_as_i32_decimal() {
        let bytes: [u8; 1] = [0x11];
        let p = SizedPayload::from(bytes);
        assert_eq!(format!("{}", p), "17");
    }
}
