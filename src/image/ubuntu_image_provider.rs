use crate::image::ImageProvider;
use crate::models::{Arch, HashAlg};
use crate::util;

pub struct UbuntuImageProvider {}

impl ImageProvider for UbuntuImageProvider {
    fn get_distro(&self) -> &str {
        "ubuntu"
    }

    fn get_base_url(&self) -> &str {
        "https://cloud-images.ubuntu.com/minimal/releases/"
    }

    fn find_image_names(&self, content: &str) -> Vec<String> {
        util::find_and_extract(r#"href=\"([a-z]+)/\""#, content)
    }

    fn get_image_dir_path(&self, name: &str, _arch: Arch) -> String {
        format!("{name}/release/")
    }

    fn get_version(&self, image_file: &str, name: &str) -> String {
        util::find_and_extract(r"ubuntu-([^-]+)-minimal-cloudimg-[^.]+.img", image_file)
            .into_iter()
            .next()
            .unwrap_or_else(|| name.to_string())
    }

    fn get_codename(&self, name: &str) -> Option<String> {
        Some(name.to_string())
    }

    fn get_image_file_pattern(&self, _name: &str, arch: Arch) -> String {
        let arch_name = arch.as_vendor_str();
        format!("ubuntu-[0-9]+\\.[0-9]+-minimal-cloudimg-{arch_name}.img")
    }

    fn get_checksum_file(&self, _image_file: &str, _name: &str, _arch: Arch) -> String {
        "SHA256SUMS".to_string()
    }

    fn get_checksum_alg(&self) -> HashAlg {
        HashAlg::Sha256
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_find_image_names_in_listing() {
        let listing = r#"<a href="jammy/">jammy/</a>
<a href="noble/">noble/</a>"#;

        assert_eq!(
            UbuntuImageProvider {}.find_image_names(listing),
            ["jammy", "noble"]
        );
    }

    #[test]
    fn test_get_version_and_codename() {
        let provider = UbuntuImageProvider {};

        assert_eq!(
            provider.get_version("ubuntu-24.04-minimal-cloudimg-amd64.img", "noble"),
            "24.04"
        );
        assert_eq!(provider.get_codename("noble"), Some("noble".to_string()));
    }

    #[test]
    fn test_image_file_pattern_matches_image_file() {
        let pattern = UbuntuImageProvider {}.get_image_file_pattern("noble", Arch::ARM64);

        assert!(
            Regex::new(&pattern)
                .unwrap()
                .is_match("ubuntu-24.04-minimal-cloudimg-arm64.img")
        );
    }
}
