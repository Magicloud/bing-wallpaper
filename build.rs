fn main() {
    windows::build!(
        Windows::Foundation::IAsyncOperation,
        Windows::Storage::StorageFile,
        Windows::System::UserProfile::UserProfilePersonalizationSettings
    );
}