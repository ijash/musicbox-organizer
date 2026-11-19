# Contract: Device Compatibility Service

```rust
pub trait DeviceCompatibilityChecker {
    fn validate_filenames(&self, filenames: &[String]) -> CompatibilityReport;
    fn validate_directory(&self, path: &Path) -> Result<CompatibilityReport, Box<dyn Error>>;
    fn sanitize_filename(&self, raw_name: &str) -> String;
}
```
