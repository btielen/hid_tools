use crate::usage_table::UsageId;

/// Consumer usage
///
///
/// <https://usb.org/sites/default/files/hut1_3_0.pdf> - page 123
#[derive(Clone, Debug, PartialEq)]
pub enum ConsumerUsage {
    Undefined,
    ConsumerControl,
    NumericKeyPad,
    ProgrammableButtons,
    Microphone,
    Headphone,
    GraphicEqualizer,
    Plus10,
    Plus100,
    AmPm,
    Power,
    Reset,
    Sleep,
    SleepAfter,
    SleepMode,
    Illumination,
    FunctionButtons,
    Menu,
    MenuPick,
    MenuUp,
    MenuDown,
    MenuLeft,
    MenuRight,
    MenuEscape,
    MenuValueIncrease,
    MenuValueDecrease,
    DataOnScreen,
    ClosedCaption,
    ClosedCaptionSelect,
    VcrTv,
    BroadcastMode,
    Snapshot,
    Still,
    PictureInPictureToggle,
    PictureInPictureSwap,
    RedMenuButton,
    GreenMenuButton,
    BlueMenuButton,
    YellowMenuButton,
    Aspect,
    Mode3DSelect,
    DisplayBrightnessIncrement,
    DisplayBrightnessDecrement,
    DisplayBrightness,
    DisplayBacklightToggle,
    DisplaySetBrightnessToMinimum,
    DisplaySetBrightnessToMaximum,
    DisplaySetAutoBrightness,
    CameraAccessEnabled,
    CameraAccessDisabled,
    CameraAccessToggle,
    KeyboardBrightnessIncrement,
    KeyboardBrightnessDecrement,
    KeyboardBacklightSetLevel,
    KeyboardBacklightOOC,
    KeyboardBacklightSetMinimum,
    KeyboardBacklightSetMaximum,
    KeyboardBacklightAuto,
    Selection,
    AssignSelection,
    ModeStep,
    RecallLast,
    EnterChannel,
    OrderMovie,
    Channel,
    MediaSelection,
    MediaSelectComputer,
    MediaSelectTV,
    MediaSelectWWW,
    MediaSelectDVD,
    MediaSelectTelephone,
    MediaSelectProgramGuide,
    MediaSelectVideoPhone,
    MediaSelectGames,
    MediaSelectMessages,
    MediaSelectCD,
    MediaSelectVCR,
    MediaSelectTuner,
    Quit,
    Help,
    MediaSelectTape,
    MediaSelectCable,
    MediaSelectSatellite,
    MediaSelectSecurity,
    MediaSelectHome,
    MediaSelectCall,
    ChannelIncrement,
    ChannelDecrement,
    MediaSelectSAP,
    VCRPlus,
    Once,
    Daily,
    Weekly,
    Monthly,
    Play,
    Pause,
    Record,
    FastForward,
    Rewind,
    ScanNextTrack,
    ScanPreviousTrack,
    Stop,
    Eject,
    RandomPlay,
    SelectDisc,
    EnterDisc,
    Repeat,
    Tracking,
    TrackNormal,
    SlowTracking,
    FrameForward,
    FrameBack,
    Mark,
    ClearMark,
    RepeatFromMark,
    ReturnToMark,
    SearchMarkForward,
    SearchMarkBackwards,
    CounterReset,
    ShowCounter,
    TrackingIncrement,
    TrackingDecrement,
    StopEject,
    PlayPause,
    PlaySkip,
    VoiceCommand,
    InvokeCaptureInterface,
    StartOrStopGameRecording,
    HistoricalGameCapture,
    CaptureGameScreenshot,
    ShowOrHideRecordingIndicator,
    StartOrStopMicrophoneCapture,
    StartOrStopCameraCapture,
    StartOrStopGameBroadcast,
    StartOrStopVoiceDictationSession,
    InvokeDismissEmojiPicker,
    Volume,
    Balance,
    Mute,
    Bass,
    Treble,
    BassBoost,
    SurroundMode,
    Loudness,
    MPX,
    VolumeIncrement,
    VolumeDecrement,
    SpeedSelect,
    PlaybackSpeed,
    StandardPlay,
    LongPlay,
    ExtendedPlay,
    Slow,
    FanEnable,
    FanSpeed,
    LightEnable,
    LightIlluminationLevel,
    ClimateControlEnable,
    RoomTemperature,
    SecurityEnable,
    FireAlarm,
    PoliceAlarm,
    Proximity,
    Motion,
    DuressAlarm,
    HoldupAlarm,
    MedicalAlarm,
    BalanceRight,
    BalanceLeft,
    BassIncrement,
    BassDecrement,
    TrebleIncrement,
    TrebleDecrement,
    SpeakerSystem,
    ChannelLeft,
    ChannelRight,
    ChannelCenter,
    ChannelFront,
    ChannelCenterFront,
    ChannelSide,
    ChannelSurround,
    ChannelLowFrequencyEnhancement,
    ChannelTop,
    ChannelUnknown,
    SubChannel,
    SubChannelIncrement,
    SubChannelDecrement,
    AlternateAudioIncrement,
    AlternateAudioDecrement,
    ApplicationLaunchButtons,
    ALLaunchButtonConfigurationTool,
    ALProgrammableButtonConfiguration,
    ALConsumerControlConfiguration,
    ALWordProcessor,
    ALTextEditor,
    ALSpreadsheet,
    ALGraphicsEditor,
    ALPresentationApp,
    ALDatabaseApp,
    ALEmailReader,
    ALNewsreader,
    ALVoicemail,
    ALContactsAddressBook,
    ALCalendarSchedule,
    ALTaskProjectManager,
    ALLogJournalTimecard,
    ALCheckbookFinance,
    ALCalculator,
    ALAVCapturePlayback,
    ALLocalMachineBrowser,
    ALLANWANBrowser,
    ALInternetBrowser,
    ALRemoteNetworkingISPConnect,
    ALNetworkConference,
    ALNetworkChat,
    ALTelephonyDialer,
    ALLogon,
    ALLogoff,
    ALLogonLogoff,
    ALTerminalLockScreensaver,
    ALControlPanel,
    ALCommandLineProcessorRun,
    ALProcessTaskManager,
    ALSelectTaskApplication,
    ALNextTaskApplication,
    ALPreviousTaskApplication,
    ALPreemptiveHaltTaskApplication,
    ALIntegratedHelpCenter,
    ALDocuments,
    ALThesaurus,
    ALDictionary,
    ALDesktop,
    ALSpellCheck,
    ALGrammarCheck,
    ALWirelessStatus,
    ALKeyboardLayout,
    ALVirusProtection,
    ALEncryption,
    ALScreenSaver,
    ALAlarms,
    ALClock,
    ALFileBrowser,
    ALPowerStatus,
    ALImageBrowser,
    ALAudioBrowser,
    ALMovieBrowser,
    ALDigitalRightsManager,
    ALDigitalWallet,
    ALInstantMessaging,
    ALOEMFeaturesTipsTutorialBrowser,
    ALOEMHelp,
    ALOnlineCommunity,
    ALEntertainmentContentBrowser,
    ALOnlineShoppingBrowser,
    ALSmartCardInformationHelp,
    ALMarketMonitorFinanceBrowser,
    ALCustomizedCorporateNewsBrowser,
    ALOnlineActivityBrowser,
    ALResearchSearchBrowser,
    ALAudioPlayer,
    ALMessageStatus,
    ALContactSync,
    ALNavigation,
    ALContextawareDesktopAssistant,
    GenericGUIApplicationControls,
    ACNew,
    ACOpen,
    ACClose,
    ACExit,
    ACMaximize,
    ACMinimize,
    ACSave,
    ACPrint,
    ACProperties,
    ACUndo,
    ACCopy,
    ACCut,
    ACPaste,
    ACSelectAll,
    ACFind,
    ACFindAndReplace,
    ACSearch,
    ACGoTo,
    ACHome,
    ACBack,
    ACForward,
    ACStop,
    ACRefresh,
    ACPreviousLink,
    ACNextLink,
    ACBookmarks,
    ACHistory,
    ACSubscriptions,
    ACZoomIn,
    ACZoomOut,
    ACZoom,
    ACFullScreenView,
    ACNormalView,
    ACViewToggle,
    ACScrollUp,
    ACScrollDown,
    ACScroll,
    ACPanLeft,
    ACPanRight,
    ACPan,
    ACNewWindow,
    ACTileHorizontally,
    ACTileVertically,
    ACFormat,
    ACEdit,
    ACBold,
    ACItalics,
    ACUnderline,
    ACStrikethrough,
    ACSubscript,
    ACSuperscript,
    ACAllCaps,
    ACRotate,
    ACResize,
    ACFlipHorizontal,
    ACFlipVertical,
    ACMirrorHorizontal,
    ACMirrorVertical,
    ACFontSelect,
    ACFontColor,
    ACFontSize,
    ACJustifyLeft,
    ACJustifyCenterH,
    ACJustifyRight,
    ACJustifyBlockH,
    ACJustifyTop,
    ACJustifyCenterV,
    ACJustifyBottom,
    ACJustifyBlockV,
    ACIndentDecrease,
    ACIndentIncrease,
    ACNumberedList,
    ACRestartNumbering,
    ACBulletedList,
    ACPromote,
    ACDemote,
    ACYes,
    ACNo,
    ACCancel,
    ACCatalog,
    ACBuyCheckout,
    ACAddToCart,
    ACExpand,
    ACExpandAll,
    ACCollapse,
    ACCollapseAll,
    ACPrintPreview,
    ACPasteSpecial,
    ACInsertMode,
    ACDelete,
    ACLock,
    ACUnlock,
    ACProtect,
    ACUnprotect,
    ACAttachComment,
    ACDeleteComment,
    ACViewComment,
    ACSelectWord,
    ACSelectSentence,
    ACSelectParagraph,
    ACSelectColumn,
    ACSelectRow,
    ACSelectTable,
    ACSelectObject,
    ACRedoRepeat,
    ACSort,
    ACSortAscending,
    ACSortDescending,
    ACFilter,
    ACSetClock,
    ACViewClock,
    ACSelectTimeZone,
    ACEditTimeZones,
    ACSetAlarm,
    ACClearAlarm,
    ACSnoozeAlarm,
    ACResetAlarm,
    ACSynchronize,
    ACSendReceive,
    ACSendTo,
    ACReply,
    ACReplyAll,
    ACForwardMsg,
    ACSend,
    ACAttachFile,
    ACUpload,
    ACDownloadSaveTargetAs,
    ACSetBorders,
    ACInsertRow,
    ACInsertColumn,
    ACInsertFile,
    ACInsertPicture,
    ACInsertObject,
    ACInsertSymbol,
    ACSaveAndClose,
    ACRename,
    ACMerge,
    ACSplit,
    ACDistributeHorizontally,
    ACDistributeVertically,
    ACNextKeyboardLayoutSelect,
    ACNavigationGuidance,
    ACDesktopShowAllWindows,
    ACSoftKeyLeft,
    ACSoftKeyRight,
    ACDesktopShowAllApplications,
    ACIdleKeepAlive,
    ExtendedKeyboardAttributesCollection,
    KeyboardFormFactor,
    KeyboardKeyType,
    KeyboardPhysicalLayout,
    VendorSpecificKeyboardPhysicalLayout,
    KeyboardIETFLanguageTagIndex,
    ImplementedKeyboardInputAssistControls,
    KeyboardInputAssistPrevious,
    KeyboardInputAssistNext,
    KeyboardInputAssistPreviousGroup,
    KeyboardInputAssistNextGroup,
    KeyboardInputAssistAccept,
    KeyboardInputAssistCancel,
    PrivacyScreenToggle,
    PrivacyScreenLevelDecrement,
    PrivacyScreenLevelIncrement,
    PrivacyScreenLevelMinimum,
    PrivacyScreenLevelMaximum,
    ContactEdited,
    ContactAdded,
    ContactRecordActive,
    ContactIndex,
    ContactNickname,
    ContactFirstName,
    ContactLastName,
    ContactFullName,
    ContactPhoneNumberPersonal,
    ContactPhoneNumberBusiness,
    ContactPhoneNumberMobile,
    ContactPhoneNumberPager,
    ContactPhoneNumberFax,
    ContactPhoneNumberOther,
    ContactEmailPersonal,
    ContactEmailBusiness,
    ContactEmailOther,
    ContactEmailMain,
    ContactSpeedDialNumber,
    ContactStatusFlag,
    ContactMisc,
    Reserved(u16),
}

impl From<u16> for ConsumerUsage {
    fn from(value: u16) -> Self {
        match value {
            0x00 => ConsumerUsage::Undefined,
            0x01 => ConsumerUsage::ConsumerControl,
            0x02 => ConsumerUsage::NumericKeyPad,
            0x03 => ConsumerUsage::ProgrammableButtons,
            0x04 => ConsumerUsage::Microphone,
            0x05 => ConsumerUsage::Headphone,
            0x06 => ConsumerUsage::GraphicEqualizer,
            0x20 => ConsumerUsage::Plus10,
            0x21 => ConsumerUsage::Plus100,
            0x22 => ConsumerUsage::AmPm,
            0x30 => ConsumerUsage::Power,
            0x31 => ConsumerUsage::Reset,
            0x32 => ConsumerUsage::Sleep,
            0x33 => ConsumerUsage::SleepAfter,
            0x34 => ConsumerUsage::SleepMode,
            0x35 => ConsumerUsage::Illumination,
            0x36 => ConsumerUsage::FunctionButtons,
            0x40 => ConsumerUsage::Menu,
            0x41 => ConsumerUsage::MenuPick,
            0x42 => ConsumerUsage::MenuUp,
            0x43 => ConsumerUsage::MenuDown,
            0x44 => ConsumerUsage::MenuLeft,
            0x45 => ConsumerUsage::MenuRight,
            0x46 => ConsumerUsage::MenuEscape,
            0x47 => ConsumerUsage::MenuValueIncrease,
            0x48 => ConsumerUsage::MenuValueDecrease,
            0x60 => ConsumerUsage::DataOnScreen,
            0x61 => ConsumerUsage::ClosedCaption,
            0x62 => ConsumerUsage::ClosedCaptionSelect,
            0x63 => ConsumerUsage::VcrTv,
            0x64 => ConsumerUsage::BroadcastMode,
            0x65 => ConsumerUsage::Snapshot,
            0x66 => ConsumerUsage::Still,
            0x67 => ConsumerUsage::PictureInPictureToggle,
            0x68 => ConsumerUsage::PictureInPictureSwap,
            0x69 => ConsumerUsage::RedMenuButton,
            0x6A => ConsumerUsage::GreenMenuButton,
            0x6B => ConsumerUsage::BlueMenuButton,
            0x6C => ConsumerUsage::YellowMenuButton,
            0x6D => ConsumerUsage::Aspect,
            0x6E => ConsumerUsage::Mode3DSelect,
            0x6F => ConsumerUsage::DisplayBrightnessIncrement,
            0x70 => ConsumerUsage::DisplayBrightnessDecrement,
            0x71 => ConsumerUsage::DisplayBrightness,
            0x72 => ConsumerUsage::DisplayBacklightToggle,
            0x73 => ConsumerUsage::DisplaySetBrightnessToMinimum,
            0x74 => ConsumerUsage::DisplaySetBrightnessToMaximum,
            0x75 => ConsumerUsage::DisplaySetAutoBrightness,
            0x76 => ConsumerUsage::CameraAccessEnabled,
            0x77 => ConsumerUsage::CameraAccessDisabled,
            0x78 => ConsumerUsage::CameraAccessToggle,
            0x79 => ConsumerUsage::KeyboardBrightnessIncrement,
            0x7A => ConsumerUsage::KeyboardBrightnessDecrement,
            0x7B => ConsumerUsage::KeyboardBacklightSetLevel,
            0x7C => ConsumerUsage::KeyboardBacklightOOC,
            0x7D => ConsumerUsage::KeyboardBacklightSetMinimum,
            0x7E => ConsumerUsage::KeyboardBacklightSetMaximum,
            0x7F => ConsumerUsage::KeyboardBacklightAuto,
            0x80 => ConsumerUsage::Selection,
            0x81 => ConsumerUsage::AssignSelection,
            0x82 => ConsumerUsage::ModeStep,
            0x83 => ConsumerUsage::RecallLast,
            0x84 => ConsumerUsage::EnterChannel,
            0x85 => ConsumerUsage::OrderMovie,
            0x86 => ConsumerUsage::Channel,
            0x87 => ConsumerUsage::MediaSelection,
            0x88 => ConsumerUsage::MediaSelectComputer,
            0x89 => ConsumerUsage::MediaSelectTV,
            0x8A => ConsumerUsage::MediaSelectWWW,
            0x8B => ConsumerUsage::MediaSelectDVD,
            0x8C => ConsumerUsage::MediaSelectTelephone,
            0x8D => ConsumerUsage::MediaSelectProgramGuide,
            0x8E => ConsumerUsage::MediaSelectVideoPhone,
            0x8F => ConsumerUsage::MediaSelectGames,
            0x90 => ConsumerUsage::MediaSelectMessages,
            0x91 => ConsumerUsage::MediaSelectCD,
            0x92 => ConsumerUsage::MediaSelectVCR,
            0x93 => ConsumerUsage::MediaSelectTuner,
            0x94 => ConsumerUsage::Quit,
            0x95 => ConsumerUsage::Help,
            0x96 => ConsumerUsage::MediaSelectTape,
            0x97 => ConsumerUsage::MediaSelectCable,
            0x98 => ConsumerUsage::MediaSelectSatellite,
            0x99 => ConsumerUsage::MediaSelectSecurity,
            0x9A => ConsumerUsage::MediaSelectHome,
            0x9B => ConsumerUsage::MediaSelectCall,
            0x9C => ConsumerUsage::ChannelIncrement,
            0x9D => ConsumerUsage::ChannelDecrement,
            0x9E => ConsumerUsage::MediaSelectSAP,
            0xA0 => ConsumerUsage::VCRPlus,
            0xA1 => ConsumerUsage::Once,
            0xA2 => ConsumerUsage::Daily,
            0xA3 => ConsumerUsage::Weekly,
            0xA4 => ConsumerUsage::Monthly,
            0xB0 => ConsumerUsage::Play,
            0xB1 => ConsumerUsage::Pause,
            0xB2 => ConsumerUsage::Record,
            0xB3 => ConsumerUsage::FastForward,
            0xB4 => ConsumerUsage::Rewind,
            0xB5 => ConsumerUsage::ScanNextTrack,
            0xB6 => ConsumerUsage::ScanPreviousTrack,
            0xB7 => ConsumerUsage::Stop,
            0xB8 => ConsumerUsage::Eject,
            0xB9 => ConsumerUsage::RandomPlay,
            0xBA => ConsumerUsage::SelectDisc,
            0xBB => ConsumerUsage::EnterDisc,
            0xBC => ConsumerUsage::Repeat,
            0xBD => ConsumerUsage::Tracking,
            0xBE => ConsumerUsage::TrackNormal,
            0xBF => ConsumerUsage::SlowTracking,
            0xC0 => ConsumerUsage::FrameForward,
            0xC1 => ConsumerUsage::FrameBack,
            0xC2 => ConsumerUsage::Mark,
            0xC3 => ConsumerUsage::ClearMark,
            0xC4 => ConsumerUsage::RepeatFromMark,
            0xC5 => ConsumerUsage::ReturnToMark,
            0xC6 => ConsumerUsage::SearchMarkForward,
            0xC7 => ConsumerUsage::SearchMarkBackwards,
            0xC8 => ConsumerUsage::CounterReset,
            0xC9 => ConsumerUsage::ShowCounter,
            0xCA => ConsumerUsage::TrackingIncrement,
            0xCB => ConsumerUsage::TrackingDecrement,
            0xCC => ConsumerUsage::StopEject,
            0xCD => ConsumerUsage::PlayPause,
            0xCE => ConsumerUsage::PlaySkip,
            0xCF => ConsumerUsage::VoiceCommand,
            0xD0 => ConsumerUsage::InvokeCaptureInterface,
            0xD1 => ConsumerUsage::StartOrStopGameRecording,
            0xD2 => ConsumerUsage::HistoricalGameCapture,
            0xD3 => ConsumerUsage::CaptureGameScreenshot,
            0xD4 => ConsumerUsage::ShowOrHideRecordingIndicator,
            0xD5 => ConsumerUsage::StartOrStopMicrophoneCapture,
            0xD6 => ConsumerUsage::StartOrStopCameraCapture,
            0xD7 => ConsumerUsage::StartOrStopGameBroadcast,
            0xD8 => ConsumerUsage::StartOrStopVoiceDictationSession,
            0xD9 => ConsumerUsage::InvokeDismissEmojiPicker,
            0xE0 => ConsumerUsage::Volume,
            0xE1 => ConsumerUsage::Balance,
            0xE2 => ConsumerUsage::Mute,
            0xE3 => ConsumerUsage::Bass,
            0xE4 => ConsumerUsage::Treble,
            0xE5 => ConsumerUsage::BassBoost,
            0xE6 => ConsumerUsage::SurroundMode,
            0xE7 => ConsumerUsage::Loudness,
            0xE8 => ConsumerUsage::MPX,
            0xE9 => ConsumerUsage::VolumeIncrement,
            0xEA => ConsumerUsage::VolumeDecrement,
            0xF0 => ConsumerUsage::SpeedSelect,
            0xF1 => ConsumerUsage::PlaybackSpeed,
            0xF2 => ConsumerUsage::StandardPlay,
            0xF3 => ConsumerUsage::LongPlay,
            0xF4 => ConsumerUsage::ExtendedPlay,
            0xF5 => ConsumerUsage::Slow,
            0x100 => ConsumerUsage::FanEnable,
            0x101 => ConsumerUsage::FanSpeed,
            0x102 => ConsumerUsage::LightEnable,
            0x103 => ConsumerUsage::LightIlluminationLevel,
            0x104 => ConsumerUsage::ClimateControlEnable,
            0x105 => ConsumerUsage::RoomTemperature,
            0x106 => ConsumerUsage::SecurityEnable,
            0x107 => ConsumerUsage::FireAlarm,
            0x108 => ConsumerUsage::PoliceAlarm,
            0x109 => ConsumerUsage::Proximity,
            0x10A => ConsumerUsage::Motion,
            0x10B => ConsumerUsage::DuressAlarm,
            0x10C => ConsumerUsage::HoldupAlarm,
            0x10D => ConsumerUsage::MedicalAlarm,
            0x150 => ConsumerUsage::BalanceRight,
            0x151 => ConsumerUsage::BalanceLeft,
            0x152 => ConsumerUsage::BassIncrement,
            0x153 => ConsumerUsage::BassDecrement,
            0x154 => ConsumerUsage::TrebleIncrement,
            0x155 => ConsumerUsage::TrebleDecrement,
            0x160 => ConsumerUsage::SpeakerSystem,
            0x161 => ConsumerUsage::ChannelLeft,
            0x162 => ConsumerUsage::ChannelRight,
            0x163 => ConsumerUsage::ChannelCenter,
            0x164 => ConsumerUsage::ChannelFront,
            0x165 => ConsumerUsage::ChannelCenterFront,
            0x166 => ConsumerUsage::ChannelSide,
            0x167 => ConsumerUsage::ChannelSurround,
            0x168 => ConsumerUsage::ChannelLowFrequencyEnhancement,
            0x169 => ConsumerUsage::ChannelTop,
            0x16A => ConsumerUsage::ChannelUnknown,
            0x170 => ConsumerUsage::SubChannel,
            0x171 => ConsumerUsage::SubChannelIncrement,
            0x172 => ConsumerUsage::SubChannelDecrement,
            0x173 => ConsumerUsage::AlternateAudioIncrement,
            0x174 => ConsumerUsage::AlternateAudioDecrement,
            0x180 => ConsumerUsage::ApplicationLaunchButtons,
            0x181 => ConsumerUsage::ALLaunchButtonConfigurationTool,
            0x182 => ConsumerUsage::ALProgrammableButtonConfiguration,
            0x183 => ConsumerUsage::ALConsumerControlConfiguration,
            0x184 => ConsumerUsage::ALWordProcessor,
            0x185 => ConsumerUsage::ALTextEditor,
            0x186 => ConsumerUsage::ALSpreadsheet,
            0x187 => ConsumerUsage::ALGraphicsEditor,
            0x188 => ConsumerUsage::ALPresentationApp,
            0x189 => ConsumerUsage::ALDatabaseApp,
            0x18A => ConsumerUsage::ALEmailReader,
            0x18B => ConsumerUsage::ALNewsreader,
            0x18C => ConsumerUsage::ALVoicemail,
            0x18D => ConsumerUsage::ALContactsAddressBook,
            0x18E => ConsumerUsage::ALCalendarSchedule,
            0x18F => ConsumerUsage::ALTaskProjectManager,
            0x190 => ConsumerUsage::ALLogJournalTimecard,
            0x191 => ConsumerUsage::ALCheckbookFinance,
            0x192 => ConsumerUsage::ALCalculator,
            0x193 => ConsumerUsage::ALAVCapturePlayback,
            0x194 => ConsumerUsage::ALLocalMachineBrowser,
            0x195 => ConsumerUsage::ALLANWANBrowser,
            0x196 => ConsumerUsage::ALInternetBrowser,
            0x197 => ConsumerUsage::ALRemoteNetworkingISPConnect,
            0x198 => ConsumerUsage::ALNetworkConference,
            0x199 => ConsumerUsage::ALNetworkChat,
            0x19A => ConsumerUsage::ALTelephonyDialer,
            0x19B => ConsumerUsage::ALLogon,
            0x19C => ConsumerUsage::ALLogoff,
            0x19D => ConsumerUsage::ALLogonLogoff,
            0x19E => ConsumerUsage::ALTerminalLockScreensaver,
            0x19F => ConsumerUsage::ALControlPanel,
            0x1A0 => ConsumerUsage::ALCommandLineProcessorRun,
            0x1A1 => ConsumerUsage::ALProcessTaskManager,
            0x1A2 => ConsumerUsage::ALSelectTaskApplication,
            0x1A3 => ConsumerUsage::ALNextTaskApplication,
            0x1A4 => ConsumerUsage::ALPreviousTaskApplication,
            0x1A5 => ConsumerUsage::ALPreemptiveHaltTaskApplication,
            0x1A6 => ConsumerUsage::ALIntegratedHelpCenter,
            0x1A7 => ConsumerUsage::ALDocuments,
            0x1A8 => ConsumerUsage::ALThesaurus,
            0x1A9 => ConsumerUsage::ALDictionary,
            0x1AA => ConsumerUsage::ALDesktop,
            0x1AB => ConsumerUsage::ALSpellCheck,
            0x1AC => ConsumerUsage::ALGrammarCheck,
            0x1AD => ConsumerUsage::ALWirelessStatus,
            0x1AE => ConsumerUsage::ALKeyboardLayout,
            0x1AF => ConsumerUsage::ALVirusProtection,
            0x1B0 => ConsumerUsage::ALEncryption,
            0x1B1 => ConsumerUsage::ALScreenSaver,
            0x1B2 => ConsumerUsage::ALAlarms,
            0x1B3 => ConsumerUsage::ALClock,
            0x1B4 => ConsumerUsage::ALFileBrowser,
            0x1B5 => ConsumerUsage::ALPowerStatus,
            0x1B6 => ConsumerUsage::ALImageBrowser,
            0x1B7 => ConsumerUsage::ALAudioBrowser,
            0x1B8 => ConsumerUsage::ALMovieBrowser,
            0x1B9 => ConsumerUsage::ALDigitalRightsManager,
            0x1BA => ConsumerUsage::ALDigitalWallet,
            0x1BC => ConsumerUsage::ALInstantMessaging,
            0x1BD => ConsumerUsage::ALOEMFeaturesTipsTutorialBrowser,
            0x1BE => ConsumerUsage::ALOEMHelp,
            0x1BF => ConsumerUsage::ALOnlineCommunity,
            0x1C0 => ConsumerUsage::ALEntertainmentContentBrowser,
            0x1C1 => ConsumerUsage::ALOnlineShoppingBrowser,
            0x1C2 => ConsumerUsage::ALSmartCardInformationHelp,
            0x1C3 => ConsumerUsage::ALMarketMonitorFinanceBrowser,
            0x1C4 => ConsumerUsage::ALCustomizedCorporateNewsBrowser,
            0x1C5 => ConsumerUsage::ALOnlineActivityBrowser,
            0x1C6 => ConsumerUsage::ALResearchSearchBrowser,
            0x1C7 => ConsumerUsage::ALAudioPlayer,
            0x1C8 => ConsumerUsage::ALMessageStatus,
            0x1C9 => ConsumerUsage::ALContactSync,
            0x1CA => ConsumerUsage::ALNavigation,
            0x1CB => ConsumerUsage::ALContextawareDesktopAssistant,
            0x200 => ConsumerUsage::GenericGUIApplicationControls,
            0x201 => ConsumerUsage::ACNew,
            0x202 => ConsumerUsage::ACOpen,
            0x203 => ConsumerUsage::ACClose,
            0x204 => ConsumerUsage::ACExit,
            0x205 => ConsumerUsage::ACMaximize,
            0x206 => ConsumerUsage::ACMinimize,
            0x207 => ConsumerUsage::ACSave,
            0x208 => ConsumerUsage::ACPrint,
            0x209 => ConsumerUsage::ACProperties,
            0x21A => ConsumerUsage::ACUndo,
            0x21B => ConsumerUsage::ACCopy,
            0x21C => ConsumerUsage::ACCut,
            0x21D => ConsumerUsage::ACPaste,
            0x21E => ConsumerUsage::ACSelectAll,
            0x21F => ConsumerUsage::ACFind,
            0x220 => ConsumerUsage::ACFindAndReplace,
            0x221 => ConsumerUsage::ACSearch,
            0x222 => ConsumerUsage::ACGoTo,
            0x223 => ConsumerUsage::ACHome,
            0x224 => ConsumerUsage::ACBack,
            0x225 => ConsumerUsage::ACForward,
            0x226 => ConsumerUsage::ACStop,
            0x227 => ConsumerUsage::ACRefresh,
            0x228 => ConsumerUsage::ACPreviousLink,
            0x229 => ConsumerUsage::ACNextLink,
            0x22A => ConsumerUsage::ACBookmarks,
            0x22B => ConsumerUsage::ACHistory,
            0x22C => ConsumerUsage::ACSubscriptions,
            0x22D => ConsumerUsage::ACZoomIn,
            0x22E => ConsumerUsage::ACZoomOut,
            0x22F => ConsumerUsage::ACZoom,
            0x230 => ConsumerUsage::ACFullScreenView,
            0x231 => ConsumerUsage::ACNormalView,
            0x232 => ConsumerUsage::ACViewToggle,
            0x233 => ConsumerUsage::ACScrollUp,
            0x234 => ConsumerUsage::ACScrollDown,
            0x235 => ConsumerUsage::ACScroll,
            0x236 => ConsumerUsage::ACPanLeft,
            0x237 => ConsumerUsage::ACPanRight,
            0x238 => ConsumerUsage::ACPan,
            0x239 => ConsumerUsage::ACNewWindow,
            0x23A => ConsumerUsage::ACTileHorizontally,
            0x23B => ConsumerUsage::ACTileVertically,
            0x23C => ConsumerUsage::ACFormat,
            0x23D => ConsumerUsage::ACEdit,
            0x23E => ConsumerUsage::ACBold,
            0x23F => ConsumerUsage::ACItalics,
            0x240 => ConsumerUsage::ACUnderline,
            0x241 => ConsumerUsage::ACStrikethrough,
            0x242 => ConsumerUsage::ACSubscript,
            0x243 => ConsumerUsage::ACSuperscript,
            0x244 => ConsumerUsage::ACAllCaps,
            0x245 => ConsumerUsage::ACRotate,
            0x246 => ConsumerUsage::ACResize,
            0x247 => ConsumerUsage::ACFlipHorizontal,
            0x248 => ConsumerUsage::ACFlipVertical,
            0x249 => ConsumerUsage::ACMirrorHorizontal,
            0x24A => ConsumerUsage::ACMirrorVertical,
            0x24B => ConsumerUsage::ACFontSelect,
            0x24C => ConsumerUsage::ACFontColor,
            0x24D => ConsumerUsage::ACFontSize,
            0x24E => ConsumerUsage::ACJustifyLeft,
            0x24F => ConsumerUsage::ACJustifyCenterH,
            0x250 => ConsumerUsage::ACJustifyRight,
            0x251 => ConsumerUsage::ACJustifyBlockH,
            0x252 => ConsumerUsage::ACJustifyTop,
            0x253 => ConsumerUsage::ACJustifyCenterV,
            0x254 => ConsumerUsage::ACJustifyBottom,
            0x255 => ConsumerUsage::ACJustifyBlockV,
            0x256 => ConsumerUsage::ACIndentDecrease,
            0x257 => ConsumerUsage::ACIndentIncrease,
            0x258 => ConsumerUsage::ACNumberedList,
            0x259 => ConsumerUsage::ACRestartNumbering,
            0x25A => ConsumerUsage::ACBulletedList,
            0x25B => ConsumerUsage::ACPromote,
            0x25C => ConsumerUsage::ACDemote,
            0x25D => ConsumerUsage::ACYes,
            0x25E => ConsumerUsage::ACNo,
            0x25F => ConsumerUsage::ACCancel,
            0x260 => ConsumerUsage::ACCatalog,
            0x261 => ConsumerUsage::ACBuyCheckout,
            0x262 => ConsumerUsage::ACAddToCart,
            0x263 => ConsumerUsage::ACExpand,
            0x264 => ConsumerUsage::ACExpandAll,
            0x265 => ConsumerUsage::ACCollapse,
            0x266 => ConsumerUsage::ACCollapseAll,
            0x267 => ConsumerUsage::ACPrintPreview,
            0x268 => ConsumerUsage::ACPasteSpecial,
            0x269 => ConsumerUsage::ACInsertMode,
            0x26A => ConsumerUsage::ACDelete,
            0x26B => ConsumerUsage::ACLock,
            0x26C => ConsumerUsage::ACUnlock,
            0x26D => ConsumerUsage::ACProtect,
            0x26E => ConsumerUsage::ACUnprotect,
            0x26F => ConsumerUsage::ACAttachComment,
            0x270 => ConsumerUsage::ACDeleteComment,
            0x271 => ConsumerUsage::ACViewComment,
            0x272 => ConsumerUsage::ACSelectWord,
            0x273 => ConsumerUsage::ACSelectSentence,
            0x274 => ConsumerUsage::ACSelectParagraph,
            0x275 => ConsumerUsage::ACSelectColumn,
            0x276 => ConsumerUsage::ACSelectRow,
            0x277 => ConsumerUsage::ACSelectTable,
            0x278 => ConsumerUsage::ACSelectObject,
            0x279 => ConsumerUsage::ACRedoRepeat,
            0x27A => ConsumerUsage::ACSort,
            0x27B => ConsumerUsage::ACSortAscending,
            0x27C => ConsumerUsage::ACSortDescending,
            0x27D => ConsumerUsage::ACFilter,
            0x27E => ConsumerUsage::ACSetClock,
            0x27F => ConsumerUsage::ACViewClock,
            0x280 => ConsumerUsage::ACSelectTimeZone,
            0x281 => ConsumerUsage::ACEditTimeZones,
            0x282 => ConsumerUsage::ACSetAlarm,
            0x283 => ConsumerUsage::ACClearAlarm,
            0x284 => ConsumerUsage::ACSnoozeAlarm,
            0x285 => ConsumerUsage::ACResetAlarm,
            0x286 => ConsumerUsage::ACSynchronize,
            0x287 => ConsumerUsage::ACSendReceive,
            0x288 => ConsumerUsage::ACSendTo,
            0x289 => ConsumerUsage::ACReply,
            0x28A => ConsumerUsage::ACReplyAll,
            0x28B => ConsumerUsage::ACForwardMsg,
            0x28C => ConsumerUsage::ACSend,
            0x28D => ConsumerUsage::ACAttachFile,
            0x28E => ConsumerUsage::ACUpload,
            0x28F => ConsumerUsage::ACDownloadSaveTargetAs,
            0x290 => ConsumerUsage::ACSetBorders,
            0x291 => ConsumerUsage::ACInsertRow,
            0x292 => ConsumerUsage::ACInsertColumn,
            0x293 => ConsumerUsage::ACInsertFile,
            0x294 => ConsumerUsage::ACInsertPicture,
            0x295 => ConsumerUsage::ACInsertObject,
            0x296 => ConsumerUsage::ACInsertSymbol,
            0x297 => ConsumerUsage::ACSaveAndClose,
            0x298 => ConsumerUsage::ACRename,
            0x299 => ConsumerUsage::ACMerge,
            0x29A => ConsumerUsage::ACSplit,
            0x29B => ConsumerUsage::ACDistributeHorizontally,
            0x29C => ConsumerUsage::ACDistributeVertically,
            0x29D => ConsumerUsage::ACNextKeyboardLayoutSelect,
            0x29E => ConsumerUsage::ACNavigationGuidance,
            0x29F => ConsumerUsage::ACDesktopShowAllWindows,
            0x2A0 => ConsumerUsage::ACSoftKeyLeft,
            0x2A1 => ConsumerUsage::ACSoftKeyRight,
            0x2A2 => ConsumerUsage::ACDesktopShowAllApplications,
            0x2B0 => ConsumerUsage::ACIdleKeepAlive,
            0x2C0 => ConsumerUsage::ExtendedKeyboardAttributesCollection,
            0x2C1 => ConsumerUsage::KeyboardFormFactor,
            0x2C2 => ConsumerUsage::KeyboardKeyType,
            0x2C3 => ConsumerUsage::KeyboardPhysicalLayout,
            0x2C4 => ConsumerUsage::VendorSpecificKeyboardPhysicalLayout,
            0x2C5 => ConsumerUsage::KeyboardIETFLanguageTagIndex,
            0x2C6 => ConsumerUsage::ImplementedKeyboardInputAssistControls,
            0x2C7 => ConsumerUsage::KeyboardInputAssistPrevious,
            0x2C8 => ConsumerUsage::KeyboardInputAssistNext,
            0x2C9 => ConsumerUsage::KeyboardInputAssistPreviousGroup,
            0x2CA => ConsumerUsage::KeyboardInputAssistNextGroup,
            0x2CB => ConsumerUsage::KeyboardInputAssistAccept,
            0x2CC => ConsumerUsage::KeyboardInputAssistCancel,
            0x2D0 => ConsumerUsage::PrivacyScreenToggle,
            0x2D1 => ConsumerUsage::PrivacyScreenLevelDecrement,
            0x2D2 => ConsumerUsage::PrivacyScreenLevelIncrement,
            0x2D3 => ConsumerUsage::PrivacyScreenLevelMinimum,
            0x2D4 => ConsumerUsage::PrivacyScreenLevelMaximum,
            0x500 => ConsumerUsage::ContactEdited,
            0x501 => ConsumerUsage::ContactAdded,
            0x502 => ConsumerUsage::ContactRecordActive,
            0x503 => ConsumerUsage::ContactIndex,
            0x504 => ConsumerUsage::ContactNickname,
            0x505 => ConsumerUsage::ContactFirstName,
            0x506 => ConsumerUsage::ContactLastName,
            0x507 => ConsumerUsage::ContactFullName,
            0x508 => ConsumerUsage::ContactPhoneNumberPersonal,
            0x509 => ConsumerUsage::ContactPhoneNumberBusiness,
            0x50A => ConsumerUsage::ContactPhoneNumberMobile,
            0x50B => ConsumerUsage::ContactPhoneNumberPager,
            0x50C => ConsumerUsage::ContactPhoneNumberFax,
            0x50D => ConsumerUsage::ContactPhoneNumberOther,
            0x50E => ConsumerUsage::ContactEmailPersonal,
            0x50F => ConsumerUsage::ContactEmailBusiness,
            0x510 => ConsumerUsage::ContactEmailOther,
            0x511 => ConsumerUsage::ContactEmailMain,
            0x512 => ConsumerUsage::ContactSpeedDialNumber,
            0x513 => ConsumerUsage::ContactStatusFlag,
            0x514 => ConsumerUsage::ContactMisc,
            i => ConsumerUsage::Reserved(i),
        }
    }
}

impl From<ConsumerUsage> for u16 {
    fn from(value: ConsumerUsage) -> Self {
        match value {
            ConsumerUsage::Undefined => 0x00,
            ConsumerUsage::ConsumerControl => 0x01,
            ConsumerUsage::NumericKeyPad => 0x02,
            ConsumerUsage::ProgrammableButtons => 0x03,
            ConsumerUsage::Microphone => 0x04,
            ConsumerUsage::Headphone => 0x05,
            ConsumerUsage::GraphicEqualizer => 0x06,
            ConsumerUsage::Plus10 => 0x20,
            ConsumerUsage::Plus100 => 0x21,
            ConsumerUsage::AmPm => 0x22,
            ConsumerUsage::Power => 0x30,
            ConsumerUsage::Reset => 0x31,
            ConsumerUsage::Sleep => 0x32,
            ConsumerUsage::SleepAfter => 0x33,
            ConsumerUsage::SleepMode => 0x34,
            ConsumerUsage::Illumination => 0x35,
            ConsumerUsage::FunctionButtons => 0x36,
            ConsumerUsage::Menu => 0x40,
            ConsumerUsage::MenuPick => 0x41,
            ConsumerUsage::MenuUp => 0x42,
            ConsumerUsage::MenuDown => 0x43,
            ConsumerUsage::MenuLeft => 0x44,
            ConsumerUsage::MenuRight => 0x45,
            ConsumerUsage::MenuEscape => 0x46,
            ConsumerUsage::MenuValueIncrease => 0x47,
            ConsumerUsage::MenuValueDecrease => 0x48,
            ConsumerUsage::DataOnScreen => 0x60,
            ConsumerUsage::ClosedCaption => 0x61,
            ConsumerUsage::ClosedCaptionSelect => 0x62,
            ConsumerUsage::VcrTv => 0x63,
            ConsumerUsage::BroadcastMode => 0x64,
            ConsumerUsage::Snapshot => 0x65,
            ConsumerUsage::Still => 0x66,
            ConsumerUsage::PictureInPictureToggle => 0x67,
            ConsumerUsage::PictureInPictureSwap => 0x68,
            ConsumerUsage::RedMenuButton => 0x69,
            ConsumerUsage::GreenMenuButton => 0x6A,
            ConsumerUsage::BlueMenuButton => 0x6B,
            ConsumerUsage::YellowMenuButton => 0x6C,
            ConsumerUsage::Aspect => 0x6D,
            ConsumerUsage::Mode3DSelect => 0x6E,
            ConsumerUsage::DisplayBrightnessIncrement => 0x6F,
            ConsumerUsage::DisplayBrightnessDecrement => 0x70,
            ConsumerUsage::DisplayBrightness => 0x71,
            ConsumerUsage::DisplayBacklightToggle => 0x72,
            ConsumerUsage::DisplaySetBrightnessToMinimum => 0x73,
            ConsumerUsage::DisplaySetBrightnessToMaximum => 0x74,
            ConsumerUsage::DisplaySetAutoBrightness => 0x75,
            ConsumerUsage::CameraAccessEnabled => 0x76,
            ConsumerUsage::CameraAccessDisabled => 0x77,
            ConsumerUsage::CameraAccessToggle => 0x78,
            ConsumerUsage::KeyboardBrightnessIncrement => 0x79,
            ConsumerUsage::KeyboardBrightnessDecrement => 0x7A,
            ConsumerUsage::KeyboardBacklightSetLevel => 0x7B,
            ConsumerUsage::KeyboardBacklightOOC => 0x7C,
            ConsumerUsage::KeyboardBacklightSetMinimum => 0x7D,
            ConsumerUsage::KeyboardBacklightSetMaximum => 0x7E,
            ConsumerUsage::KeyboardBacklightAuto => 0x7F,
            ConsumerUsage::Selection => 0x80,
            ConsumerUsage::AssignSelection => 0x81,
            ConsumerUsage::ModeStep => 0x82,
            ConsumerUsage::RecallLast => 0x83,
            ConsumerUsage::EnterChannel => 0x84,
            ConsumerUsage::OrderMovie => 0x85,
            ConsumerUsage::Channel => 0x86,
            ConsumerUsage::MediaSelection => 0x87,
            ConsumerUsage::MediaSelectComputer => 0x88,
            ConsumerUsage::MediaSelectTV => 0x89,
            ConsumerUsage::MediaSelectWWW => 0x8A,
            ConsumerUsage::MediaSelectDVD => 0x8B,
            ConsumerUsage::MediaSelectTelephone => 0x8C,
            ConsumerUsage::MediaSelectProgramGuide => 0x8D,
            ConsumerUsage::MediaSelectVideoPhone => 0x8E,
            ConsumerUsage::MediaSelectGames => 0x8F,
            ConsumerUsage::MediaSelectMessages => 0x90,
            ConsumerUsage::MediaSelectCD => 0x91,
            ConsumerUsage::MediaSelectVCR => 0x92,
            ConsumerUsage::MediaSelectTuner => 0x93,
            ConsumerUsage::Quit => 0x94,
            ConsumerUsage::Help => 0x95,
            ConsumerUsage::MediaSelectTape => 0x96,
            ConsumerUsage::MediaSelectCable => 0x97,
            ConsumerUsage::MediaSelectSatellite => 0x98,
            ConsumerUsage::MediaSelectSecurity => 0x99,
            ConsumerUsage::MediaSelectHome => 0x9A,
            ConsumerUsage::MediaSelectCall => 0x9B,
            ConsumerUsage::ChannelIncrement => 0x9C,
            ConsumerUsage::ChannelDecrement => 0x9D,
            ConsumerUsage::MediaSelectSAP => 0x9E,
            ConsumerUsage::VCRPlus => 0xA0,
            ConsumerUsage::Once => 0xA1,
            ConsumerUsage::Daily => 0xA2,
            ConsumerUsage::Weekly => 0xA3,
            ConsumerUsage::Monthly => 0xA4,
            ConsumerUsage::Play => 0xB0,
            ConsumerUsage::Pause => 0xB1,
            ConsumerUsage::Record => 0xB2,
            ConsumerUsage::FastForward => 0xB3,
            ConsumerUsage::Rewind => 0xB4,
            ConsumerUsage::ScanNextTrack => 0xB5,
            ConsumerUsage::ScanPreviousTrack => 0xB6,
            ConsumerUsage::Stop => 0xB7,
            ConsumerUsage::Eject => 0xB8,
            ConsumerUsage::RandomPlay => 0xB9,
            ConsumerUsage::SelectDisc => 0xBA,
            ConsumerUsage::EnterDisc => 0xBB,
            ConsumerUsage::Repeat => 0xBC,
            ConsumerUsage::Tracking => 0xBD,
            ConsumerUsage::TrackNormal => 0xBE,
            ConsumerUsage::SlowTracking => 0xBF,
            ConsumerUsage::FrameForward => 0xC0,
            ConsumerUsage::FrameBack => 0xC1,
            ConsumerUsage::Mark => 0xC2,
            ConsumerUsage::ClearMark => 0xC3,
            ConsumerUsage::RepeatFromMark => 0xC4,
            ConsumerUsage::ReturnToMark => 0xC5,
            ConsumerUsage::SearchMarkForward => 0xC6,
            ConsumerUsage::SearchMarkBackwards => 0xC7,
            ConsumerUsage::CounterReset => 0xC8,
            ConsumerUsage::ShowCounter => 0xC9,
            ConsumerUsage::TrackingIncrement => 0xCA,
            ConsumerUsage::TrackingDecrement => 0xCB,
            ConsumerUsage::StopEject => 0xCC,
            ConsumerUsage::PlayPause => 0xCD,
            ConsumerUsage::PlaySkip => 0xCE,
            ConsumerUsage::VoiceCommand => 0xCF,
            ConsumerUsage::InvokeCaptureInterface => 0xD0,
            ConsumerUsage::StartOrStopGameRecording => 0xD1,
            ConsumerUsage::HistoricalGameCapture => 0xD2,
            ConsumerUsage::CaptureGameScreenshot => 0xD3,
            ConsumerUsage::ShowOrHideRecordingIndicator => 0xD4,
            ConsumerUsage::StartOrStopMicrophoneCapture => 0xD5,
            ConsumerUsage::StartOrStopCameraCapture => 0xD6,
            ConsumerUsage::StartOrStopGameBroadcast => 0xD7,
            ConsumerUsage::StartOrStopVoiceDictationSession => 0xD8,
            ConsumerUsage::InvokeDismissEmojiPicker => 0xD9,
            ConsumerUsage::Volume => 0xE0,
            ConsumerUsage::Balance => 0xE1,
            ConsumerUsage::Mute => 0xE2,
            ConsumerUsage::Bass => 0xE3,
            ConsumerUsage::Treble => 0xE4,
            ConsumerUsage::BassBoost => 0xE5,
            ConsumerUsage::SurroundMode => 0xE6,
            ConsumerUsage::Loudness => 0xE7,
            ConsumerUsage::MPX => 0xE8,
            ConsumerUsage::VolumeIncrement => 0xE9,
            ConsumerUsage::VolumeDecrement => 0xEA,
            ConsumerUsage::SpeedSelect => 0xF0,
            ConsumerUsage::PlaybackSpeed => 0xF1,
            ConsumerUsage::StandardPlay => 0xF2,
            ConsumerUsage::LongPlay => 0xF3,
            ConsumerUsage::ExtendedPlay => 0xF4,
            ConsumerUsage::Slow => 0xF5,
            ConsumerUsage::FanEnable => 0x100,
            ConsumerUsage::FanSpeed => 0x101,
            ConsumerUsage::LightEnable => 0x102,
            ConsumerUsage::LightIlluminationLevel => 0x103,
            ConsumerUsage::ClimateControlEnable => 0x104,
            ConsumerUsage::RoomTemperature => 0x105,
            ConsumerUsage::SecurityEnable => 0x106,
            ConsumerUsage::FireAlarm => 0x107,
            ConsumerUsage::PoliceAlarm => 0x108,
            ConsumerUsage::Proximity => 0x109,
            ConsumerUsage::Motion => 0x10A,
            ConsumerUsage::DuressAlarm => 0x10B,
            ConsumerUsage::HoldupAlarm => 0x10C,
            ConsumerUsage::MedicalAlarm => 0x10D,
            ConsumerUsage::BalanceRight => 0x150,
            ConsumerUsage::BalanceLeft => 0x151,
            ConsumerUsage::BassIncrement => 0x152,
            ConsumerUsage::BassDecrement => 0x153,
            ConsumerUsage::TrebleIncrement => 0x154,
            ConsumerUsage::TrebleDecrement => 0x155,
            ConsumerUsage::SpeakerSystem => 0x160,
            ConsumerUsage::ChannelLeft => 0x161,
            ConsumerUsage::ChannelRight => 0x162,
            ConsumerUsage::ChannelCenter => 0x163,
            ConsumerUsage::ChannelFront => 0x164,
            ConsumerUsage::ChannelCenterFront => 0x165,
            ConsumerUsage::ChannelSide => 0x166,
            ConsumerUsage::ChannelSurround => 0x167,
            ConsumerUsage::ChannelLowFrequencyEnhancement => 0x168,
            ConsumerUsage::ChannelTop => 0x169,
            ConsumerUsage::ChannelUnknown => 0x16A,
            ConsumerUsage::SubChannel => 0x170,
            ConsumerUsage::SubChannelIncrement => 0x171,
            ConsumerUsage::SubChannelDecrement => 0x172,
            ConsumerUsage::AlternateAudioIncrement => 0x173,
            ConsumerUsage::AlternateAudioDecrement => 0x174,
            ConsumerUsage::ApplicationLaunchButtons => 0x180,
            ConsumerUsage::ALLaunchButtonConfigurationTool => 0x181,
            ConsumerUsage::ALProgrammableButtonConfiguration => 0x182,
            ConsumerUsage::ALConsumerControlConfiguration => 0x183,
            ConsumerUsage::ALWordProcessor => 0x184,
            ConsumerUsage::ALTextEditor => 0x185,
            ConsumerUsage::ALSpreadsheet => 0x186,
            ConsumerUsage::ALGraphicsEditor => 0x187,
            ConsumerUsage::ALPresentationApp => 0x188,
            ConsumerUsage::ALDatabaseApp => 0x189,
            ConsumerUsage::ALEmailReader => 0x18A,
            ConsumerUsage::ALNewsreader => 0x18B,
            ConsumerUsage::ALVoicemail => 0x18C,
            ConsumerUsage::ALContactsAddressBook => 0x18D,
            ConsumerUsage::ALCalendarSchedule => 0x18E,
            ConsumerUsage::ALTaskProjectManager => 0x18F,
            ConsumerUsage::ALLogJournalTimecard => 0x190,
            ConsumerUsage::ALCheckbookFinance => 0x191,
            ConsumerUsage::ALCalculator => 0x192,
            ConsumerUsage::ALAVCapturePlayback => 0x193,
            ConsumerUsage::ALLocalMachineBrowser => 0x194,
            ConsumerUsage::ALLANWANBrowser => 0x195,
            ConsumerUsage::ALInternetBrowser => 0x196,
            ConsumerUsage::ALRemoteNetworkingISPConnect => 0x197,
            ConsumerUsage::ALNetworkConference => 0x198,
            ConsumerUsage::ALNetworkChat => 0x199,
            ConsumerUsage::ALTelephonyDialer => 0x19A,
            ConsumerUsage::ALLogon => 0x19B,
            ConsumerUsage::ALLogoff => 0x19C,
            ConsumerUsage::ALLogonLogoff => 0x19D,
            ConsumerUsage::ALTerminalLockScreensaver => 0x19E,
            ConsumerUsage::ALControlPanel => 0x19F,
            ConsumerUsage::ALCommandLineProcessorRun => 0x1A0,
            ConsumerUsage::ALProcessTaskManager => 0x1A1,
            ConsumerUsage::ALSelectTaskApplication => 0x1A2,
            ConsumerUsage::ALNextTaskApplication => 0x1A3,
            ConsumerUsage::ALPreviousTaskApplication => 0x1A4,
            ConsumerUsage::ALPreemptiveHaltTaskApplication => 0x1A5,
            ConsumerUsage::ALIntegratedHelpCenter => 0x1A6,
            ConsumerUsage::ALDocuments => 0x1A7,
            ConsumerUsage::ALThesaurus => 0x1A8,
            ConsumerUsage::ALDictionary => 0x1A9,
            ConsumerUsage::ALDesktop => 0x1AA,
            ConsumerUsage::ALSpellCheck => 0x1AB,
            ConsumerUsage::ALGrammarCheck => 0x1AC,
            ConsumerUsage::ALWirelessStatus => 0x1AD,
            ConsumerUsage::ALKeyboardLayout => 0x1AE,
            ConsumerUsage::ALVirusProtection => 0x1AF,
            ConsumerUsage::ALEncryption => 0x1B0,
            ConsumerUsage::ALScreenSaver => 0x1B1,
            ConsumerUsage::ALAlarms => 0x1B2,
            ConsumerUsage::ALClock => 0x1B3,
            ConsumerUsage::ALFileBrowser => 0x1B4,
            ConsumerUsage::ALPowerStatus => 0x1B5,
            ConsumerUsage::ALImageBrowser => 0x1B6,
            ConsumerUsage::ALAudioBrowser => 0x1B7,
            ConsumerUsage::ALMovieBrowser => 0x1B8,
            ConsumerUsage::ALDigitalRightsManager => 0x1B9,
            ConsumerUsage::ALDigitalWallet => 0x1BA,
            ConsumerUsage::ALInstantMessaging => 0x1BC,
            ConsumerUsage::ALOEMFeaturesTipsTutorialBrowser => 0x1BD,
            ConsumerUsage::ALOEMHelp => 0x1BE,
            ConsumerUsage::ALOnlineCommunity => 0x1BF,
            ConsumerUsage::ALEntertainmentContentBrowser => 0x1C0,
            ConsumerUsage::ALOnlineShoppingBrowser => 0x1C1,
            ConsumerUsage::ALSmartCardInformationHelp => 0x1C2,
            ConsumerUsage::ALMarketMonitorFinanceBrowser => 0x1C3,
            ConsumerUsage::ALCustomizedCorporateNewsBrowser => 0x1C4,
            ConsumerUsage::ALOnlineActivityBrowser => 0x1C5,
            ConsumerUsage::ALResearchSearchBrowser => 0x1C6,
            ConsumerUsage::ALAudioPlayer => 0x1C7,
            ConsumerUsage::ALMessageStatus => 0x1C8,
            ConsumerUsage::ALContactSync => 0x1C9,
            ConsumerUsage::ALNavigation => 0x1CA,
            ConsumerUsage::ALContextawareDesktopAssistant => 0x1CB,
            ConsumerUsage::GenericGUIApplicationControls => 0x200,
            ConsumerUsage::ACNew => 0x201,
            ConsumerUsage::ACOpen => 0x202,
            ConsumerUsage::ACClose => 0x203,
            ConsumerUsage::ACExit => 0x204,
            ConsumerUsage::ACMaximize => 0x205,
            ConsumerUsage::ACMinimize => 0x206,
            ConsumerUsage::ACSave => 0x207,
            ConsumerUsage::ACPrint => 0x208,
            ConsumerUsage::ACProperties => 0x209,
            ConsumerUsage::ACUndo => 0x21A,
            ConsumerUsage::ACCopy => 0x21B,
            ConsumerUsage::ACCut => 0x21C,
            ConsumerUsage::ACPaste => 0x21D,
            ConsumerUsage::ACSelectAll => 0x21E,
            ConsumerUsage::ACFind => 0x21F,
            ConsumerUsage::ACFindAndReplace => 0x220,
            ConsumerUsage::ACSearch => 0x221,
            ConsumerUsage::ACGoTo => 0x222,
            ConsumerUsage::ACHome => 0x223,
            ConsumerUsage::ACBack => 0x224,
            ConsumerUsage::ACForward => 0x225,
            ConsumerUsage::ACStop => 0x226,
            ConsumerUsage::ACRefresh => 0x227,
            ConsumerUsage::ACPreviousLink => 0x228,
            ConsumerUsage::ACNextLink => 0x229,
            ConsumerUsage::ACBookmarks => 0x22A,
            ConsumerUsage::ACHistory => 0x22B,
            ConsumerUsage::ACSubscriptions => 0x22C,
            ConsumerUsage::ACZoomIn => 0x22D,
            ConsumerUsage::ACZoomOut => 0x22E,
            ConsumerUsage::ACZoom => 0x22F,
            ConsumerUsage::ACFullScreenView => 0x230,
            ConsumerUsage::ACNormalView => 0x231,
            ConsumerUsage::ACViewToggle => 0x232,
            ConsumerUsage::ACScrollUp => 0x233,
            ConsumerUsage::ACScrollDown => 0x234,
            ConsumerUsage::ACScroll => 0x235,
            ConsumerUsage::ACPanLeft => 0x236,
            ConsumerUsage::ACPanRight => 0x237,
            ConsumerUsage::ACPan => 0x238,
            ConsumerUsage::ACNewWindow => 0x239,
            ConsumerUsage::ACTileHorizontally => 0x23A,
            ConsumerUsage::ACTileVertically => 0x23B,
            ConsumerUsage::ACFormat => 0x23C,
            ConsumerUsage::ACEdit => 0x23D,
            ConsumerUsage::ACBold => 0x23E,
            ConsumerUsage::ACItalics => 0x23F,
            ConsumerUsage::ACUnderline => 0x240,
            ConsumerUsage::ACStrikethrough => 0x241,
            ConsumerUsage::ACSubscript => 0x242,
            ConsumerUsage::ACSuperscript => 0x243,
            ConsumerUsage::ACAllCaps => 0x244,
            ConsumerUsage::ACRotate => 0x245,
            ConsumerUsage::ACResize => 0x246,
            ConsumerUsage::ACFlipHorizontal => 0x247,
            ConsumerUsage::ACFlipVertical => 0x248,
            ConsumerUsage::ACMirrorHorizontal => 0x249,
            ConsumerUsage::ACMirrorVertical => 0x24A,
            ConsumerUsage::ACFontSelect => 0x24B,
            ConsumerUsage::ACFontColor => 0x24C,
            ConsumerUsage::ACFontSize => 0x24D,
            ConsumerUsage::ACJustifyLeft => 0x24E,
            ConsumerUsage::ACJustifyCenterH => 0x24F,
            ConsumerUsage::ACJustifyRight => 0x250,
            ConsumerUsage::ACJustifyBlockH => 0x251,
            ConsumerUsage::ACJustifyTop => 0x252,
            ConsumerUsage::ACJustifyCenterV => 0x253,
            ConsumerUsage::ACJustifyBottom => 0x254,
            ConsumerUsage::ACJustifyBlockV => 0x255,
            ConsumerUsage::ACIndentDecrease => 0x256,
            ConsumerUsage::ACIndentIncrease => 0x257,
            ConsumerUsage::ACNumberedList => 0x258,
            ConsumerUsage::ACRestartNumbering => 0x259,
            ConsumerUsage::ACBulletedList => 0x25A,
            ConsumerUsage::ACPromote => 0x25B,
            ConsumerUsage::ACDemote => 0x25C,
            ConsumerUsage::ACYes => 0x25D,
            ConsumerUsage::ACNo => 0x25E,
            ConsumerUsage::ACCancel => 0x25F,
            ConsumerUsage::ACCatalog => 0x260,
            ConsumerUsage::ACBuyCheckout => 0x261,
            ConsumerUsage::ACAddToCart => 0x262,
            ConsumerUsage::ACExpand => 0x263,
            ConsumerUsage::ACExpandAll => 0x264,
            ConsumerUsage::ACCollapse => 0x265,
            ConsumerUsage::ACCollapseAll => 0x266,
            ConsumerUsage::ACPrintPreview => 0x267,
            ConsumerUsage::ACPasteSpecial => 0x268,
            ConsumerUsage::ACInsertMode => 0x269,
            ConsumerUsage::ACDelete => 0x26A,
            ConsumerUsage::ACLock => 0x26B,
            ConsumerUsage::ACUnlock => 0x26C,
            ConsumerUsage::ACProtect => 0x26D,
            ConsumerUsage::ACUnprotect => 0x26E,
            ConsumerUsage::ACAttachComment => 0x26F,
            ConsumerUsage::ACDeleteComment => 0x270,
            ConsumerUsage::ACViewComment => 0x271,
            ConsumerUsage::ACSelectWord => 0x272,
            ConsumerUsage::ACSelectSentence => 0x273,
            ConsumerUsage::ACSelectParagraph => 0x274,
            ConsumerUsage::ACSelectColumn => 0x275,
            ConsumerUsage::ACSelectRow => 0x276,
            ConsumerUsage::ACSelectTable => 0x277,
            ConsumerUsage::ACSelectObject => 0x278,
            ConsumerUsage::ACRedoRepeat => 0x279,
            ConsumerUsage::ACSort => 0x27A,
            ConsumerUsage::ACSortAscending => 0x27B,
            ConsumerUsage::ACSortDescending => 0x27C,
            ConsumerUsage::ACFilter => 0x27D,
            ConsumerUsage::ACSetClock => 0x27E,
            ConsumerUsage::ACViewClock => 0x27F,
            ConsumerUsage::ACSelectTimeZone => 0x280,
            ConsumerUsage::ACEditTimeZones => 0x281,
            ConsumerUsage::ACSetAlarm => 0x282,
            ConsumerUsage::ACClearAlarm => 0x283,
            ConsumerUsage::ACSnoozeAlarm => 0x284,
            ConsumerUsage::ACResetAlarm => 0x285,
            ConsumerUsage::ACSynchronize => 0x286,
            ConsumerUsage::ACSendReceive => 0x287,
            ConsumerUsage::ACSendTo => 0x288,
            ConsumerUsage::ACReply => 0x289,
            ConsumerUsage::ACReplyAll => 0x28A,
            ConsumerUsage::ACForwardMsg => 0x28B,
            ConsumerUsage::ACSend => 0x28C,
            ConsumerUsage::ACAttachFile => 0x28D,
            ConsumerUsage::ACUpload => 0x28E,
            ConsumerUsage::ACDownloadSaveTargetAs => 0x28F,
            ConsumerUsage::ACSetBorders => 0x290,
            ConsumerUsage::ACInsertRow => 0x291,
            ConsumerUsage::ACInsertColumn => 0x292,
            ConsumerUsage::ACInsertFile => 0x293,
            ConsumerUsage::ACInsertPicture => 0x294,
            ConsumerUsage::ACInsertObject => 0x295,
            ConsumerUsage::ACInsertSymbol => 0x296,
            ConsumerUsage::ACSaveAndClose => 0x297,
            ConsumerUsage::ACRename => 0x298,
            ConsumerUsage::ACMerge => 0x299,
            ConsumerUsage::ACSplit => 0x29A,
            ConsumerUsage::ACDistributeHorizontally => 0x29B,
            ConsumerUsage::ACDistributeVertically => 0x29C,
            ConsumerUsage::ACNextKeyboardLayoutSelect => 0x29D,
            ConsumerUsage::ACNavigationGuidance => 0x29E,
            ConsumerUsage::ACDesktopShowAllWindows => 0x29F,
            ConsumerUsage::ACSoftKeyLeft => 0x2A0,
            ConsumerUsage::ACSoftKeyRight => 0x2A1,
            ConsumerUsage::ACDesktopShowAllApplications => 0x2A2,
            ConsumerUsage::ACIdleKeepAlive => 0x2B0,
            ConsumerUsage::ExtendedKeyboardAttributesCollection => 0x2C0,
            ConsumerUsage::KeyboardFormFactor => 0x2C1,
            ConsumerUsage::KeyboardKeyType => 0x2C2,
            ConsumerUsage::KeyboardPhysicalLayout => 0x2C3,
            ConsumerUsage::VendorSpecificKeyboardPhysicalLayout => 0x2C4,
            ConsumerUsage::KeyboardIETFLanguageTagIndex => 0x2C5,
            ConsumerUsage::ImplementedKeyboardInputAssistControls => 0x2C6,
            ConsumerUsage::KeyboardInputAssistPrevious => 0x2C7,
            ConsumerUsage::KeyboardInputAssistNext => 0x2C8,
            ConsumerUsage::KeyboardInputAssistPreviousGroup => 0x2C9,
            ConsumerUsage::KeyboardInputAssistNextGroup => 0x2CA,
            ConsumerUsage::KeyboardInputAssistAccept => 0x2CB,
            ConsumerUsage::KeyboardInputAssistCancel => 0x2CC,
            ConsumerUsage::PrivacyScreenToggle => 0x2D0,
            ConsumerUsage::PrivacyScreenLevelDecrement => 0x2D1,
            ConsumerUsage::PrivacyScreenLevelIncrement => 0x2D2,
            ConsumerUsage::PrivacyScreenLevelMinimum => 0x2D3,
            ConsumerUsage::PrivacyScreenLevelMaximum => 0x2D4,
            ConsumerUsage::ContactEdited => 0x500,
            ConsumerUsage::ContactAdded => 0x501,
            ConsumerUsage::ContactRecordActive => 0x502,
            ConsumerUsage::ContactIndex => 0x503,
            ConsumerUsage::ContactNickname => 0x504,
            ConsumerUsage::ContactFirstName => 0x505,
            ConsumerUsage::ContactLastName => 0x506,
            ConsumerUsage::ContactFullName => 0x507,
            ConsumerUsage::ContactPhoneNumberPersonal => 0x508,
            ConsumerUsage::ContactPhoneNumberBusiness => 0x509,
            ConsumerUsage::ContactPhoneNumberMobile => 0x50A,
            ConsumerUsage::ContactPhoneNumberPager => 0x50B,
            ConsumerUsage::ContactPhoneNumberFax => 0x50C,
            ConsumerUsage::ContactPhoneNumberOther => 0x50D,
            ConsumerUsage::ContactEmailPersonal => 0x50E,
            ConsumerUsage::ContactEmailBusiness => 0x50F,
            ConsumerUsage::ContactEmailOther => 0x510,
            ConsumerUsage::ContactEmailMain => 0x511,
            ConsumerUsage::ContactSpeedDialNumber => 0x512,
            ConsumerUsage::ContactStatusFlag => 0x513,
            ConsumerUsage::ContactMisc => 0x514,
            ConsumerUsage::Reserved(i) => i,
        }
    }
}

impl UsageId for ConsumerUsage {
    fn usage_id(self) -> u16 {
        self.into()
    }
}

//Undefined,
// Consumer Control,
// Numeric Key Pad,
// Programmable Buttons,
// Microphone,
// Headphone,
// Graphic Equalizer,
// +10,
// +100,
// AM/PM,
// Power,
// Reset,
// Sleep,
// Sleep After,
// Sleep Mode,
// Illumination,
// Function Buttons,
// Menu,
// Menu Pick,
// Menu Up,
// Menu Down,
// Menu Left,
// Menu Right,
// Menu Escape,
// Menu Value Increase,
// Menu Value Decrease,
// Data On Screen,
// Closed Caption,
// Closed Caption Select,
// VCR/TV,
// Broadcast Mode,
// Snapshot,
// Still,
// Picture-in-Picture Toggle,
// Picture-in-Picture Swap,
// Red Menu Button,
// Green Menu Button,
// Blue Menu Button,
// Yellow Menu Button,
// Aspect,
// 3D Mode Select,
// Display Brightness Increment,
// Display Brightness Decrement,
// Display Brightness,
// Display Backlight Toggle,
// Display Set Brightness to Minimum,
// Display Set Brightness to Maximum,
// Display Set Auto Brightness,
// Camera Access Enabled,
// Camera Access Disabled,
// Camera Access Toggle,
// Keyboard Brightness Increment,
// Keyboard Brightness Decrement,
// Keyboard Backlight Set Level,
// Keyboard Backlight OOC,
// Keyboard Backlight Set Minimum,
// Keyboard Backlight Set Maximum,
// Keyboard Backlight Auto,
// Selection,
// Assign Selection,
// Mode Step,
// Recall Last,
// Enter Channel,
// Order Movie,
// Channel,
// Media Selection,
// Media Select Computer,
// Media Select TV,
// Media Select WWW,
// Media Select DVD,
// Media Select Telephone,
// Media Select Program Guide,
// Media Select Video Phone,
// Media Select Games,
// Media Select Messages,
// Media Select CD,
// Media Select VCR,
// Media Select Tuner,
// Quit,
// Help,
// Media Select Tape,
// Media Select Cable,
// Media Select Satellite,
// Media Select Security,
// Media Select Home,
// Media Select Call,
// Channel Increment,
// Channel Decrement,
// Media Select SAP,
// VCR Plus,
// Once,
// Daily,
// Weekly,
// Monthly,
// Play,
// Pause,
// Record,
// Fast Forward,
// Rewind,
// Scan Next Track,
// Scan Previous Track,
// Stop,
// Eject,
// Random Play,
// Select Disc,
// Enter Disc,
// Repeat,
// Tracking,
// Track Normal,
// Slow Tracking,
// Frame Forward,
// Frame Back,
// Mark,
// Clear Mark,
// Repeat From Mark,
// Return To Mark,
// Search Mark Forward,
// Search Mark Backwards,
// Counter Reset,
// Show Counter,
// Tracking Increment,
// Tracking Decrement,
// Stop/Eject,
// Play/Pause,
// Play/Skip,
// Voice Command,
// Invoke Capture Interface,
// Start or Stop Game Recording,
// Historical Game Capture,
// Capture Game Screenshot,
// Show or Hide Recording Indicator,
// Start or Stop Microphone Capture,
// Start or Stop Camera Capture,
// Start or Stop Game Broadcast,
// Start or Stop Voice Dictation Session,
// Invoke/Dismiss Emoji Picker,
// Volume,
// Balance,
// Mute,
// Bass,
// Treble,
// Bass Boost,
// Surround Mode,
// Loudness,
// MPX,
// Volume Increment,
// Volume Decrement,
// Speed Select,
// Playback Speed,
// Standard Play,
// Long Play,
// Extended Play,
// Slow,
// Fan Enable,
// Fan Speed,
// Light Enable,
// Light Illumination Level,
// Climate Control Enable,
// Room Temperature,
// Security Enable,
// Fire Alarm,
// Police Alarm,
// Proximity,
// Motion,
// Duress Alarm,
// Holdup Alarm,
// Medical Alarm,
// Balance Right,
// Balance Left,
// Bass Increment,
// Bass Decrement,
// Treble Increment,
// Treble Decrement,
// Speaker System,
// Channel Left,
// Channel Right,
// Channel Center,
// Channel Front,
// Channel Center Front,
// Channel Side,
// Channel Surround,
// Channel Low Frequency Enhancement,
// Channel Top,
// Channel Unknown,
// Sub-channel,
// Sub-channel Increment,
// Sub-channel Decrement,
// Alternate Audio Increment,
// Alternate Audio Decrement,
// Application Launch Buttons,
// AL Launch Button Configuration Tool,
// AL Programmable Button Configuration,
// AL Consumer Control Configuration,
// AL Word Processor,
// AL Text Editor,
// AL Spreadsheet,
// AL Graphics Editor,
// AL Presentation App,
// AL Database App,
// AL Email Reader,
// AL Newsreader,
// AL Voicemail,
// AL Contacts/Address Book,
// AL Calendar/Schedule,
// AL Task/Project Manager,
// AL Log/Journal/Timecard,
// AL Checkbook/Finance,
// AL Calculator,
// AL A/V Capture/Playback,
// AL Local Machine Browser,
// AL LAN/WAN Browser,
// AL Internet Browser,
// AL Remote Networking/ISP Connect,
// AL Network Conference,
// AL Network Chat,
// AL Telephony/Dialer,
// AL Logon,
// AL Logoff,
// AL Logon/Logoff,
// AL Terminal Lock/Screensaver,
// AL Control Panel,
// AL Command Line Processor/Run,
// AL Process/Task Manager,
// AL Select Task/Application,
// AL Next Task/Application,
// AL Previous Task/Application,
// AL Preemptive Halt Task/Application,
// AL Integrated Help Center,
// AL Documents,
// AL Thesaurus,
// AL Dictionary,
// AL Desktop,
// AL Spell Check,
// AL Grammar Check,
// AL Wireless Status,
// AL Keyboard Layout,
// AL Virus Protection,
// AL Encryption,
// AL Screen Saver,
// AL Alarms,
// AL Clock,
// AL File Browser,
// AL Power Status,
// AL Image Browser,
// AL Audio Browser,
// AL Movie Browser,
// AL Digital Rights Manager,
// AL Digital Wallet,
// AL Instant Messaging,
// AL OEM Features/ Tips/Tutorial Browser,
// AL OEM Help,
// AL Online Community,
// AL Entertainment Content Browser,
// AL Online Shopping Browser,
// AL SmartCard Information/Help,
// AL Market Monitor/Finance Browser,
// AL Customized Corporate News Browser,
// AL Online Activity Browser,
// AL Research/Search Browser,
// AL Audio Player,
// AL Message Status,
// AL Contact Sync,
// AL Navigation,
// AL Context-aware Desktop Assistant,
// Generic GUI Application Controls,
// AC New,
// AC Open,
// AC Close,
// AC Exit,
// AC Maximize,
// AC Minimize,
// AC Save,
// AC Print,
// AC Properties,
// AC Undo,
// AC Copy,
// AC Cut,
// AC Paste,
// AC Select All,
// AC Find,
// AC Find and Replace,
// AC Search,
// AC Go To,
// AC Home,
// AC Back,
// AC Forward,
// AC Stop,
// AC Refresh,
// AC Previous Link,
// AC Next Link,
// AC Bookmarks,
// AC History,
// AC Subscriptions,
// AC Zoom In,
// AC Zoom Out,
// AC Zoom,
// AC Full Screen View,
// AC Normal View,
// AC View Toggle,
// AC Scroll Up,
// AC Scroll Down,
// AC Scroll,
// AC Pan Left,
// AC Pan Right,
// AC Pan,
// AC New Window,
// AC Tile Horizontally,
// AC Tile Vertically,
// AC Format,
// AC Edit,
// AC Bold,
// AC Italics,
// AC Underline,
// AC Strikethrough,
// AC Subscript,
// AC Superscript,
// AC All Caps,
// AC Rotate,
// AC Resize,
// AC Flip Horizontal,
// AC Flip Vertical,
// AC Mirror Horizontal,
// AC Mirror Vertical,
// AC Font Select,
// AC Font Color,
// AC Font Size,
// AC Justify Left,
// AC Justify Center H,
// AC Justify Right,
// AC Justify Block H,
// AC Justify Top,
// AC Justify Center V,
// AC Justify Bottom,
// AC Justify Block V,
// AC Indent Decrease,
// AC Indent Increase,
// AC Numbered List,
// AC Restart Numbering,
// AC Bulleted List,
// AC Promote,
// AC Demote,
// AC Yes,
// AC No,
// AC Cancel,
// AC Catalog,
// AC Buy/Checkout,
// AC Add to Cart,
// AC Expand,
// AC Expand All,
// AC Collapse,
// AC Collapse All,
// AC Print Preview,
// AC Paste Special,
// AC Insert Mode,
// AC Delete,
// AC Lock,
// AC Unlock,
// AC Protect,
// AC Unprotect,
// AC Attach Comment,
// AC Delete Comment,
// AC View Comment,
// AC Select Word,
// AC Select Sentence,
// AC Select Paragraph,
// AC Select Column,
// AC Select Row,
// AC Select Table,
// AC Select Object,
// AC Redo/Repeat,
// AC Sort,
// AC Sort Ascending,
// AC Sort Descending,
// AC Filter,
// AC Set Clock,
// AC View Clock,
// AC Select Time Zone,
// AC Edit Time Zones,
// AC Set Alarm,
// AC Clear Alarm,
// AC Snooze Alarm,
// AC Reset Alarm,
// AC Synchronize,
// AC Send/Receive,
// AC Send To,
// AC Reply,
// AC Reply All,
// AC Forward Msg,
// AC Send,
// AC Attach File,
// AC Upload,
// AC Download (Save Target As),
// AC Set Borders,
// AC Insert Row,
// AC Insert Column,
// AC Insert File,
// AC Insert Picture,
// AC Insert Object,
// AC Insert Symbol,
// AC Save and Close,
// AC Rename,
// AC Merge,
// AC Split,
// AC Disribute Horizontally,
// AC Distribute Vertically,
// AC Next Keyboard Layout Select,
// AC Navigation Guidance,
// AC Desktop Show All Windows,
// AC Soft Key Left,
// AC Soft Key Right,
// AC Desktop Show All Applications,
// AC Idle Keep Alive,
// Extended Keyboard Attributes Collection,
// Keyboard Form Factor,
// Keyboard Key Type,
// Keyboard Physical Layout,
// Vendor-Specific Keyboard Physical Layout,
// Keyboard IETF Language Tag Index,
// Implemented Keyboard Input Assist Controls,
// Keyboard Input Assist Previous,
// Keyboard Input Assist Next,
// Keyboard Input Assist Previous Group,
// Keyboard Input Assist Next Group,
// Keyboard Input Assist Accept,
// Keyboard Input Assist Cancel,
// Privacy Screen Toggle,
// Privacy Screen Level Decrement,
// Privacy Screen Level Increment,
// Privacy Screen Level Minimum,
// Privacy Screen Level Maximum,
// Contact Edited,
// Contact Added,
// Contact Record Active,
// Contact Index,
// Contact Nickname,
// Contact First Name,
// Contact Last Name,
// Contact Full Name,
// Contact Phone Number Personal,
// Contact Phone Number Business,
// Contact Phone Number Mobile,
// Contact Phone Number Pager,
// Contact Phone Number Fax,
// Contact Phone Number Other,
// Contact Email Personal,
// Contact Email Business,
// Contact Email Other,
// Contact Email Main,
// Contact Speed Dial Number,
// Contact Status Flag,
// Contact Misc.,
// Reserved

// 00 Undefined
// 01 Consumer Control
// 02 Numeric Key Pad
// 03 Programmable Buttons
// 04 Microphone
// 05 Headphone
// 06 Graphic Equalizer
// 07-1F Reserved
// 20 +10
// 21 +100
// 22 AM/PM
// 23-2F Reserved
// 30 Power
// 31 Reset
// 32 Sleep
// 33 Sleep After
// 34 Sleep Mode
// 35 Illumination
// 36 Function Buttons
// 37-3F Reserved
// 40 Menu
// 41 Menu Pick
// 42 Menu Up
// 43 Menu Down
// 44 Menu Left
// 45 Menu Right
// 46 Menu Escape
// 47 Menu Value Increase
// 48 Menu Value Decrease
// 49-5F Reserved
// 60 Data On Screen
// 61 Closed Caption
// 62 Closed Caption Select
// 63 VCR/TV
// 64 Broadcast Mode
// 65 Snapshot
// 66 Still
// 67 Picture-in-Picture Toggle
// 68 Picture-in-Picture Swap
// 69 Red Menu Button
// 6A Green Menu Button
// 6B Blue Menu Button
// 6C Yellow Menu Button
// 6D Aspect
// 6E 3D Mode Select
// 6F Display Brightness Increment
// 70 Display Brightness Decrement
// 71 Display Brightness
// 72 Display Backlight Toggle
// 73 Display Set Brightness to Minimum
// 74 Display Set Brightness to Maximum
// 75 Display Set Auto Brightness
// 76 Camera Access Enabled
// 77 Camera Access Disabled
// 78 Camera Access Toggle
// 79 Keyboard Brightness Increment
// 7A Keyboard Brightness Decrement
// 7B Keyboard Backlight Set Level
// 7C Keyboard Backlight OOC
// 7D Keyboard Backlight Set Minimum
// 7E Keyboard Backlight Set Maximum
// 7F Keyboard Backlight Auto
// 80 Selection
// 81 Assign Selection
// 82 Mode Step
// 83 Recall Last
// 84 Enter Channel
// 85 Order Movie
// 86 Channel
// 87 Media Selection
// 88 Media Select Computer
// 89 Media Select TV
// 8A Media Select WWW
// 8B Media Select DVD
// 8C Media Select Telephone
// 8D Media Select Program Guide
// 8E Media Select Video Phone
// 8F Media Select Games
// 90 Media Select Messages
// 91 Media Select CD
// 92 Media Select VCR
// 93 Media Select Tuner
// 94 Quit
// 95 Help
// 96 Media Select Tape
// 97 Media Select Cable
// 98 Media Select Satellite
// 99 Media Select Security
// 9A Media Select Home
// 9B Media Select Call
// 9C Channel Increment
// 9D Channel Decrement
// 9E Media Select SAP
// 9F-9F Reserved
// A0 VCR Plus
// A1 Once
// A2 Daily
// A3 Weekly
// A4 Monthly
// A5-AF Reserved
// B0 Play
// B1 Pause
// B2 Record
// B3 Fast Forward
// B4 Rewind
// B5 Scan Next Track
// B6 Scan Previous Track
// B7 Stop
// B8 Eject
// B9 Random Play
// BA Select Disc
// BB Enter Disc
// BC Repeat
// BD Tracking
// BE Track Normal
// BF Slow Tracking
// C0 Frame Forward
// C1 Frame Back
// C2 Mark
// C3 Clear Mark
// C4 Repeat From Mark
// C5 Return To Mark
// C6 Search Mark Forward
// C7 Search Mark Backwards
// C8 Counter Reset
// C9 Show Counter
// CA Tracking Increment
// CB Tracking Decrement
// CC Stop/Eject
// CD Play/Pause
// CE Play/Skip
// CF Voice Command
// D0 Invoke Capture Interface
// D1 Start or Stop Game Recording
// D2 Historical Game Capture
// D3 Capture Game Screenshot
// D4 Show or Hide Recording Indicator
// D5 Start or Stop Microphone Capture
// D6 Start or Stop Camera Capture
// D7 Start or Stop Game Broadcast
// D8 Start or Stop Voice Dictation Session
// D9 Invoke/Dismiss Emoji Picker
// DA-DF Reserved
// E0 Volume
// E1 Balance
// E2 Mute
// E3 Bass
// E4 Treble
// E5 Bass Boost
// E6 Surround Mode
// E7 Loudness
// E8 MPX
// E9 Volume Increment
// EA Volume Decrement
// EB-EF Reserved
// F0 Speed Select
// F1 Playback Speed
// F2 Standard Play
// F3 Long Play
// F4 Extended Play
// F5 Slow
// F6-FF Reserved
// 100 Fan Enable
// 101 Fan Speed
// 102 Light Enable
// 103 Light Illumination Level
// 104 Climate Control Enable
// 105 Room Temperature
// 106 Security Enable
// 107 Fire Alarm
// 108 Police Alarm
// 109 Proximity
// 10A Motion
// 10B Duress Alarm
// 10C Holdup Alarm
// 10D Medical Alarm
// 10E-14F Reserved
// 150 Balance Right
// 151 Balance Left
// 152 Bass Increment
// 153 Bass Decrement
// 154 Treble Increment
// 155 Treble Decrement
// 156-15F Reserved
// 160 Speaker System
// 161 Channel Left
// 162 Channel Right
// 163 Channel Center
// 164 Channel Front
// 165 Channel Center Front
// 166 Channel Side
// 167 Channel Surround
// 168 Channel Low Frequency Enhancement
// 169 Channel Top
// 16A Channel Unknown
// 16B-16F Reserved
// 170 Sub-channel
// 171 Sub-channel Increment
// 172 Sub-channel Decrement
// 173 Alternate Audio Increment
// 174 Alternate Audio Decrement
// 175-17F Reserved
// 180 Application Launch Buttons
// 181 AL Launch Button Configuration Tool
// 182 AL Programmable Button Configuration
// 183 AL Consumer Control Configuration
// 184 AL Word Processor
// 185 AL Text Editor
// 186 AL Spreadsheet
// 187 AL Graphics Editor
// 188 AL Presentation App
// 189 AL Database App
// 18A AL Email Reader
// 18B AL Newsreader
// 18C AL Voicemail
// 18D AL Contacts/Address Book
// 18E AL Calendar/Schedule
// 18F AL Task/Project Manager
// 190 AL Log/Journal/Timecard
// 191 AL Checkbook/Finance
// 192 AL Calculator
// 193 AL A/V Capture/Playback
// 194 AL Local Machine Browser
// 195 AL LAN/WAN Browser
// 196 AL Internet Browser
// 197 AL Remote Networking/ISP Connect
// 198 AL Network Conference
// 199 AL Network Chat
// 19A AL Telephony/Dialer
// 19B AL Logon
// 19C AL Logoff
// 19D AL Logon/Logoff
// 19E AL Terminal Lock/Screensaver
// 19F AL Control Panel
// 1A0 AL Command Line Processor/Run
// 1A1 AL Process/Task Manager
// 1A2 AL Select Task/Application
// 1A3 AL Next Task/Application
// 1A4 AL Previous Task/Application
// 1A5 AL Preemptive Halt Task/Application
// 1A6 AL Integrated Help Center
// 1A7 AL Documents
// 1A8 AL Thesaurus
// 1A9 AL Dictionary
// 1AA AL Desktop
// 1AB AL Spell Check
// 1AC AL Grammar Check
// 1AD AL Wireless Status
// 1AE AL Keyboard Layout
// 1AF AL Virus Protection
// 1B0 AL Encryption
// 1B1 AL Screen Saver
// 1B2 AL Alarms
// 1B3 AL Clock
// 1B4 AL File Browser
// 1B5 AL Power Status
// 1B6 AL Image Browser
// 1B7 AL Audio Browser
// 1B8 AL Movie Browser
// 1B9 AL Digital Rights Manager
// 1BA AL Digital Wallet
// 1BB-1BB Reserved
// 1BC AL Instant Messaging
// 1BD AL OEM Features/ Tips/Tutorial Browser
// 1BE AL OEM Help
// 1BF AL Online Community
// 1C0 AL Entertainment Content Browser
// 1C1 AL Online Shopping Browser
// 1C2 AL SmartCard Information/Help
// 1C3 AL Market Monitor/Finance Browser
// 1C4 AL Customized Corporate News Browser
// 1C5 AL Online Activity Browser
// 1C6 AL Research/Search Browser
// 1C7 AL Audio Player
// 1C8 AL Message Status
// 1C9 AL Contact Sync
// 1CA AL Navigation
// 1CB AL Context-aware Desktop Assistant
// 1CC-1FF Reserved
// 200 Generic GUI Application Controls
// 201 AC New
// 202 AC Open
// 203 AC Close
// 204 AC Exit
// 205 AC Maximize
// 206 AC Minimize
// 207 AC Save
// 208 AC Print
// 209 AC Properties
// 20A-219 Reserved
// 21A AC Undo
// 21B AC Copy
// 21C AC Cut
// 21D AC Paste
// 21E AC Select All
// 21F AC Find
// 220 AC Find and Replace
// 221 AC Search
// 222 AC Go To
// 223 AC Home
// 224 AC Back
// 225 AC Forward
// 226 AC Stop
// 227 AC Refresh
// 228 AC Previous Link
// 229 AC Next Link
// 22A AC Bookmarks
// 22B AC History
// 22C AC Subscriptions
// 22D AC Zoom In
// 22E AC Zoom Out
// 22F AC Zoom
// 230 AC Full Screen View
// 231 AC Normal View
// 232 AC View Toggle
// 233 AC Scroll Up
// 234 AC Scroll Down
// 235 AC Scroll
// 236 AC Pan Left
// 237 AC Pan Right
// 238 AC Pan
// 239 AC New Window
// 23A AC Tile Horizontally
// 23B AC Tile Vertically
// 23C AC Format
// 23D AC Edit
// 23E AC Bold
// 23F AC Italics
// 240 AC Underline
// 241 AC Strikethrough
// 242 AC Subscript
// 243 AC Superscript
// 244 AC All Caps
// 245 AC Rotate
// 246 AC Resize
// 247 AC Flip Horizontal
// 248 AC Flip Vertical
// 249 AC Mirror Horizontal
// 24A AC Mirror Vertical
// 24B AC Font Select
// 24C AC Font Color
// 24D AC Font Size
// 24E AC Justify Left
// 24F AC Justify Center H
// 250 AC Justify Right
// 251 AC Justify Block H
// 252 AC Justify Top
// 253 AC Justify Center V
// 254 AC Justify Bottom
// 255 AC Justify Block V
// 256 AC Indent Decrease
// 257 AC Indent Increase
// 258 AC Numbered List
// 259 AC Restart Numbering
// 25A AC Bulleted List
// 25B AC Promote
// 25C AC Demote
// 25D AC Yes
// 25E AC No
// 25F AC Cancel
// 260 AC Catalog
// 261 AC Buy/Checkout
// 262 AC Add to Cart
// 263 AC Expand
// 264 AC Expand All
// 265 AC Collapse
// 266 AC Collapse All
// 267 AC Print Preview
// 268 AC Paste Special
// 269 AC Insert Mode
// 26A AC Delete
// 26B AC Lock
// 26C AC Unlock
// 26D AC Protect
// 26E AC Unprotect
// 26F AC Attach Comment
// 270 AC Delete Comment
// 271 AC View Comment
// 272 AC Select Word
// 273 AC Select Sentence
// 274 AC Select Paragraph
// 275 AC Select Column
// 276 AC Select Row
// 277 AC Select Table
// 278 AC Select Object
// 279 AC Redo/Repeat
// 27A AC Sort
// 27B AC Sort Ascending
// 27C AC Sort Descending
// 27D AC Filter
// 27E AC Set Clock
// 27F AC View Clock
// 280 AC Select Time Zone
// 281 AC Edit Time Zones
// 282 AC Set Alarm
// 283 AC Clear Alarm
// 284 AC Snooze Alarm
// 285 AC Reset Alarm
// 286 AC Synchronize
// 287 AC Send/Receive
// 288 AC Send To
// 289 AC Reply
// 28A AC Reply All
// 28B AC Forward Msg
// 28C AC Send
// 28D AC Attach File
// 28E AC Upload
// 28F AC Download (Save Target As)
// 290 AC Set Borders
// 291 AC Insert Row
// 292 AC Insert Column
// 293 AC Insert File
// 294 AC Insert Picture
// 295 AC Insert Object
// 296 AC Insert Symbol
// 297 AC Save and Close
// 298 AC Rename
// 299 AC Merge
// 29A AC Split
// 29B AC Disribute Horizontally
// 29C AC Distribute Vertically
// 29D AC Next Keyboard Layout Select
// 29E AC Navigation Guidance
// 29F AC Desktop Show All Windows
// 2A0 AC Soft Key Left
// 2A1 AC Soft Key Right
// 2A2 AC Desktop Show All Applications
// 2A3-2AF Reserved
// 2B0 AC Idle Keep Alive
// 2B1-2BF Reserved
// 2C0 Extended Keyboard Attributes Collection
// 2C1 Keyboard Form Factor
// 2C2 Keyboard Key Type
// 2C3 Keyboard Physical Layout
// 2C4 Vendor-Specific Keyboard Physical Layout
// 2C5 Keyboard IETF Language Tag Index
// 2C6 Implemented Keyboard Input Assist Controls
// 2C7 Keyboard Input Assist Previous
// 2C8 Keyboard Input Assist Next
// 2C9 Keyboard Input Assist Previous Group
// 2CA Keyboard Input Assist Next Group
// 2CB Keyboard Input Assist Accept
// 2CC Keyboard Input Assist Cancel
// 2CD-2CF Reserved
// 2D0 Privacy Screen Toggle
// 2D1 Privacy Screen Level Decrement
// 2D2 Privacy Screen Level Increment
// 2D3 Privacy Screen Level Minimum
// 2D4 Privacy Screen Level Maximum
// 2D5-4FF Reserved
// 500 Contact Edited
// 501 Contact Added
// 502 Contact Record Active
// 503 Contact Index
// 504 Contact Nickname
// 505 Contact First Name
// 506 Contact Last Name
// 507 Contact Full Name
// 508 Contact Phone Number Personal
// 509 Contact Phone Number Business
// 50A Contact Phone Number Mobile
// 50B Contact Phone Number Pager
// 50C Contact Phone Number Fax
// 50D Contact Phone Number Other
// 50E Contact Email Personal
// 50F Contact Email Business
// 510 Contact Email Other
// 511 Contact Email Main
// 512 Contact Speed Dial Number
// 513 Contact Status Flag
// 514 Contact Misc.
// 515-FFFF Reserved
