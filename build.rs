fn main() {
    windows::build!(
        Windows::Win32::WindowsAndMessaging::{
            SYSTEM_PARAMETERS_INFO_ACTION,
            SystemParametersInfo_fWinIni,
            SystemParametersInfoW
        },
        Windows::Foundation::{
            IAsyncAction,
            IAsyncOperation,
            AsyncActionCompletedHandler
        },
        Windows::Storage::StorageFile,
        Windows::System::UserProfile::LockScreen
    );
}