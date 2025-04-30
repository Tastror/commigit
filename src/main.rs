use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([350.0, 500.0])
            .with_title("Git Commit Message Generator"),
        ..Default::default()
    };

    eframe::run_native(
        "Git Commit Message Generator",
        options,
        Box::new(|_cc| Ok(Box::<CommitMessageGenerator>::default())),
    )
}

#[derive(Default)]
struct CommitMessageGenerator {
    commit_type: String,
    scope: String,
    subject: String,
    body: String,
    use_breaking: bool,
    breaking_change: String,
    use_deprecations: bool,
    deprecations: String,
    refs: String,
    copied_time: Option<f64>,
}

impl eframe::App for CommitMessageGenerator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Git Commit Message Generator");
            ui.separator();

            // Commit Type + Scope
            ui.horizontal(|ui| {
                // Type dropdown
                egui::ComboBox::from_label("Type")
                    .selected_text(if self.commit_type.is_empty() {
                        "Select type".to_owned()
                    } else {
                        // format!("{} - {}", self.commit_type, get_type_description(&self.commit_type))
                        format!("{}", self.commit_type)
                    })
                    .width(100.0)
                    .show_ui(ui, |ui| {
                        for (typ, desc) in get_commit_types() {
                            ui.selectable_value(
                                &mut self.commit_type,
                                typ.to_string(),
                                format!("{} - {}", typ, desc),
                            );
                        }
                    });

                // Optional scope
                ui.label("(");
                let scope = egui::TextEdit::singleline(&mut self.scope)
                    .hint_text("scope")
                    .desired_width(120.0);
                ui.add(scope);
                ui.label(")");
            });

            // Subject
            ui.add_space(5.0);
            ui.label("Subject");
            let subject = egui::TextEdit::singleline(&mut self.subject)
                .hint_text("e.g. add login feature");
            ui.add(subject);

            // Body
            ui.add_space(10.0);
            ui.label("Body");
            let body = egui::TextEdit::multiline(&mut self.body)
                .hint_text("e.g.\n- Detailed description\n- Use bullet points\n- Explain WHY if needed");
            ui.add(body);

            // Breaking change
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.checkbox(
                    &mut self.use_breaking,
                    "Contains BREAKING CHANGE",
                );
                if self.use_breaking {
                    let breaking_change = egui::TextEdit::singleline(&mut self.breaking_change)
                        .hint_text("Describe breaking changes");
                    ui.add(breaking_change);
                }
            });

            // deprecations
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.checkbox(
                    &mut self.use_deprecations,
                    "Contains DEPRECATION",
                );
                if self.use_deprecations {
                    let deprecations = egui::TextEdit::singleline(&mut self.deprecations)
                        .hint_text("Describe deprecations");
                    ui.add(deprecations);
                }
            });

            // References
            ui.add_space(10.0);
            ui.label("References");
            let refs = egui::TextEdit::singleline(&mut self.refs)
                .hint_text("Issue/Ticket IDs (e.g. #123, JIRA-456)");
            ui.add(refs);

            // Generate output
            ui.add_space(20.0);
            let command = self.generate_command();
            ui.label("Generated Command");
            let response = ui.add(egui::Label::new(&command).sense(egui::Sense::click()));
            if response.clicked() {
                ctx.copy_text(command.clone());
                self.copied_time = Some(ctx.input(|i| i.time));
            }

            // Auto-dismiss after 2 seconds
            if let Some(copied_time) = self.copied_time {
                let current_time = ctx.input(|i| i.time);
                if current_time - copied_time < 1.0 {
                    ui.colored_label(egui::Color32::GREEN, "✓ Copied to clipboard!");
                } else {
                    self.copied_time = None;
                }
            }
        });
    }
}

impl CommitMessageGenerator {
    fn generate_command(&self) -> String {
        let mut msg = String::new();

        // Type(Scope): Subject
        msg.push_str(&self.commit_type);
        if !self.scope.is_empty() {
            msg.push('(');
            msg.push_str(&self.scope);
            msg.push(')');
        }
        msg.push_str(": ");
        msg.push_str(&self.subject);

        // Body
        if !self.body.is_empty() {
            msg.push_str("\n\n");
            msg.push_str(&self.body);
        }

        if (self.use_breaking && !self.breaking_change.is_empty()) ||
            (self.use_deprecations && !self.deprecations.is_empty()) ||
            !self.refs.is_empty() {
            msg.push_str("\n");
        }

        // Breaking change
        if self.use_breaking && !self.breaking_change.is_empty() {
            msg.push_str("\nBREAKING CHANGE: ");
            msg.push_str(&self.breaking_change);
        }

        // Deprecation
        if self.use_deprecations && !self.deprecations.is_empty() {
            msg.push_str("\nDEPRECATION: ");
            msg.push_str(&self.deprecations);
        }

        // References
        if !self.refs.is_empty() {
            msg.push_str("\nRefs: ");
            msg.push_str(&self.refs);
        }

        if !self.commit_type.is_empty() && !self.subject.is_empty() {
            return format!("git commit -m \"{}\"", msg.replace('"', "\\\""));
        } else {
            return "Please select a type and enter the subject".to_owned();
        };

    }
}

fn get_commit_types() -> Vec<(&'static str, &'static str)> {
    vec![
        ("feat", "A new feature"),
        ("fix", "A bug fix"),
        ("docs", "Documentation changes"),
        ("style", "Code style changes (no functional changes)"),
        ("refactor", "Code refactoring (no functional changes)"),
        ("perf", "Performance improvements"),
        ("test", "Adding or modifying tests"),
        ("chore", "Build process or auxiliary tool changes"),
    ]
}

// fn get_type_description(typ: &str) -> &'static str {
//     get_commit_types()
//         .iter()
//         .find(|(t, _)| *t == typ)
//         .map(|(_, d)| *d)
//         .unwrap_or("Select a commit type")
// }