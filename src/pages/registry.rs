use crate::domains::structural::template_ids::GENERIC;
use crate::pages::analysis::template::AnalysisTemplate;
use crate::pages::generic::template::GenericTemplate;
use crate::pages::navigation::template::NavigationTemplate;
use crate::pages::results::template::ResultsTemplate;
use crate::pages::template::PageTemplate;
use hyper_ui::TemplateId;
use std::collections::HashMap;

pub fn page_templates() -> HashMap<TemplateId, Box<dyn PageTemplate>> {
    let templates: Vec<Box<dyn PageTemplate>> = vec![
        Box::new(NavigationTemplate),
        Box::new(AnalysisTemplate),
        Box::new(ResultsTemplate),
        Box::new(GenericTemplate),
    ];
    templates.into_iter().map(|template| (template.id(), template)).collect()
}

pub fn template_for(
    templates: &HashMap<TemplateId, Box<dyn PageTemplate>>,
    id: TemplateId,
) -> &dyn PageTemplate {
    templates
        .get(&id)
        .or_else(|| templates.get(&GENERIC))
        .map(Box::as_ref)
        .expect("generic page template must be registered")
}
