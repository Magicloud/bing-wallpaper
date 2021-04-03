mod bindings {
    ::windows::include_bindings!();
}

use bindings::{
    Windows::Win32::WindowsAndMessaging::{
        SYSTEM_PARAMETERS_INFO_ACTION,
        SystemParametersInfo_fWinIni,
        SystemParametersInfoW
    },
    Windows::Storage::StorageFile,
    Windows::System::UserProfile::LockScreen
};

use reqwest::blocking::get;
use sxd_document::parser::parse;
use sxd_xpath::evaluate_xpath;
use std::fs::File;
use std::io::copy;
use std::env::var;
use std::ffi::OsString;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;

fn update_wallpaper(pic_filename : String) -> Result<(), Box<dyn std::error::Error>> {
    let pic = OsString::from(pic_filename).as_os_str().encode_wide().chain(once(0)).collect::<Vec<u16>>();
    unsafe {
        SystemParametersInfoW(SYSTEM_PARAMETERS_INFO_ACTION::SPI_SETDESKWALLPAPER,
            0, pic.as_ptr() as *mut _,
            SystemParametersInfo_fWinIni::SPIF_SENDWININICHANGE);
    }
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // idx is from today back.
    // mkt : en-US,zh-CN,ja-JP,en-AU,en-UK,de-DE,en-NZ,en-CA
    let page_url = "http://www.bing.com/HPImageArchive.aspx?format=xml&idx=0&n=1&mkt=zh-CN";
    let pic_file_path = format!("{}/wallpaper.jpg", var("TEMP")?);
    let body = get(page_url)?.text()?;
    let pkg = parse(&body)?;
    let doc = pkg.as_document();
    let value = evaluate_xpath(&doc, "//urlBase/text()")?;
    let pic_url = format!("http://www.bing.com{}_1920x1080.jpg", value.string());
    let img_bytes = get(pic_url)?.bytes()?;
    let mut data = img_bytes.as_ref();
    let mut f = File::create(pic_file_path.clone())?;
    copy(&mut data, &mut f)?;
    update_wallpaper(pic_file_path)?;
    Ok(())
}