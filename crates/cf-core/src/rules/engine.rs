use std::time::{SystemTime, UNIX_EPOCH};

use regex::Regex;
use uuid::Uuid;

use crate::models::{
    Action, ActionReason, ConditionLogic, FileCategory, PlannedAction,
    Rule, RuleAction, RuleCondition, ScannedFile,
};

pub struct RuleEngine {
    rules: Vec<Rule>,
}

impl RuleEngine {
    pub fn new(rules: Vec<Rule>) -> Self {
        let mut sorted = rules;
        sorted.sort_by(|a, b| b.priority.cmp(&a.priority));
        Self { rules: sorted }
    }

    pub fn apply(&self, file: &ScannedFile) -> Vec<PlannedAction> {
        let mut actions = vec![];

        for rule in self.rules.iter().filter(|r| r.enabled) {
            if self.matches(rule, file) {
                let rule_actions = self.build_actions(rule, file);
                for action in rule_actions {
                    actions.push(PlannedAction {
                        id: Uuid::new_v4().to_string(),
                        action,
                        reason: ActionReason::RuleMatch {
                            rule_id: rule.id.clone(),
                            rule_name: rule.name.clone(),
                        },
                        priority: rule.priority.clamp(0, 255) as u8,
                        selected: true,
                    });
                }
            }
        }

        actions
    }

    fn matches(&self, rule: &Rule, file: &ScannedFile) -> bool {
        let results: Vec<bool> = rule
            .conditions
            .iter()
            .map(|c| check_condition(c, file))
            .collect();

        match rule.condition_logic {
            ConditionLogic::All => results.iter().all(|&r| r),
            ConditionLogic::Any => results.iter().any(|&r| r),
        }
    }

    fn build_actions(&self, rule: &Rule, file: &ScannedFile) -> Vec<Action> {
        rule.actions
            .iter()
            .filter_map(|ra| rule_action_to_action(ra, file))
            .collect()
    }
}

fn check_condition(cond: &RuleCondition, file: &ScannedFile) -> bool {
    let name = file
        .path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_lowercase();
    let ext = file
        .path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match cond {
        RuleCondition::MimeType(mime) => file.mime_type.contains(mime.as_str()),
        RuleCondition::Extension(e)   => ext == e.trim_start_matches('.').to_lowercase(),
        RuleCondition::NameContains(s)=> name.contains(s.to_lowercase().as_str()),
        RuleCondition::NameMatches(pattern) => {
            glob::Pattern::new(pattern)
                .map(|p| p.matches(&name))
                .unwrap_or(false)
        }
        RuleCondition::SizeGt(n)      => file.size_bytes > *n,
        RuleCondition::SizeLt(n)      => file.size_bytes < *n,
        RuleCondition::OlderThanDays(days) => {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs() as i64)
                .unwrap_or(0);
            now - file.modified_at > (*days as i64) * 86400
        }
        RuleCondition::AiCategory(cat) => file
            .ai_classification
            .as_ref()
            .map(|c| &c.category == cat)
            .unwrap_or(false),
        RuleCondition::AiTagContains(tag) => file
            .ai_classification
            .as_ref()
            .map(|c| c.tags.iter().any(|t| t.contains(tag.as_str())))
            .unwrap_or(false),
        RuleCondition::PathContains(s) => file
            .path
            .to_string_lossy()
            .to_lowercase()
            .contains(s.to_lowercase().as_str()),
        RuleCondition::NeverAccessed  => file.flags.is_zombie,
    }
}

fn rule_action_to_action(ra: &RuleAction, file: &ScannedFile) -> Option<Action> {
    match ra {
        RuleAction::MoveTo(dest) => {
            let filename = file.path.file_name()?;
            Some(Action::Move {
                file_id: file.id.clone(),
                from: file.path.clone(),
                to: dest.join(filename),
            })
        }
        RuleAction::RenamePattern(pattern) => {
            let new_name = expand_pattern(pattern, file);
            Some(Action::Rename {
                file_id: file.id.clone(),
                path: file.path.clone(),
                new_name,
            })
        }
        RuleAction::AddTag(tag) => Some(Action::Tag {
            file_id: file.id.clone(),
            tags: vec![tag.clone()],
        }),
        RuleAction::Trash => Some(Action::Trash {
            file_id: file.id.clone(),
            path: file.path.clone(),
        }),
        RuleAction::Archive(dest) => {
            let filename = file.path.file_name()?;
            Some(Action::Move {
                file_id: file.id.clone(),
                from: file.path.clone(),
                to: dest.join(filename),
            })
        }
    }
}

fn expand_pattern(pattern: &str, file: &ScannedFile) -> String {
    use chrono::TimeZone;

    let dt = chrono::Utc.timestamp_opt(file.modified_at, 0)
        .single()
        .unwrap_or_else(chrono::Utc::now);

    let name = file
        .path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy();

    let category = file
        .ai_classification
        .as_ref()
        .map(|c| c.category.to_string())
        .unwrap_or_else(|| "Misc".into());

    pattern
        .replace("{year}",     &dt.format("%Y").to_string())
        .replace("{month}",    &dt.format("%m").to_string())
        .replace("{day}",      &dt.format("%d").to_string())
        .replace("{name}",     &name)
        .replace("{category}", &category)
}
