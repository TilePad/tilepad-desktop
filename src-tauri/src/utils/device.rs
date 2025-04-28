pub fn get_device_name() -> String {
    gethostname::gethostname().to_string_lossy().to_string()
}
