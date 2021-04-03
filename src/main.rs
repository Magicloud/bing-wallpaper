mod bindings {
    ::windows::include_bindings!();
}

use bindings::{
    Windows::Storage::StorageFile,
    Windows::System::UserProfile::UserProfilePersonalizationSettings
};

use reqwest::blocking::get;
use sxd_document::parser::parse;
use sxd_xpath::evaluate_xpath;
use std::fs::File;
use std::io::copy;
use std::env::var;

async fn set_all(pic_filename : String) -> Result<(), Box<dyn std::error::Error>> {
    let file = StorageFile::GetFileFromPathAsync(pic_filename)?.await?;
    let upps = UserProfilePersonalizationSettings::Current()?;
    upps.TrySetWallpaperImageAsync(&file)?.await?;
    upps.TrySetLockScreenImageAsync(&file)?.await?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if UserProfilePersonalizationSettings::IsSupported()? {
        // idx is from today back.
        // mkt : en-US,zh-CN,ja-JP,en-AU,en-UK,de-DE,en-NZ,en-CA
        let page_url = "http://www.bing.com/HPImageArchive.aspx?format=xml&idx=0&n=1&mkt=zh-CN";
        let pic_file_path = format!("{}\\wallpaper.jpg", var("TEMP")?);
        let body = get(page_url)?.text()?;
        let pkg = parse(&body)?;
        let doc = pkg.as_document();
        let value = evaluate_xpath(&doc, "//urlBase/text()")?;
        let pic_url = format!("http://www.bing.com{}_1920x1080.jpg", value.string());
        let img_bytes = get(pic_url)?.bytes()?;
        let mut data = img_bytes.as_ref();
        let mut f = File::create(&pic_file_path)?;
        copy(&mut data, &mut f)?;
        futures::executor::block_on(set_all(pic_file_path))?;
        Ok(())
    } else {
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "not supported")))
    }
}