//! Pure-stdout native workflow catalog projection, derived from the public immutable registry.
use sc_core::command::CommandWorkflowId;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut out = io::stdout().lock();
    writeln!(
        out,
        "| Workflow | Steps | Commands | Classes | Permissions |"
    )?;
    writeln!(out, "| --- | --- | --- | --- | --- |")?;
    for workflow in CommandWorkflowId::ALL {
        let steps = workflow
            .steps()
            .iter()
            .map(|s| s.token())
            .collect::<Vec<_>>()
            .join(", ");
        let commands = workflow
            .commands()
            .iter()
            .map(|c| c.token())
            .collect::<Vec<_>>()
            .join(", ");
        let classes = workflow
            .command_classes()
            .iter()
            .map(|c| c.token())
            .collect::<Vec<_>>()
            .join(", ");
        let permissions = workflow
            .required_authorities()
            .iter()
            .map(|a| a.token())
            .collect::<Vec<_>>()
            .join(", ");
        writeln!(
            out,
            "| {} | {steps} | {commands} | {classes} | {permissions} |",
            workflow.token()
        )?;
    }
    Ok(())
}
