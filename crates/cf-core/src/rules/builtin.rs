use std::path::PathBuf;

use uuid::Uuid;

use crate::models::{
    ConditionLogic, FileCategory, Rule, RuleAction, RuleCondition, RuleTrigger,
};

pub fn builtin_rules() -> Vec<Rule> {
    vec![
        rule(
            "Screenshots",
            100,
            vec![RuleCondition::NameMatches("screenshot*".into())],
            ConditionLogic::Any,
            vec![RuleAction::MoveTo(PathBuf::from("~/Pictures/Screenshots/{year}/{month}"))],
        ),
        rule(
            "macOS Screenshots (System)",
            100,
            vec![RuleCondition::NameMatches("Screen Shot*".into())],
            ConditionLogic::Any,
            vec![RuleAction::MoveTo(PathBuf::from("~/Pictures/Screenshots"))],
        ),
        rule(
            "DMG Installers",
            80,
            vec![RuleCondition::Extension("dmg".into())],
            ConditionLogic::All,
            vec![RuleAction::Trash],
        ),
        rule(
            ".DS_Store & System Junk",
            90,
            vec![
                RuleCondition::NameContains(".ds_store".into()),
                RuleCondition::NameContains("thumbs.db".into()),
                RuleCondition::NameContains("desktop.ini".into()),
            ],
            ConditionLogic::Any,
            vec![RuleAction::Trash],
        ),
        rule(
            "Temp & Cache Files",
            70,
            vec![
                RuleCondition::Extension("tmp".into()),
                RuleCondition::Extension("cache".into()),
                RuleCondition::Extension("log".into()),
            ],
            ConditionLogic::Any,
            vec![RuleAction::Trash],
        ),
        rule(
            "PDF Invoices",
            60,
            vec![
                RuleCondition::MimeType("application/pdf".into()),
                RuleCondition::AiCategory(FileCategory::Invoice),
            ],
            ConditionLogic::All,
            vec![
                RuleAction::MoveTo(PathBuf::from("~/Documents/Finance/{year}")),
                RuleAction::AddTag("invoice".into()),
            ],
        ),
        rule(
            "Archive Old Downloads",
            30,
            vec![
                RuleCondition::PathContains("Downloads".into()),
                RuleCondition::OlderThanDays(90),
            ],
            ConditionLogic::All,
            vec![RuleAction::Archive(PathBuf::from("~/Documents/Archive"))],
        ),
        rule(
            "Zombie Files",
            20,
            vec![RuleCondition::NeverAccessed],
            ConditionLogic::All,
            vec![RuleAction::Archive(PathBuf::from("~/Documents/Archive/Unused"))],
        ),
    ]
}

fn rule(
    name: &str,
    priority: i32,
    conditions: Vec<RuleCondition>,
    logic: ConditionLogic,
    actions: Vec<RuleAction>,
) -> Rule {
    Rule {
        id: Uuid::new_v4().to_string(),
        name: name.to_string(),
        enabled: true,
        priority,
        trigger: RuleTrigger::OnScan,
        conditions,
        condition_logic: logic,
        actions,
        schedule: None,
    }
}
