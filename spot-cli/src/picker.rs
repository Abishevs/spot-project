use std::sync::Arc;
use std::thread;
use skim::prelude::*;
use std::io;

// impl display!!!
#[derive(Clone, Debug)]
pub struct Project {
    pub name: String,
    pub description: String,
}

impl SkimItem for Project {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.name)
    }

    fn preview(&self, _context: PreviewContext) -> ItemPreview {
        ItemPreview::Text(self.description.clone())
    }
}

pub fn show_picker_project(projects: Vec<Project>) -> io::Result<Option<Project>> {
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
        .map(|selected_item| (**selected_item).as_any().downcast_ref::<Project>().unwrap().to_owned())
        .collect::<Vec<Project>>();

    for item in selected_items {
        return Ok(Some(item));
    } 

    Ok(None)
}

