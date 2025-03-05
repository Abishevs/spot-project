use skim::prelude::*;
use std::io;
use std::sync::Arc;
use std::thread;
// use spot_lib::models::{ProjectTag, Session, Tag};
use spot_lib::models::Project as LibProject;
use spot_lib::utils::format_duration;

#[derive(Clone, Debug)]
pub struct ProjectAdapter {
    pub project: LibProject,
}

impl ProjectAdapter {
    pub fn from_project(project: LibProject) -> Self {
        ProjectAdapter { project }
    }
}

impl SkimItem for ProjectAdapter {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.project.name)
    }

    fn preview(&self, _context: PreviewContext) -> ItemPreview {
        let name = self.project.name.clone();
        let desc = self.project.description.clone().unwrap_or_default();
        let cum_time = self.project.cumulative_time.clone();

        let formated_time = format_duration(cum_time);
        ItemPreview::Text(format!(
            "Project\nName: {}\nTotal Time spent: {}\nDescription: {}",
            name, formated_time, desc
        ))
    }
}

pub fn show_picker_project(projects: Vec<ProjectAdapter>) -> io::Result<Option<ProjectAdapter>> {
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .preview(Some(""))
        .multi(false)
        .build()
        .unwrap();

    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

    thread::spawn(move || {
        for project in projects {
            tx.send(Arc::new(project)).expect("Failed to send project");
        }
        drop(tx);
    });

    let selected_items = Skim::run_with(&options, Some(rx))
        .map(|out| out.selected_items)
        .unwrap_or_else(Vec::new)
        .iter()
        .map(|selected_item| {
            (**selected_item)
                .as_any()
                .downcast_ref::<ProjectAdapter>()
                .unwrap()
                .to_owned()
        })
        .collect::<Vec<ProjectAdapter>>();

    for item in selected_items {
        return Ok(Some(item));
    }

    Ok(None)
}
