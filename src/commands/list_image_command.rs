use crate::commands::{AllImagesArg, Command, Context, fetch_image_list};
use crate::error::Result;
use crate::image::ImageStore;
use crate::models::{Arch, DataSize};
use crate::view::{Alignment, Console, TableView};
use clap::Parser;

/// List VM images
///
/// Examples:
///
///   $ cubic images
///   Name                Arch         Size   Cached
///   archlinux:rolling   amd64   530.8 MiB       no
///   debian:12           amd64   428.7 MiB       no
///   debian:13           amd64   413.6 MiB      yes
///   fedora:43           amd64   556.3 MiB       no
///   fedora:44           amd64   556.7 MiB       no
///   [...]
///   ubuntu:25.10        amd64   394.2 MiB       no
///   ubuntu:26.04        amd64   407.8 MiB      yes
///   [...]
///
///
#[derive(Parser)]
#[clap(verbatim_doc_comment)]
pub struct ListImageCommand {
    #[clap(flatten)]
    all: AllImagesArg,
}

impl Command for ListImageCommand {
    fn run(&self, console: &mut Console<'_>, context: &Context) -> Result<()> {
        let images = fetch_image_list(console, context.get_system(), context.get_env());

        let mut view = TableView::new();
        view.add_row()
            .add("Name", Alignment::Left)
            .add("Arch", Alignment::Left)
            .add("Size", Alignment::Right)
            .add("Cached", Alignment::Right);

        for image in images {
            if !self.all.value && image.arch != Arch::get_host() {
                continue;
            }

            let size = image
                .size
                .map(|size| DataSize::new(size as usize).to_size())
                .unwrap_or_default();

            view.add_row()
                .add(&image.get_image_name(), Alignment::Left)
                .add(&image.arch.to_string(), Alignment::Left)
                .add(&size, Alignment::Right)
                .add(
                    if ImageStore::new().exists(context.get_system(), context.get_env(), &image) {
                        "yes"
                    } else {
                        "no"
                    },
                    Alignment::Right,
                );
        }
        view.print(console);
        Ok(())
    }
}
