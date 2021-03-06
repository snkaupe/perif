use crate::{
    device::SupportedDevice,
    device::DeviceKind
};

mod ath_g1wl;

pub fn supported_devices() -> Vec<SupportedDevice> {

    vec![
        SupportedDevice::new(
            "Audio Technica ATH-G1WL",
            DeviceKind::Headset,
            0x0451u16,
            0x16bau16,
            Some(ath_g1wl::get_battery),
            None,
            None,
            None
        )
    ]

}
