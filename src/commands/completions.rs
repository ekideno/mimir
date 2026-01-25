#[derive(Clone, Copy, Debug, clap::ValueEnum)]
pub enum Shell {
    Zsh,
    Bash,
    Fish,
}

pub fn handle(shell: Shell) {
    match shell {
        Shell::Zsh => {
            print!("{}", include_str!("../../completions/_mimir"));
        }
        Shell::Bash => {
            print!("{}", include_str!("../../completions/mimir.bash"));
        }
        Shell::Fish => {
            print!("{}", include_str!("../../completions/mimir.fish"));
        }
    }
}
