#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum AppleDeviceTarget {
    MacOS,
    IPhoneOS,
    TvOS,
    WatchOS,
}

impl TryFrom<&str> for AppleDeviceTarget {
    type Error = ();

    fn try_from(sdk_name: &str) -> Result<Self, Self::Error> {
        match sdk_name {
            "appletvos" => Ok(AppleDeviceTarget::TvOS),
            "iphoneos" => Ok(AppleDeviceTarget::IPhoneOS),
            "macosx" => Ok(AppleDeviceTarget::MacOS),
            "watchos" => Ok(AppleDeviceTarget::WatchOS),
            _ => Err(())
        }
    }
}

impl ToString for AppleDeviceTarget {
    fn to_string(&self) -> String {
        match self {
            AppleDeviceTarget::TvOS => "appletvos",
            AppleDeviceTarget::IPhoneOS => "iphoneos",
            AppleDeviceTarget::MacOS => "macosx",
            AppleDeviceTarget::WatchOS => "watchos",
        }.into()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum AppleTarget {
    Device(AppleDeviceTarget),
    Simulator(AppleDeviceTarget)
}

impl TryFrom<&str> for AppleTarget {
    type Error = ();

    fn try_from(sdk_name: &str) -> Result<Self, Self::Error> {
        match sdk_name {
            "appletvos" => Ok(AppleTarget::Device(AppleDeviceTarget::TvOS)),
            "appletvsimulator" => Ok(AppleTarget::Simulator(AppleDeviceTarget::TvOS)),
            "iphoneos" => Ok(AppleTarget::Device(AppleDeviceTarget::IPhoneOS)),
            "iphonesimulator" => Ok(AppleTarget::Simulator(AppleDeviceTarget::IPhoneOS)),
            "macosx" => Ok(AppleTarget::Device(AppleDeviceTarget::MacOS)),
            "watchos" => Ok(AppleTarget::Device(AppleDeviceTarget::WatchOS)),
            "watchsimulator" => Ok(AppleTarget::Simulator(AppleDeviceTarget::WatchOS)),
            _ => Err(())
        }
    }
}

impl ToString for AppleTarget {
    fn to_string(&self) -> String {
        match self {
            AppleTarget::Simulator(device_target) => format!("{}simulator", device_target.to_string()),
            AppleTarget::Device(device_target) => device_target.to_string()
        }
    }
}