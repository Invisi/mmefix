use windows::core::implement;
use windows::core::PCWSTR;
use windows::Win32::Media::Audio::{
    EDataFlow, ERole, IMMNotificationClient, IMMNotificationClient_Impl,
};
use windows::Win32::UI::Shell::PropertiesSystem::PROPERTYKEY;

#[implement(IMMNotificationClient)]
pub(crate) struct ListenClient;

#[allow(non_snake_case)]
impl IMMNotificationClient_Impl for ListenClient {
    fn OnDeviceStateChanged(
        &self,
        pwstrdeviceid: &PCWSTR,
        dwnewstate: u32,
    ) -> windows::core::Result<()> {
        println!("{:?} {:?}", pwstrdeviceid, dwnewstate);

        Ok(())
    }

    fn OnDeviceAdded(&self, _pwstrdeviceid: &PCWSTR) -> windows::core::Result<()> {
        Ok(())
    }

    fn OnDeviceRemoved(&self, _pwstrdeviceid: &PCWSTR) -> windows::core::Result<()> {
        Ok(())
    }

    fn OnDefaultDeviceChanged(
        &self,
        _flow: EDataFlow,
        _role: ERole,
        _pwstrdefaultdeviceid: &PCWSTR,
    ) -> windows::core::Result<()> {
        Ok(())
    }

    fn OnPropertyValueChanged(
        &self,
        pwstrdeviceid: &PCWSTR,
        key: &PROPERTYKEY,
    ) -> windows::core::Result<()> {
        unsafe {
            println!("{:?} {:?}", pwstrdeviceid.to_string().unwrap(), key);
        }

        Ok(())
    }
}
