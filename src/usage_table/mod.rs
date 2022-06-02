#![allow(missing_docs)]
use crate::usage_table::fido::FIDOAllianceUsage;
use crate::usage_table::generic_desktop::GenericDesktopControlsUsage;
use crate::usage_table::keyboard::KeyboardUsage;

pub mod fido;
pub mod generic_desktop;
pub mod keyboard;

pub trait UsageId {
    fn usage_id(self) -> u16;
}

/// UsagePage organizes all related Usage controls
///
/// For definition in the HID protocol see:
/// <https://usb.org/sites/default/files/hut1_3_0.pdf> - page 17
#[derive(Debug, PartialEq, Clone)]
pub enum UsagePage {
    Undefined,
    GenericDesktopControls,
    SimulationControls,
    VRControls,
    SportControls,
    GameControls,
    GenericDeviceControls,
    Keyboard,
    LED,
    Button,
    Ordinal,
    Telephony,
    Consumer,
    Digitizer,
    Haptics,
    PhysicalInputDevice,
    Unicode,
    EyeAndHeadTracker,
    AuxiliaryDisplay,
    Sensor,
    MedicalInstruments,
    BrailleDisplay,
    LightningAndIllumination,
    Monitor,
    MonitorEnumerated,
    VESAVirtualControls,
    Power,
    BatterySystem,
    BarcodeScanner,
    Scale,
    MagneticStripeReader,
    CameraControl,
    Arcade,
    GamingDevice,
    FIDOAlliance,
    VendorDefined(u16),
    Reserved(u16),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Usage {
    Undefined,
    GenericDesktopControls(GenericDesktopControlsUsage),
    SimulationControls(u16),
    VRControls(u16),
    SportControls(u16),
    GameControls(u16),
    GenericDeviceControls(u16),
    Keyboard(KeyboardUsage),
    LED(u16),
    Button(u16),
    Ordinal(u16),
    Telephony(u16),
    Consumer(u16),
    Digitizer(u16),
    Haptics(u16),
    PhysicalInputDevice(u16),
    Unicode(u16),
    EyeAndHeadTracker(u16),
    AuxiliaryDisplay(u16),
    Sensor(u16),
    MedicalInstruments(u16),
    BrailleDisplay(u16),
    LightningAndIllumination(u16),
    Monitor(u16),
    MonitorEnumerated(u16),
    VESAVirtualControls(u16),
    Power(u16),
    BatterySystem(u16),
    BarcodeScanner(u16),
    Scale(u16),
    MagneticStripeReader(u16),
    CameraControl(u16),
    Arcade(u16),
    GamingDevice(u16),
    FIDOAlliance(FIDOAllianceUsage),
    VendorDefined(u16),
}

impl From<u16> for UsagePage {
    fn from(value: u16) -> Self {
        match value {
            0x00 => UsagePage::Undefined,
            0x01 => UsagePage::GenericDesktopControls,
            0x02 => UsagePage::SimulationControls,
            0x03 => UsagePage::VRControls,
            0x04 => UsagePage::SportControls,
            0x05 => UsagePage::GameControls,
            0x06 => UsagePage::GenericDeviceControls,
            0x07 => UsagePage::Keyboard,
            0x08 => UsagePage::LED,
            0x09 => UsagePage::Button,
            0x0A => UsagePage::Ordinal,
            0x0B => UsagePage::Telephony,
            0x0C => UsagePage::Consumer,
            0x0D => UsagePage::Digitizer,
            0x0E => UsagePage::Haptics,
            0x0F => UsagePage::PhysicalInputDevice,
            0x10 => UsagePage::Unicode,
            0x11 => UsagePage::Reserved(0x11),
            0x12 => UsagePage::EyeAndHeadTracker,
            0x13 => UsagePage::Reserved(0x13),
            0x14 => UsagePage::AuxiliaryDisplay,
            i if (0x15..=0x1F).contains(&i) => UsagePage::Reserved(i),
            0x20 => UsagePage::Sensor,
            i if (0x21..=0x3F).contains(&i) => UsagePage::Reserved(i),
            0x40 => UsagePage::MedicalInstruments,
            0x41 => UsagePage::BrailleDisplay,
            i if (0x42..=0x58).contains(&i) => UsagePage::Reserved(i),
            0x59 => UsagePage::LightningAndIllumination,
            i if (0x5A..=0x7F).contains(&i) => UsagePage::Reserved(i),
            0x80 => UsagePage::Monitor,
            0x81 => UsagePage::MonitorEnumerated,
            0x82 => UsagePage::VESAVirtualControls,
            0x83 => UsagePage::Reserved(0x83),
            0x84 => UsagePage::Power,
            0x85 => UsagePage::BatterySystem,
            i if (0x86..=0x8B).contains(&i) => UsagePage::Reserved(i),
            0x8C => UsagePage::BarcodeScanner,
            0x8D => UsagePage::Scale,
            0x8E => UsagePage::MagneticStripeReader,
            0x8F => UsagePage::Reserved(0x8F),
            0x90 => UsagePage::CameraControl,
            0x91 => UsagePage::Arcade,
            0x92 => UsagePage::GamingDevice,
            i if (0x93..=0xF1CF).contains(&i) => UsagePage::Reserved(i),
            0xF1D0 => UsagePage::FIDOAlliance,
            i if (0xF1D1..=0xFEFF).contains(&i) => UsagePage::Reserved(i),
            i => UsagePage::VendorDefined(i),
        }
    }
}

impl From<UsagePage> for u16 {
    fn from(value: UsagePage) -> Self {
        match value {
            UsagePage::Undefined => 0x00,
            UsagePage::GenericDesktopControls => 0x01,
            UsagePage::SimulationControls => 0x02,
            UsagePage::VRControls => 0x03,
            UsagePage::SportControls => 0x04,
            UsagePage::GameControls => 0x05,
            UsagePage::GenericDeviceControls => 0x06,
            UsagePage::Keyboard => 0x07,
            UsagePage::LED => 0x08,
            UsagePage::Button => 0x09,
            UsagePage::Ordinal => 0x0A,
            UsagePage::Telephony => 0x0B,
            UsagePage::Consumer => 0x0C,
            UsagePage::Digitizer => 0x0D,
            UsagePage::Haptics => 0x0E,
            UsagePage::PhysicalInputDevice => 0x0F,
            UsagePage::Unicode => 0x10,
            UsagePage::EyeAndHeadTracker => 0x12,
            UsagePage::AuxiliaryDisplay => 0x14,
            UsagePage::Sensor => 0x20,
            UsagePage::MedicalInstruments => 0x40,
            UsagePage::BrailleDisplay => 0x41,
            UsagePage::LightningAndIllumination => 0x59,
            UsagePage::Monitor => 0x80,
            UsagePage::MonitorEnumerated => 0x81,
            UsagePage::VESAVirtualControls => 0x82,
            UsagePage::Power => 0x84,
            UsagePage::BatterySystem => 0x85,
            UsagePage::BarcodeScanner => 0x8C,
            UsagePage::Scale => 0x8D,
            UsagePage::MagneticStripeReader => 0x8E,
            UsagePage::CameraControl => 0x90,
            UsagePage::Arcade => 0x91,
            UsagePage::GamingDevice => 0x92,
            UsagePage::FIDOAlliance => 0xF1D0,
            UsagePage::VendorDefined(i) => i,
            UsagePage::Reserved(i) => i,
        }
    }
}

impl From<Usage> for u16 {
    fn from(value: Usage) -> Self {
        match value {
            Usage::Undefined => 0x00,
            Usage::GenericDesktopControls(i) => u16::from(i),
            Usage::SimulationControls(i) => i,
            Usage::VRControls(i) => i,
            Usage::SportControls(i) => i,
            Usage::GameControls(i) => i,
            Usage::GenericDeviceControls(i) => i,
            Usage::Keyboard(i) => u16::from(i),
            Usage::LED(i) => i,
            Usage::Button(i) => i,
            Usage::Ordinal(i) => i,
            Usage::Telephony(i) => i,
            Usage::Consumer(i) => i,
            Usage::Digitizer(i) => i,
            Usage::Haptics(i) => i,
            Usage::PhysicalInputDevice(i) => i,
            Usage::Unicode(i) => i,
            Usage::EyeAndHeadTracker(i) => i,
            Usage::AuxiliaryDisplay(i) => i,
            Usage::Sensor(i) => i,
            Usage::MedicalInstruments(i) => i,
            Usage::BrailleDisplay(i) => i,
            Usage::LightningAndIllumination(i) => i,
            Usage::Monitor(i) => i,
            Usage::MonitorEnumerated(i) => i,
            Usage::VESAVirtualControls(i) => i,
            Usage::Power(i) => i,
            Usage::BatterySystem(i) => i,
            Usage::BarcodeScanner(i) => i,
            Usage::Scale(i) => i,
            Usage::MagneticStripeReader(i) => i,
            Usage::CameraControl(i) => i,
            Usage::Arcade(i) => i,
            Usage::GamingDevice(i) => i,
            Usage::FIDOAlliance(i) => u16::from(i),
            Usage::VendorDefined(i) => i,
        }
    }
}

impl From<(&UsagePage, u16)> for Usage {
    fn from(key: (&UsagePage, u16)) -> Self {
        match key.0 {
            UsagePage::Undefined => Usage::Undefined,
            UsagePage::GenericDesktopControls => {
                Usage::GenericDesktopControls(GenericDesktopControlsUsage::from(key.1))
            }
            UsagePage::SimulationControls => Usage::SimulationControls(key.1),
            UsagePage::VRControls => Usage::VRControls(key.1),
            UsagePage::SportControls => Usage::SportControls(key.1),
            UsagePage::GameControls => Usage::GameControls(key.1),
            UsagePage::GenericDeviceControls => Usage::GenericDeviceControls(key.1),
            UsagePage::Keyboard => Usage::Keyboard(KeyboardUsage::from(key.1)),
            UsagePage::LED => Usage::LED(key.1),
            UsagePage::Button => Usage::Button(key.1),
            UsagePage::Ordinal => Usage::Ordinal(key.1),
            UsagePage::Telephony => Usage::Telephony(key.1),
            UsagePage::Consumer => Usage::Consumer(key.1),
            UsagePage::Digitizer => Usage::Digitizer(key.1),
            UsagePage::Haptics => Usage::Haptics(key.1),
            UsagePage::PhysicalInputDevice => Usage::PhysicalInputDevice(key.1),
            UsagePage::Unicode => Usage::Unicode(key.1),
            UsagePage::EyeAndHeadTracker => Usage::EyeAndHeadTracker(key.1),
            UsagePage::AuxiliaryDisplay => Usage::AuxiliaryDisplay(key.1),
            UsagePage::Sensor => Usage::Sensor(key.1),
            UsagePage::MedicalInstruments => Usage::MedicalInstruments(key.1),
            UsagePage::BrailleDisplay => Usage::BrailleDisplay(key.1),
            UsagePage::LightningAndIllumination => Usage::LightningAndIllumination(key.1),
            UsagePage::Monitor => Usage::Monitor(key.1),
            UsagePage::MonitorEnumerated => Usage::MonitorEnumerated(key.1),
            UsagePage::VESAVirtualControls => Usage::VESAVirtualControls(key.1),
            UsagePage::Power => Usage::Power(key.1),
            UsagePage::BatterySystem => Usage::BatterySystem(key.1),
            UsagePage::BarcodeScanner => Usage::BarcodeScanner(key.1),
            UsagePage::Scale => Usage::Scale(key.1),
            UsagePage::MagneticStripeReader => Usage::MagneticStripeReader(key.1),
            UsagePage::CameraControl => Usage::CameraControl(key.1),
            UsagePage::Arcade => Usage::Arcade(key.1),
            UsagePage::GamingDevice => Usage::GamingDevice(key.1),
            UsagePage::FIDOAlliance => Usage::FIDOAlliance(FIDOAllianceUsage::from(key.1)),
            UsagePage::VendorDefined(_) => Usage::VendorDefined(key.1),
            UsagePage::Reserved(_) => Usage::VendorDefined(key.1), // incorrect, but should never happen
        }
    }
}

impl Default for UsagePage {
    fn default() -> Self {
        UsagePage::Undefined
    }
}

impl Default for Usage {
    fn default() -> Self {
        Usage::Undefined
    }
}

impl UsageId for Usage {
    fn usage_id(self) -> u16 {
        self.into()
    }
}
