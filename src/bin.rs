use md_to_incodoc::parse_md_to_incodoc;
use incodoc::output::doc_out;

const INPUT: &str =
"
Key                     | Value
------------------------|-----------------------------------
Os                      | Artix
Init system             | OpenRC
Display server          | X11
Display init            | sx (https://github.com/Earnestly/sx)
WM (phase in)           | Ringwm (https://github.com/codybloemhard/ringwm)
WM (phase out)          | i3-gaps
Shell (bin/sh)          | Dash
Shell (Login)           | Fish
Terminal                | St (https://github.com/codybloemhard/st-cody)
Launcher                | Dmenu (https://github.com/codybloemhard/dmenu-cody)
Status bar              | Shapebar (https://gitlab.com/codybloemhard/shapebar)
Prompt                  | Starship (https://github.com/starship/starship)
Text editor             | NeoVim
Compositor              | Picom
AUR helper              | Paru
Keymap                  | Physical: QMK (https://github.com/codybloemhard/qmk-cody)
Notable Utils           | Scrot, Slock, Feh, Fzf, Ag, Bat, Eza, Dust, Btop, Paclog, Reat
Themes                  | Nord, Gruvbox, Hawkrad, Tokyo Night, Space, Dark

- A | B
  --|--
  0 | 1
  1 | 2
- C | D
  --|--
  0 | 1
  1 | 2
";

fn main() {
    let doc = parse_md_to_incodoc(INPUT);
    // println!("{:#?}", doc);
    let mut output = String::new();
    doc_out(&doc, &mut output);
    println!("{output}");
}

