use clap::Args;
use dialoguer::{theme::ColorfulTheme, Input};

use crate::config::{self, save_conifg};

struct InitArgs {
    name: String,
    description: Option<String>,
}

impl InitArgs {
    fn new(
        name:String,
        description:Option<String>,
    ) -> Self {

        Self {
            name,
            description,
        }

    }
}

#[derive(Args)]
pub struct SubCommandArgs {
    /// repository name
    #[arg(short)]
    pub name: Option<String>,
    /// repository description
    #[arg(long)]
    description: Option<String>,
}


pub fn launch_init(
    command_args: &SubCommandArgs,
) {
    let args = get_init_args(command_args);

    let mut cfg = config::load_config();

    // call testable functions here
    println!(
        "{}, {}, default relays: {:?}",
        args.name,
        match args.description {
            None => "".to_string(),
            Some(description) => description,
        },
        cfg.default_relays,
    );

    cfg.default_relays[0] = "wss://localhost:80".to_string();
    save_conifg(&cfg);

}

fn get_init_args(command_args:&SubCommandArgs) -> InitArgs {

    let repo_name: String = match &command_args.name {
        Some(name) => name.clone(),
        None => {
            Input::with_theme(&ColorfulTheme::default())
                .with_prompt("repository name")
                .interact_text()
                .unwrap()
        },
    };

    let repo_description: String = match &command_args.description {
        Some(description) => description.clone(),
        None => {
            Input::with_theme(&ColorfulTheme::default())
                .with_prompt("repository description")
                .allow_empty(true)
                .interact_text()
                .unwrap()
        },
    };

    InitArgs::new(
        repo_name,
        Some(repo_description),
    )
    
}
