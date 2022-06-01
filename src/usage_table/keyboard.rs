use crate::usage_table::UsageId;

//https://usb.org/sites/default/files/hut1_3_0.pdf - page 88
#[derive(Clone, Debug, PartialEq)]
pub enum KeyboardUsage {
    KeyboardErrorRollOver,
    KeyboardPOSTFail,
    KeyboardErrorUndefined,
    KeyboardaandA,
    KeyboardbandB,
    KeyboardcandC,
    KeyboarddandD,
    KeyboardeandE,
    KeyboardfandF,
    KeyboardgandG,
    KeyboardhandH,
    KeyboardiandI,
    KeyboardjandJ,
    KeyboardkandK,
    KeyboardlandL,
    KeyboardmandM,
    KeyboardnandN,
    KeyboardoandO,
    KeyboardpandP,
    KeyboardqandQ,
    KeyboardrandR,
    KeyboardsandS,
    KeyboardtandT,
    KeyboarduandU,
    KeyboardvandV,
    KeyboardwandW,
    KeyboardxandX,
    KeyboardyandY,
    KeyboardzandZ,
    Keyboard1andExclamation,
    Keyboard2andAt,
    Keyboard3andHash,
    Keyboard4andDollar,
    Keyboard5andPercent,
    Keyboard6andPower,
    Keyboard7andAmpersand,
    Keyboard8andStar,
    Keyboard9andLeftParentheses,
    Keyboard0andRightParentheses,
    KeyboardReturnEnter,
    KeyboardESCAPE,
    KeyboardDELETE,
    KeyboardTab,
    KeyboardSpacebar,
    KeyboardDashandUnderscore,
    KeyboardEqualandPlus,
    KeyboardLeftSquareBracketandLeftCurlyBracket,
    KeyboardRightSquareBracketandRightCurlyBracket,
    KeyboardBackslashandPipe,
    KeyboardNonUSHashandTilde,
    KeyboardSemicolonandColon,
    KeyboardQuoteandDoubleQuote,
    KeyboardGraveAccentandTilde,
    KeyboardCommaandLessThan,
    KeyboardDotandGreatherThan,
    KeyboardSlashandQuestion,
    KeyboardCapsLock,
    KeyboardF1,
    KeyboardF2,
    KeyboardF3,
    KeyboardF4,
    KeyboardF5,
    KeyboardF6,
    KeyboardF7,
    KeyboardF8,
    KeyboardF9,
    KeyboardF10,
    KeyboardF11,
    KeyboardF12,
    KeyboardPrintScreen,
    KeyboardScrollLock,
    KeyboardPause,
    KeyboardInsert,
    KeyboardHome,
    KeyboardPageUp,
    KeyboardDeleteForward,
    KeyboardEnd,
    KeyboardPageDown,
    KeyboardRightArrow,
    KeyboardLeftArrow,
    KeyboardDownArrow,
    KeyboardUpArrow,
    KeypadNumLockandClear,
    KeypadSlash,
    KeypadStar,
    KeypadDash,
    KeypadPlus,
    KeypadENTER,
    Keypad1andEnd,
    Keypad2andDownArrow,
    Keypad3andPageDn,
    Keypad4andLeftArrow,
    Keypad,
    Keypad6andRightArrow,
    Keypad7andHome,
    Keypad8andUpArrow,
    Keypad9andPageUp,
    Keypad0andInsert,
    KeypadDotandDelete,
    KeyboardNonUSBackslashandPipe,
    KeyboardApplication1,
    KeyboardPower,
    KeypadEqual,
    KeyboardF13,
    KeyboardF14,
    KeyboardF15,
    KeyboardF16,
    KeyboardF17,
    KeyboardF18,
    KeyboardF19,
    KeyboardF20,
    KeyboardF21,
    KeyboardF22,
    KeyboardF23,
    KeyboardF24,
    KeyboardExecute,
    KeyboardHelp,
    KeyboardMenu,
    KeyboardSelect,
    KeyboardStop,
    KeyboardAgain,
    KeyboardUndo,
    KeyboardCut,
    KeyboardCopy,
    KeyboardPaste,
    KeyboardFind,
    KeyboardMute,
    KeyboardVolumeUp,
    KeyboardVolumeDown,
    KeyboardLockingCapsLock,
    KeyboardLockingNumLock,
    KeyboardLockingScrollLock,
    KeypadComma,
    KeypadEqualSign,
    KeyboardInternational1,
    KeyboardInternational2,
    KeyboardInternational3,
    KeyboardInternational4,
    KeyboardInternational5,
    KeyboardInternational6,
    KeyboardInternational7,
    KeyboardInternational8,
    KeyboardInternational9,
    KeyboardLANG1,
    KeyboardLANG2,
    KeyboardLANG3,
    KeyboardLANG4,
    KeyboardLANG5,
    KeyboardLANG6,
    KeyboardLANG7,
    KeyboardLANG8,
    KeyboardLANG9,
    KeyboardAlternateErase,
    KeyboardSysReqAttention,
    KeyboardCancel,
    KeyboardClear,
    KeyboardPrior,
    KeyboardReturn,
    KeyboardSeparator,
    KeyboardOut,
    KeyboardOper,
    KeyboardClearAgain,
    KeyboardCrSelProps,
    KeyboardExSel,
    Keypad00,
    Keypad000,
    ThousandsSeparator,
    DecimalSeparator,
    CurrencyUnit,
    CurrencySubunit,
    KeypadLeftParentheses,
    KeypadRightParentheses,
    KeypadLeftCurlyBracket,
    KeypadRightCurlyBracket,
    KeypadTab,
    KeypadBackspace,
    KeypadA,
    KeypadB,
    KeypadC,
    KeypadD,
    KeypadE,
    KeypadF,
    KeypadXOR,
    KeypadPower,
    KeypadPercent,
    KeypadLessThan,
    KeypadGreaterThan,
    KeypadAmpersand,
    KeypadDoubleAmpersand,
    KeypadPipe,
    KeypadDoublePipe,
    KeypadColon,
    KeypadHash,
    KeypadSpace,
    KeypadAt,
    KeypadExclamation,
    KeypadMemoryStore,
    KeypadMemoryRecall,
    KeypadMemoryClear,
    KeypadMemoryAdd,
    KeypadMemorySubtract,
    KeypadMemoryMultiply,
    KeypadMemoryDivide,
    KeypadPlusMinus,
    KeypadClear,
    KeypadClearEntry,
    KeypadBinary,
    KeypadOctal,
    KeypadDecimal,
    KeypadHexadecimal,
    KeyboardLeftControl,
    KeyboardLeftShift,
    KeyboardLeftAlt,
    KeyboardLeftGUI,
    KeyboardRightControl,
    KeyboardRightShift,
    KeyboardRightAlt,
    KeyboardRightGUI,
    Reserved(u16),
}

impl From<u16> for KeyboardUsage {
    fn from(value: u16) -> Self {
        match value {
            0x01 => KeyboardUsage::KeyboardErrorRollOver,
            0x02 => KeyboardUsage::KeyboardPOSTFail,
            0x03 => KeyboardUsage::KeyboardErrorUndefined,
            0x04 => KeyboardUsage::KeyboardaandA,
            0x05 => KeyboardUsage::KeyboardbandB,
            0x06 => KeyboardUsage::KeyboardcandC,
            0x07 => KeyboardUsage::KeyboarddandD,
            0x08 => KeyboardUsage::KeyboardeandE,
            0x09 => KeyboardUsage::KeyboardfandF,
            0x0A => KeyboardUsage::KeyboardgandG,
            0x0B => KeyboardUsage::KeyboardhandH,
            0x0C => KeyboardUsage::KeyboardiandI,
            0x0D => KeyboardUsage::KeyboardjandJ,
            0x0E => KeyboardUsage::KeyboardkandK,
            0x0F => KeyboardUsage::KeyboardlandL,
            0x10 => KeyboardUsage::KeyboardmandM,
            0x11 => KeyboardUsage::KeyboardnandN,
            0x12 => KeyboardUsage::KeyboardoandO,
            0x13 => KeyboardUsage::KeyboardpandP,
            0x14 => KeyboardUsage::KeyboardqandQ,
            0x15 => KeyboardUsage::KeyboardrandR,
            0x16 => KeyboardUsage::KeyboardsandS,
            0x17 => KeyboardUsage::KeyboardtandT,
            0x18 => KeyboardUsage::KeyboarduandU,
            0x19 => KeyboardUsage::KeyboardvandV,
            0x1A => KeyboardUsage::KeyboardwandW,
            0x1B => KeyboardUsage::KeyboardxandX,
            0x1C => KeyboardUsage::KeyboardyandY,
            0x1D => KeyboardUsage::KeyboardzandZ,
            0x1E => KeyboardUsage::Keyboard1andExclamation,
            0x1F => KeyboardUsage::Keyboard2andAt,
            0x20 => KeyboardUsage::Keyboard3andHash,
            0x21 => KeyboardUsage::Keyboard4andDollar,
            0x22 => KeyboardUsage::Keyboard5andPercent,
            0x23 => KeyboardUsage::Keyboard6andPower,
            0x24 => KeyboardUsage::Keyboard7andAmpersand,
            0x25 => KeyboardUsage::Keyboard8andStar,
            0x26 => KeyboardUsage::Keyboard9andLeftParentheses,
            0x27 => KeyboardUsage::Keyboard0andRightParentheses,
            0x28 => KeyboardUsage::KeyboardReturnEnter,
            0x29 => KeyboardUsage::KeyboardESCAPE,
            0x2A => KeyboardUsage::KeyboardDELETE,
            0x2B => KeyboardUsage::KeyboardTab,
            0x2C => KeyboardUsage::KeyboardSpacebar,
            0x2D => KeyboardUsage::KeyboardDashandUnderscore,
            0x2E => KeyboardUsage::KeyboardEqualandPlus,
            0x2F => KeyboardUsage::KeyboardLeftSquareBracketandLeftCurlyBracket,
            0x30 => KeyboardUsage::KeyboardRightSquareBracketandRightCurlyBracket,
            0x31 => KeyboardUsage::KeyboardBackslashandPipe,
            0x32 => KeyboardUsage::KeyboardNonUSHashandTilde,
            0x33 => KeyboardUsage::KeyboardSemicolonandColon,
            0x34 => KeyboardUsage::KeyboardQuoteandDoubleQuote,
            0x35 => KeyboardUsage::KeyboardGraveAccentandTilde,
            0x36 => KeyboardUsage::KeyboardCommaandLessThan,
            0x37 => KeyboardUsage::KeyboardDotandGreatherThan,
            0x38 => KeyboardUsage::KeyboardSlashandQuestion,
            0x39 => KeyboardUsage::KeyboardCapsLock,
            0x3A => KeyboardUsage::KeyboardF1,
            0x3B => KeyboardUsage::KeyboardF2,
            0x3C => KeyboardUsage::KeyboardF3,
            0x3D => KeyboardUsage::KeyboardF4,
            0x3E => KeyboardUsage::KeyboardF5,
            0x3F => KeyboardUsage::KeyboardF6,
            0x40 => KeyboardUsage::KeyboardF7,
            0x41 => KeyboardUsage::KeyboardF8,
            0x42 => KeyboardUsage::KeyboardF9,
            0x43 => KeyboardUsage::KeyboardF10,
            0x44 => KeyboardUsage::KeyboardF11,
            0x45 => KeyboardUsage::KeyboardF12,
            0x46 => KeyboardUsage::KeyboardPrintScreen,
            0x47 => KeyboardUsage::KeyboardScrollLock,
            0x48 => KeyboardUsage::KeyboardPause,
            0x49 => KeyboardUsage::KeyboardInsert,
            0x4A => KeyboardUsage::KeyboardHome,
            0x4B => KeyboardUsage::KeyboardPageUp,
            0x4C => KeyboardUsage::KeyboardDeleteForward,
            0x4D => KeyboardUsage::KeyboardEnd,
            0x4E => KeyboardUsage::KeyboardPageDown,
            0x4F => KeyboardUsage::KeyboardRightArrow,
            0x50 => KeyboardUsage::KeyboardLeftArrow,
            0x51 => KeyboardUsage::KeyboardDownArrow,
            0x52 => KeyboardUsage::KeyboardUpArrow,
            0x53 => KeyboardUsage::KeypadNumLockandClear,
            0x54 => KeyboardUsage::KeypadSlash,
            0x55 => KeyboardUsage::KeypadStar,
            0x56 => KeyboardUsage::KeypadDash,
            0x57 => KeyboardUsage::KeypadPlus,
            0x58 => KeyboardUsage::KeypadENTER,
            0x59 => KeyboardUsage::Keypad1andEnd,
            0x5A => KeyboardUsage::Keypad2andDownArrow,
            0x5B => KeyboardUsage::Keypad3andPageDn,
            0x5C => KeyboardUsage::Keypad4andLeftArrow,
            0x5D => KeyboardUsage::Keypad,
            0x5E => KeyboardUsage::Keypad6andRightArrow,
            0x5F => KeyboardUsage::Keypad7andHome,
            0x60 => KeyboardUsage::Keypad8andUpArrow,
            0x61 => KeyboardUsage::Keypad9andPageUp,
            0x62 => KeyboardUsage::Keypad0andInsert,
            0x63 => KeyboardUsage::KeypadDotandDelete,
            0x64 => KeyboardUsage::KeyboardNonUSBackslashandPipe,
            0x65 => KeyboardUsage::KeyboardApplication1,
            0x66 => KeyboardUsage::KeyboardPower,
            0x67 => KeyboardUsage::KeypadEqual,
            0x68 => KeyboardUsage::KeyboardF13,
            0x69 => KeyboardUsage::KeyboardF14,
            0x6A => KeyboardUsage::KeyboardF15,
            0x6B => KeyboardUsage::KeyboardF16,
            0x6C => KeyboardUsage::KeyboardF17,
            0x6D => KeyboardUsage::KeyboardF18,
            0x6E => KeyboardUsage::KeyboardF19,
            0x6F => KeyboardUsage::KeyboardF20,
            0x70 => KeyboardUsage::KeyboardF21,
            0x71 => KeyboardUsage::KeyboardF22,
            0x72 => KeyboardUsage::KeyboardF23,
            0x73 => KeyboardUsage::KeyboardF24,
            0x74 => KeyboardUsage::KeyboardExecute,
            0x75 => KeyboardUsage::KeyboardHelp,
            0x76 => KeyboardUsage::KeyboardMenu,
            0x77 => KeyboardUsage::KeyboardSelect,
            0x78 => KeyboardUsage::KeyboardStop,
            0x79 => KeyboardUsage::KeyboardAgain,
            0x7A => KeyboardUsage::KeyboardUndo,
            0x7B => KeyboardUsage::KeyboardCut,
            0x7C => KeyboardUsage::KeyboardCopy,
            0x7D => KeyboardUsage::KeyboardPaste,
            0x7E => KeyboardUsage::KeyboardFind,
            0x7F => KeyboardUsage::KeyboardMute,
            0x80 => KeyboardUsage::KeyboardVolumeUp,
            0x81 => KeyboardUsage::KeyboardVolumeDown,
            0x82 => KeyboardUsage::KeyboardLockingCapsLock,
            0x83 => KeyboardUsage::KeyboardLockingNumLock,
            0x84 => KeyboardUsage::KeyboardLockingScrollLock,
            0x85 => KeyboardUsage::KeypadComma,
            0x86 => KeyboardUsage::KeypadEqualSign,
            0x87 => KeyboardUsage::KeyboardInternational1,
            0x88 => KeyboardUsage::KeyboardInternational2,
            0x89 => KeyboardUsage::KeyboardInternational3,
            0x8A => KeyboardUsage::KeyboardInternational4,
            0x8B => KeyboardUsage::KeyboardInternational5,
            0x8C => KeyboardUsage::KeyboardInternational6,
            0x8D => KeyboardUsage::KeyboardInternational7,
            0x8E => KeyboardUsage::KeyboardInternational8,
            0x8F => KeyboardUsage::KeyboardInternational9,
            0x90 => KeyboardUsage::KeyboardLANG1,
            0x91 => KeyboardUsage::KeyboardLANG2,
            0x92 => KeyboardUsage::KeyboardLANG3,
            0x93 => KeyboardUsage::KeyboardLANG4,
            0x94 => KeyboardUsage::KeyboardLANG5,
            0x95 => KeyboardUsage::KeyboardLANG6,
            0x96 => KeyboardUsage::KeyboardLANG7,
            0x97 => KeyboardUsage::KeyboardLANG8,
            0x98 => KeyboardUsage::KeyboardLANG9,
            0x99 => KeyboardUsage::KeyboardAlternateErase,
            0x9A => KeyboardUsage::KeyboardSysReqAttention,
            0x9B => KeyboardUsage::KeyboardCancel,
            0x9C => KeyboardUsage::KeyboardClear,
            0x9D => KeyboardUsage::KeyboardPrior,
            0x9E => KeyboardUsage::KeyboardReturn,
            0x9F => KeyboardUsage::KeyboardSeparator,
            0xA0 => KeyboardUsage::KeyboardOut,
            0xA1 => KeyboardUsage::KeyboardOper,
            0xA2 => KeyboardUsage::KeyboardClearAgain,
            0xA3 => KeyboardUsage::KeyboardCrSelProps,
            0xA4 => KeyboardUsage::KeyboardExSel,
            0xB0 => KeyboardUsage::Keypad00,
            0xB1 => KeyboardUsage::Keypad000,
            0xB2 => KeyboardUsage::ThousandsSeparator,
            0xB3 => KeyboardUsage::DecimalSeparator,
            0xB4 => KeyboardUsage::CurrencyUnit,
            0xB5 => KeyboardUsage::CurrencySubunit,
            0xB6 => KeyboardUsage::KeypadLeftParentheses,
            0xB7 => KeyboardUsage::KeypadRightParentheses,
            0xB8 => KeyboardUsage::KeypadLeftCurlyBracket,
            0xB9 => KeyboardUsage::KeypadRightCurlyBracket,
            0xBA => KeyboardUsage::KeypadTab,
            0xBB => KeyboardUsage::KeypadBackspace,
            0xBC => KeyboardUsage::KeypadA,
            0xBD => KeyboardUsage::KeypadB,
            0xBE => KeyboardUsage::KeypadC,
            0xBF => KeyboardUsage::KeypadD,
            0xC0 => KeyboardUsage::KeypadE,
            0xC1 => KeyboardUsage::KeypadF,
            0xC2 => KeyboardUsage::KeypadXOR,
            0xC3 => KeyboardUsage::KeypadPower,
            0xC4 => KeyboardUsage::KeypadPercent,
            0xC5 => KeyboardUsage::KeypadLessThan,
            0xC6 => KeyboardUsage::KeypadGreaterThan,
            0xC7 => KeyboardUsage::KeypadAmpersand,
            0xC8 => KeyboardUsage::KeypadDoubleAmpersand,
            0xC9 => KeyboardUsage::KeypadPipe,
            0xCA => KeyboardUsage::KeypadDoublePipe,
            0xCB => KeyboardUsage::KeypadColon,
            0xCC => KeyboardUsage::KeypadHash,
            0xCD => KeyboardUsage::KeypadSpace,
            0xCE => KeyboardUsage::KeypadAt,
            0xCF => KeyboardUsage::KeypadExclamation,
            0xD0 => KeyboardUsage::KeypadMemoryStore,
            0xD1 => KeyboardUsage::KeypadMemoryRecall,
            0xD2 => KeyboardUsage::KeypadMemoryClear,
            0xD3 => KeyboardUsage::KeypadMemoryAdd,
            0xD4 => KeyboardUsage::KeypadMemorySubtract,
            0xD5 => KeyboardUsage::KeypadMemoryMultiply,
            0xD6 => KeyboardUsage::KeypadMemoryDivide,
            0xD7 => KeyboardUsage::KeypadPlusMinus,
            0xD8 => KeyboardUsage::KeypadClear,
            0xD9 => KeyboardUsage::KeypadClearEntry,
            0xDA => KeyboardUsage::KeypadBinary,
            0xDB => KeyboardUsage::KeypadOctal,
            0xDC => KeyboardUsage::KeypadDecimal,
            0xDD => KeyboardUsage::KeypadHexadecimal,
            0xE0 => KeyboardUsage::KeyboardLeftControl,
            0xE1 => KeyboardUsage::KeyboardLeftShift,
            0xE2 => KeyboardUsage::KeyboardLeftAlt,
            0xE3 => KeyboardUsage::KeyboardLeftGUI,
            0xE4 => KeyboardUsage::KeyboardRightControl,
            0xE5 => KeyboardUsage::KeyboardRightShift,
            0xE6 => KeyboardUsage::KeyboardRightAlt,
            0xE7 => KeyboardUsage::KeyboardRightGUI,
            i => KeyboardUsage::Reserved(i),
        }
    }
}

impl From<KeyboardUsage> for u16 {
    fn from(value: KeyboardUsage) -> Self {
        match value {
            KeyboardUsage::KeyboardErrorRollOver => 0x01,
            KeyboardUsage::KeyboardPOSTFail => 0x02,
            KeyboardUsage::KeyboardErrorUndefined => 0x03,
            KeyboardUsage::KeyboardaandA => 0x04,
            KeyboardUsage::KeyboardbandB => 0x05,
            KeyboardUsage::KeyboardcandC => 0x06,
            KeyboardUsage::KeyboarddandD => 0x07,
            KeyboardUsage::KeyboardeandE => 0x08,
            KeyboardUsage::KeyboardfandF => 0x09,
            KeyboardUsage::KeyboardgandG => 0x0A,
            KeyboardUsage::KeyboardhandH => 0x0B,
            KeyboardUsage::KeyboardiandI => 0x0C,
            KeyboardUsage::KeyboardjandJ => 0x0D,
            KeyboardUsage::KeyboardkandK => 0x0E,
            KeyboardUsage::KeyboardlandL => 0x0F,
            KeyboardUsage::KeyboardmandM => 0x10,
            KeyboardUsage::KeyboardnandN => 0x11,
            KeyboardUsage::KeyboardoandO => 0x12,
            KeyboardUsage::KeyboardpandP => 0x13,
            KeyboardUsage::KeyboardqandQ => 0x14,
            KeyboardUsage::KeyboardrandR => 0x15,
            KeyboardUsage::KeyboardsandS => 0x16,
            KeyboardUsage::KeyboardtandT => 0x17,
            KeyboardUsage::KeyboarduandU => 0x18,
            KeyboardUsage::KeyboardvandV => 0x19,
            KeyboardUsage::KeyboardwandW => 0x1A,
            KeyboardUsage::KeyboardxandX => 0x1B,
            KeyboardUsage::KeyboardyandY => 0x1C,
            KeyboardUsage::KeyboardzandZ => 0x1D,
            KeyboardUsage::Keyboard1andExclamation => 0x1E,
            KeyboardUsage::Keyboard2andAt => 0x1F,
            KeyboardUsage::Keyboard3andHash => 0x20,
            KeyboardUsage::Keyboard4andDollar => 0x21,
            KeyboardUsage::Keyboard5andPercent => 0x22,
            KeyboardUsage::Keyboard6andPower => 0x23,
            KeyboardUsage::Keyboard7andAmpersand => 0x24,
            KeyboardUsage::Keyboard8andStar => 0x25,
            KeyboardUsage::Keyboard9andLeftParentheses => 0x26,
            KeyboardUsage::Keyboard0andRightParentheses => 0x27,
            KeyboardUsage::KeyboardReturnEnter => 0x28,
            KeyboardUsage::KeyboardESCAPE => 0x29,
            KeyboardUsage::KeyboardDELETE => 0x2A,
            KeyboardUsage::KeyboardTab => 0x2B,
            KeyboardUsage::KeyboardSpacebar => 0x2C,
            KeyboardUsage::KeyboardDashandUnderscore => 0x2D,
            KeyboardUsage::KeyboardEqualandPlus => 0x2E,
            KeyboardUsage::KeyboardLeftSquareBracketandLeftCurlyBracket => 0x2F,
            KeyboardUsage::KeyboardRightSquareBracketandRightCurlyBracket => 0x30,
            KeyboardUsage::KeyboardBackslashandPipe => 0x31,
            KeyboardUsage::KeyboardNonUSHashandTilde => 0x32,
            KeyboardUsage::KeyboardSemicolonandColon => 0x33,
            KeyboardUsage::KeyboardQuoteandDoubleQuote => 0x34,
            KeyboardUsage::KeyboardGraveAccentandTilde => 0x35,
            KeyboardUsage::KeyboardCommaandLessThan => 0x36,
            KeyboardUsage::KeyboardDotandGreatherThan => 0x37,
            KeyboardUsage::KeyboardSlashandQuestion => 0x38,
            KeyboardUsage::KeyboardCapsLock => 0x39,
            KeyboardUsage::KeyboardF1 => 0x3A,
            KeyboardUsage::KeyboardF2 => 0x3B,
            KeyboardUsage::KeyboardF3 => 0x3C,
            KeyboardUsage::KeyboardF4 => 0x3D,
            KeyboardUsage::KeyboardF5 => 0x3E,
            KeyboardUsage::KeyboardF6 => 0x3F,
            KeyboardUsage::KeyboardF7 => 0x40,
            KeyboardUsage::KeyboardF8 => 0x41,
            KeyboardUsage::KeyboardF9 => 0x42,
            KeyboardUsage::KeyboardF10 => 0x43,
            KeyboardUsage::KeyboardF11 => 0x44,
            KeyboardUsage::KeyboardF12 => 0x45,
            KeyboardUsage::KeyboardPrintScreen => 0x46,
            KeyboardUsage::KeyboardScrollLock => 0x47,
            KeyboardUsage::KeyboardPause => 0x48,
            KeyboardUsage::KeyboardInsert => 0x49,
            KeyboardUsage::KeyboardHome => 0x4A,
            KeyboardUsage::KeyboardPageUp => 0x4B,
            KeyboardUsage::KeyboardDeleteForward => 0x4C,
            KeyboardUsage::KeyboardEnd => 0x4D,
            KeyboardUsage::KeyboardPageDown => 0x4E,
            KeyboardUsage::KeyboardRightArrow => 0x4F,
            KeyboardUsage::KeyboardLeftArrow => 0x50,
            KeyboardUsage::KeyboardDownArrow => 0x51,
            KeyboardUsage::KeyboardUpArrow => 0x52,
            KeyboardUsage::KeypadNumLockandClear => 0x53,
            KeyboardUsage::KeypadSlash => 0x54,
            KeyboardUsage::KeypadStar => 0x55,
            KeyboardUsage::KeypadDash => 0x56,
            KeyboardUsage::KeypadPlus => 0x57,
            KeyboardUsage::KeypadENTER => 0x58,
            KeyboardUsage::Keypad1andEnd => 0x59,
            KeyboardUsage::Keypad2andDownArrow => 0x5A,
            KeyboardUsage::Keypad3andPageDn => 0x5B,
            KeyboardUsage::Keypad4andLeftArrow => 0x5C,
            KeyboardUsage::Keypad => 0x5D,
            KeyboardUsage::Keypad6andRightArrow => 0x5E,
            KeyboardUsage::Keypad7andHome => 0x5F,
            KeyboardUsage::Keypad8andUpArrow => 0x60,
            KeyboardUsage::Keypad9andPageUp => 0x61,
            KeyboardUsage::Keypad0andInsert => 0x62,
            KeyboardUsage::KeypadDotandDelete => 0x63,
            KeyboardUsage::KeyboardNonUSBackslashandPipe => 0x64,
            KeyboardUsage::KeyboardApplication1 => 0x65,
            KeyboardUsage::KeyboardPower => 0x66,
            KeyboardUsage::KeypadEqual => 0x67,
            KeyboardUsage::KeyboardF13 => 0x68,
            KeyboardUsage::KeyboardF14 => 0x69,
            KeyboardUsage::KeyboardF15 => 0x6A,
            KeyboardUsage::KeyboardF16 => 0x6B,
            KeyboardUsage::KeyboardF17 => 0x6C,
            KeyboardUsage::KeyboardF18 => 0x6D,
            KeyboardUsage::KeyboardF19 => 0x6E,
            KeyboardUsage::KeyboardF20 => 0x6F,
            KeyboardUsage::KeyboardF21 => 0x70,
            KeyboardUsage::KeyboardF22 => 0x71,
            KeyboardUsage::KeyboardF23 => 0x72,
            KeyboardUsage::KeyboardF24 => 0x73,
            KeyboardUsage::KeyboardExecute => 0x74,
            KeyboardUsage::KeyboardHelp => 0x75,
            KeyboardUsage::KeyboardMenu => 0x76,
            KeyboardUsage::KeyboardSelect => 0x77,
            KeyboardUsage::KeyboardStop => 0x78,
            KeyboardUsage::KeyboardAgain => 0x79,
            KeyboardUsage::KeyboardUndo => 0x7A,
            KeyboardUsage::KeyboardCut => 0x7B,
            KeyboardUsage::KeyboardCopy => 0x7C,
            KeyboardUsage::KeyboardPaste => 0x7D,
            KeyboardUsage::KeyboardFind => 0x7E,
            KeyboardUsage::KeyboardMute => 0x7F,
            KeyboardUsage::KeyboardVolumeUp => 0x80,
            KeyboardUsage::KeyboardVolumeDown => 0x81,
            KeyboardUsage::KeyboardLockingCapsLock => 0x82,
            KeyboardUsage::KeyboardLockingNumLock => 0x83,
            KeyboardUsage::KeyboardLockingScrollLock => 0x84,
            KeyboardUsage::KeypadComma => 0x85,
            KeyboardUsage::KeypadEqualSign => 0x86,
            KeyboardUsage::KeyboardInternational1 => 0x87,
            KeyboardUsage::KeyboardInternational2 => 0x88,
            KeyboardUsage::KeyboardInternational3 => 0x89,
            KeyboardUsage::KeyboardInternational4 => 0x8A,
            KeyboardUsage::KeyboardInternational5 => 0x8B,
            KeyboardUsage::KeyboardInternational6 => 0x8C,
            KeyboardUsage::KeyboardInternational7 => 0x8D,
            KeyboardUsage::KeyboardInternational8 => 0x8E,
            KeyboardUsage::KeyboardInternational9 => 0x8F,
            KeyboardUsage::KeyboardLANG1 => 0x90,
            KeyboardUsage::KeyboardLANG2 => 0x91,
            KeyboardUsage::KeyboardLANG3 => 0x92,
            KeyboardUsage::KeyboardLANG4 => 0x93,
            KeyboardUsage::KeyboardLANG5 => 0x94,
            KeyboardUsage::KeyboardLANG6 => 0x95,
            KeyboardUsage::KeyboardLANG7 => 0x96,
            KeyboardUsage::KeyboardLANG8 => 0x97,
            KeyboardUsage::KeyboardLANG9 => 0x98,
            KeyboardUsage::KeyboardAlternateErase => 0x99,
            KeyboardUsage::KeyboardSysReqAttention => 0x9A,
            KeyboardUsage::KeyboardCancel => 0x9B,
            KeyboardUsage::KeyboardClear => 0x9C,
            KeyboardUsage::KeyboardPrior => 0x9D,
            KeyboardUsage::KeyboardReturn => 0x9E,
            KeyboardUsage::KeyboardSeparator => 0x9F,
            KeyboardUsage::KeyboardOut => 0xA0,
            KeyboardUsage::KeyboardOper => 0xA1,
            KeyboardUsage::KeyboardClearAgain => 0xA2,
            KeyboardUsage::KeyboardCrSelProps => 0xA3,
            KeyboardUsage::KeyboardExSel => 0xA4,
            KeyboardUsage::Keypad00 => 0xB0,
            KeyboardUsage::Keypad000 => 0xB1,
            KeyboardUsage::ThousandsSeparator => 0xB2,
            KeyboardUsage::DecimalSeparator => 0xB3,
            KeyboardUsage::CurrencyUnit => 0xB4,
            KeyboardUsage::CurrencySubunit => 0xB5,
            KeyboardUsage::KeypadLeftParentheses => 0xB6,
            KeyboardUsage::KeypadRightParentheses => 0xB7,
            KeyboardUsage::KeypadLeftCurlyBracket => 0xB8,
            KeyboardUsage::KeypadRightCurlyBracket => 0xB9,
            KeyboardUsage::KeypadTab => 0xBA,
            KeyboardUsage::KeypadBackspace => 0xBB,
            KeyboardUsage::KeypadA => 0xBC,
            KeyboardUsage::KeypadB => 0xBD,
            KeyboardUsage::KeypadC => 0xBE,
            KeyboardUsage::KeypadD => 0xBF,
            KeyboardUsage::KeypadE => 0xC0,
            KeyboardUsage::KeypadF => 0xC1,
            KeyboardUsage::KeypadXOR => 0xC2,
            KeyboardUsage::KeypadPower => 0xC3,
            KeyboardUsage::KeypadPercent => 0xC4,
            KeyboardUsage::KeypadLessThan => 0xC5,
            KeyboardUsage::KeypadGreaterThan => 0xC6,
            KeyboardUsage::KeypadAmpersand => 0xC7,
            KeyboardUsage::KeypadDoubleAmpersand => 0xC8,
            KeyboardUsage::KeypadPipe => 0xC9,
            KeyboardUsage::KeypadDoublePipe => 0xCA,
            KeyboardUsage::KeypadColon => 0xCB,
            KeyboardUsage::KeypadHash => 0xCC,
            KeyboardUsage::KeypadSpace => 0xCD,
            KeyboardUsage::KeypadAt => 0xCE,
            KeyboardUsage::KeypadExclamation => 0xCF,
            KeyboardUsage::KeypadMemoryStore => 0xD0,
            KeyboardUsage::KeypadMemoryRecall => 0xD1,
            KeyboardUsage::KeypadMemoryClear => 0xD2,
            KeyboardUsage::KeypadMemoryAdd => 0xD3,
            KeyboardUsage::KeypadMemorySubtract => 0xD4,
            KeyboardUsage::KeypadMemoryMultiply => 0xD5,
            KeyboardUsage::KeypadMemoryDivide => 0xD6,
            KeyboardUsage::KeypadPlusMinus => 0xD7,
            KeyboardUsage::KeypadClear => 0xD8,
            KeyboardUsage::KeypadClearEntry => 0xD9,
            KeyboardUsage::KeypadBinary => 0xDA,
            KeyboardUsage::KeypadOctal => 0xDB,
            KeyboardUsage::KeypadDecimal => 0xDC,
            KeyboardUsage::KeypadHexadecimal => 0xDD,
            KeyboardUsage::KeyboardLeftControl => 0xE0,
            KeyboardUsage::KeyboardLeftShift => 0xE1,
            KeyboardUsage::KeyboardLeftAlt => 0xE2,
            KeyboardUsage::KeyboardLeftGUI => 0xE3,
            KeyboardUsage::KeyboardRightControl => 0xE4,
            KeyboardUsage::KeyboardRightShift => 0xE5,
            KeyboardUsage::KeyboardRightAlt => 0xE6,
            KeyboardUsage::KeyboardRightGUI => 0xE7,
            KeyboardUsage::Reserved(i) => i,
        }
    }
}

impl UsageId for KeyboardUsage {
    fn usage_id(self) -> u16 {
        self.into()
    }
}
