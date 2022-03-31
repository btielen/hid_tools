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
