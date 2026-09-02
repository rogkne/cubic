use crate::error::{Error, Result};
use crate::image::{self, ImageCache};
use crate::models::{Arch, Environment, Image, ImageName};
use crate::platform::System;
use crate::util;
use crate::view::Console;
use crate::web::WebClient;
use std::path::Path;

const IMAGE_PROVIDERS: &[&dyn image::ImageProvider] = &[
    &image::AlmaLinuxImageProvider {},
    &image::ArchLinuxImageProvider {},
    &image::DebianImageProvider {},
    &image::FedoraImageProvider {},
    &image::GentooImageProvider {},
    &image::OpenSuseImageProvider {},
    &image::RockyLinuxImageProvider {},
    &image::UbuntuImageProvider {},
];

pub struct ImageFactory<'a> {
    env: Environment,
    system: &'a dyn System,
}

impl<'a> ImageFactory<'a> {
    pub fn new(system: &'a dyn System, env: &Environment) -> Self {
        Self {
            env: env.clone(),
            system,
        }
    }

    fn filter_arch(filter: Option<ImageName>) -> Vec<Arch> {
        let mut arches = vec![Arch::AMD64, Arch::ARM64];

        if let Some(filter) = filter {
            arches.retain(|a| filter.get_arch() == *a);
        }

        arches
    }

    fn get_images_from_provider_name_arch(
        console: &mut Console<'_>,
        web: &mut WebClient,
        image_provider: &dyn image::ImageProvider,
        name: &str,
        arch: Arch,
    ) -> Option<Image> {
        let image_dir_url = format!(
            "{}{}",
            image_provider.get_base_url(),
            &image_provider.get_image_dir_path(name, arch)
        );
        console.debug(&format!(
            "Fetching image directory listing '{image_dir_url}'"
        ));
        let image_content = match web.download_content(&image_dir_url) {
            Ok(content) => content,
            Err(e) => {
                console.debug(&format!(
                    "Cannot fetch image directory listing '{image_dir_url}' ({e})"
                ));
                return None;
            }
        };

        let image_file = util::find_and_extract(
            &format!(
                "href=\"\\.?/?({})\"",
                image_provider.get_image_file_pattern(name, arch)
            ),
            &image_content,
        );
        let image_file = image_file.first();

        if let Some(image_file) = image_file {
            let image_url = format!("{image_dir_url}{image_file}");
            web.get_file_size(&image_url)
                .ok()
                .and_then(|size| size)
                .map(|size| Image {
                    distro: image_provider.get_distro().to_string(),
                    version: image_provider.get_version(image_file, name),
                    codename: image_provider.get_codename(name),
                    tags: Vec::new(),
                    arch,
                    image_url,
                    checksum_url: format!(
                        "{image_dir_url}{}",
                        image_provider.get_checksum_file(image_file, name, arch)
                    ),
                    hash_alg: image_provider.get_checksum_alg(),
                    size: Some(size),
                })
        } else {
            None
        }
    }

    fn get_images_from_provider(
        console: &mut Console<'_>,
        web: &mut WebClient,
        image_provider: &dyn image::ImageProvider,
        filter: Option<ImageName>,
    ) -> Vec<Image> {
        web.download_content(image_provider.get_base_url())
            .map(|content| {
                image_provider
                    .find_image_names(&content)
                    .into_iter()
                    .flat_map(|name| {
                        Self::filter_arch(filter.clone())
                            .into_iter()
                            .flat_map(|arch| {
                                Self::get_images_from_provider_name_arch(
                                    console,
                                    web,
                                    image_provider,
                                    &name,
                                    arch,
                                )
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    fn get_images(
        console: &mut Console<'_>,
        web: &mut WebClient,
        filter: Option<ImageName>,
    ) -> Vec<Image> {
        IMAGE_PROVIDERS
            .iter()
            .filter(|p| filter.is_none() || filter.as_ref().unwrap().get_distro() == p.get_distro())
            .flat_map(|provider| {
                Self::get_images_from_provider(console, web, *provider, filter.clone())
            })
            .collect()
    }

    fn find_matching_image(images: &[Image], filter: &ImageName) -> Option<Image> {
        images
            .iter()
            .find(|image| {
                image.distro == filter.get_distro()
                    && image.arch == filter.get_arch()
                    && image.has_name(filter.get_name())
            })
            .cloned()
    }

    fn read_images(
        &self,
        console: &mut Console<'_>,
        filter: Option<ImageName>,
    ) -> Result<Vec<Image>> {
        // Read cache
        let cache =
            ImageCache::read_from_file(self.system, Path::new(&self.env.get_image_cache_file()));

        // Use cache if valid
        let images = if let Some(cache) = &cache
            && cache.is_valid()
        {
            console.debug("Using cached image list");
            cache.images.clone()
        } else {
            // Fetch image info
            console.debug("Image cache missing or stale, fetching image list from providers");
            let images = Self::get_images(console, &mut WebClient::new()?, filter.clone());

            // Return cache if fetching failed
            if images.is_empty()
                && let Some(cache) = &cache
            {
                console.debug("Fetching image list failed, falling back to stale cache");
                cache.images.clone()
            } else {
                // Write cache
                if filter.is_none() {
                    ImageCache::new(images.clone())
                        .write_to_file(self.system, Path::new(&self.env.get_image_cache_file()));
                }
                images
            }
        };

        Ok(match &filter {
            Some(name) => Self::find_matching_image(&images, name)
                .into_iter()
                .collect(),
            None => images,
        })
    }

    pub fn get_all_images(&self, console: &mut Console<'_>) -> Result<Vec<Image>> {
        self.read_images(console, None)
    }

    pub fn find_image(&self, console: &mut Console<'_>, name: &ImageName) -> Result<Image> {
        self.read_images(console, Some(name.clone()))
            .and_then(|images| {
                images
                    .into_iter()
                    .next()
                    .ok_or_else(|| Error::UnknownImage(name.to_string()))
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::HashAlg;
    use std::str::FromStr;

    fn build_image(distro: &str, version: &str, codename: Option<&str>, arch: Arch) -> Image {
        Image {
            distro: distro.to_string(),
            version: version.to_string(),
            codename: codename.map(str::to_string),
            tags: Vec::new(),
            arch,
            image_url: "image_url".to_string(),
            checksum_url: "checksum_url".to_string(),
            hash_alg: HashAlg::Sha256,
            size: None,
        }
    }

    #[test]
    fn test_find_matching_image_matches_distro_arch_and_name() {
        let images = vec![
            build_image("almalinux", "9", None, Arch::AMD64),
            build_image("debian", "12", Some("bookworm"), Arch::AMD64),
        ];
        let filter = ImageName::from_str("debian:bookworm:amd64").unwrap();

        let found = ImageFactory::find_matching_image(&images, &filter);

        assert_eq!(found, Some(images[1].clone()));
    }

    #[test]
    fn test_find_matching_image_returns_none_on_a_mismatch() {
        let images = vec![build_image("debian", "12", Some("bookworm"), Arch::AMD64)];

        for name in [
            "ubuntu:bookworm:amd64",
            "debian:bookworm:arm64",
            "debian:bullseye:amd64",
        ] {
            let filter = ImageName::from_str(name).unwrap();

            assert_eq!(ImageFactory::find_matching_image(&images, &filter), None);
        }
    }

    #[test]
    fn test_filter_arch_keeps_the_filtered_arch_only() {
        let filter = ImageName::from_str("debian:bookworm:arm64").unwrap();

        assert_eq!(
            ImageFactory::filter_arch(None),
            vec![Arch::AMD64, Arch::ARM64]
        );
        assert_eq!(ImageFactory::filter_arch(Some(filter)), vec![Arch::ARM64]);
    }
}
