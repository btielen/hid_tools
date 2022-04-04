use crate::usage_table::UsageId;

#[derive(Clone)]
pub enum FIDOAllianceUsage {
    Undefined,
    U2FAuthenticatorDevice,
    InputReportData,
    OutputReportData,
    Reserved(u16),
}

impl From<u16> for FIDOAllianceUsage {
    fn from(value: u16) -> Self {
        match value {
            0x00 => FIDOAllianceUsage::Undefined,
            0x01 => FIDOAllianceUsage::U2FAuthenticatorDevice,
            0x20 => FIDOAllianceUsage::InputReportData,
            0x21 => FIDOAllianceUsage::OutputReportData,
            i => FIDOAllianceUsage::Reserved(i),
        }
    }
}

impl From<FIDOAllianceUsage> for u16 {
    fn from(value: FIDOAllianceUsage) -> Self {
        match value {
            FIDOAllianceUsage::Undefined => 0x00,
            FIDOAllianceUsage::U2FAuthenticatorDevice => 0x01,
            FIDOAllianceUsage::InputReportData => 0x20,
            FIDOAllianceUsage::OutputReportData => 0x21,
            FIDOAllianceUsage::Reserved(i) => i,
        }
    }
}

impl UsageId for FIDOAllianceUsage {
    fn usage_id(self) -> u16 {
        self.into()
    }
}
