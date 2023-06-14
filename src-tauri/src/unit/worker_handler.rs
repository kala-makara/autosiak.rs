use std::time::Duration;

use scraper::{Html, Node, Selector};
use serde::Serialize;

use super::{
    constants::FEError::*,
    constants::{FEResult, COURSEPLANEDIT_URL},
    Session,
};

#[derive(Debug, Default, Serialize)]
pub struct Class {
    pub name: String,
    pub value: String,
    pub formal_name: String,
    pub term: u8,
}

#[derive(Debug, Default, Serialize)]
pub struct ClassList {
    pub group: String,
    pub classes: Vec<Class>,
}

pub type ScrapResult = FEResult<Vec<ClassList>>;

pub async fn scraper(state: &tauri::State<'_, Session>) -> ScrapResult {
    let mut classlist: Vec<ClassList> = Vec::with_capacity(4);

    let cpe_page = state
        .client
        .get(COURSEPLANEDIT_URL)
        .timeout(Duration::new(5, 0))
        .send()
        .await
        .map_err(|_| CourseEditInaccessible)
        .map(|x| async {
            let content = x.text().await.map_err(|_| CourseEditLoadError)?;

            let document = Html::parse_document(&content);
            let form_selector = Selector::parse("form[action='CoursePlanSave']").unwrap();

            let course_form = document
                .select(&form_selector)
                .next()
                .ok_or(CourseEditParseError)?;

            let class_selector = Selector::parse("h3, tr[id], input[name='tokens']").unwrap();
            let classes = course_form.select(&class_selector);

            for class in classes {
                let tagname = class.value().name();

                match tagname {
                    "h3" => {
                        let class_cat = class.inner_html();
                        classlist.push(ClassList {
                            group: class_cat,
                            classes: Vec::with_capacity(100),
                        });
                    }
                    "tr" => {
                        let mut res = Class::default();

                        let columns = class
                            .children()
                            .filter(|child| matches!(child.value(), Node::Element(_)))
                            .collect::<Vec<_>>();

                        if let Node::Element(v) = columns[0]
                            .children()
                            .find(|child| matches!(child.value(), Node::Element(_)))
                            .ok_or(CourseEditParseError)?
                            .value()
                        {
                            res.name = v.attr("name").ok_or(CourseEditParseError)?.to_string();
                            res.value = v.attr("value").ok_or(CourseEditParseError)?.to_string();
                        } else {
                            return Err(CourseEditParseError);
                        }

                        let class_name = columns[1]
                            .children()
                            .find(|child| matches!(child.value(), Node::Element(_)))
                            .ok_or(CourseEditParseError)?
                            .children()
                            // .filter(|child| matches!(child.value(), Node::Element(_)))
                            .collect::<Vec<_>>();

                        if let Node::Text(v) = class_name[1]
                            .first_child()
                            .ok_or(CourseEditParseError)?
                            .value()
                        {
                            res.formal_name = v.text.to_string();
                        }

                        if let Node::Text(v) = columns[5]
                            .first_child()
                            .ok_or(CourseEditParseError)?
                            .value()
                        {
                            res.term = v
                                .text
                                .to_string()
                                .parse::<u8>()
                                .map_err(|_| CourseEditParseError)?;
                        }

                        match classlist.last_mut() {
                            Some(v) => v.classes.push(res),
                            None => {
                                classlist.push(ClassList::default());
                                classlist.last_mut().unwrap().classes.push(res);
                            }
                        }
                    }
                    _ => (),
                };
            }

            ScrapResult::Ok(classlist)
        })?
        .await?;

    Ok(cpe_page)
}

#[tauri::command]
pub async fn worker(state: tauri::State<'_, Session>) -> ScrapResult {
    let classes = scraper(&state).await?;

    Ok(classes)
}
