use embassy_time::Duration;
use rmk::action::{EncoderAction, KeyAction};
use rmk::combo::Combo;
use rmk::config::macro_config::KeyboardMacrosConfig;
use rmk::config::{CombosConfig, MorsesConfig};
use rmk::heapless::Vec;
use rmk::keyboard_macros::define_macro_sequences;
use rmk::keycode::ModifierCombination;
use rmk::{a, encoder, k, mt, wm};

pub(crate) const COL: usize = 14;
pub(crate) const ROW: usize = 4;
pub(crate) const NUM_LAYER: usize = 11;
pub(crate) const NUM_ENCODER: usize = 2;

const MOD_G: ModifierCombination = ModifierCombination::new_from(false, true, false, false, false);
const MOD_A: ModifierCombination = ModifierCombination::new_from(false, false, true, false, false);
const MOD_S: ModifierCombination = ModifierCombination::new_from(false, false, false, true, false);
const MOD_C: ModifierCombination = ModifierCombination::new_from(false, false, false, false, true);

#[rustfmt::skip]
pub const fn get_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    const NO: [[KeyAction; COL]; ROW] = [
        [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
        [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
        [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
        [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
    ];

    const BASE: [[KeyAction; COL]; ROW] = [
         [k!(Tab)   , k!(Q)   , k!(W)   , k!(E)        , k!(R)    , k!(T)    , a!(No)   , a!(No)   , k!(Y)    , k!(U)    , k!(I)     , k!(O)             , k!(P)        , k!(Backspace),], 
         [k!(Escape), k!(A)   , k!(S)   , k!(D)        , k!(F)    , k!(G)    , a!(No)   , k!(Space), k!(H)    , k!(J)    , k!(K)     , k!(L)             , k!(Semicolon), k!(Quote)    ,], 
         [k!(LShift), k!(Z)   , k!(X)   , k!(C)        , k!(V)    , k!(B)    , k!(Space), a!(No)   , k!(N)    , k!(M)    , k!(Comma) , k!(Dot)           , k!(Slash)    , k!(Space)    ,], 
         [k!(LCtrl) , k!(LAlt), k!(LGui), k!(Backspace), k!(Space), k!(Enter), a!(No)   , a!(No)   , k!(Enter), k!(Space), k!(Delete), k!(CapsWordToggle), k!(Down)     , k!(Space)    ,], 
    ];

    [BASE, NO, NO, NO, NO, NO, NO, NO, NO, NO, NO,]
}

pub const fn get_encoder_map() -> [[EncoderAction; NUM_ENCODER]; NUM_LAYER] {
    [
        [
            encoder!(k!(Left), k!(Right)),
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
        ],
        [
            encoder!(k!(Left), k!(Right)),
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
        ],
        [
            encoder!(k!(Left), k!(Right)),
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
        ],
        [
            encoder!(k!(Left), k!(Right)),
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
        ],
        [
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
        ],
        [
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
        ],
        [
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
        ],
        [
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
        ],
        [
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
        ],
        [
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
        ],
        [
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
        ],
    ]
}

pub fn get_macros() -> KeyboardMacrosConfig {
    KeyboardMacrosConfig::new(define_macro_sequences(&[]))
}

pub fn get_combos() -> CombosConfig {
    CombosConfig {
        timeout: Duration::from_millis(100),
        combos: [
            Combo::new([k!(Q), mt!(A, MOD_G)], k!(Kc1), Some(0)),
            Combo::new([k!(W), mt!(S, MOD_A)], k!(Kc2), Some(0)),
            Combo::new([k!(E), mt!(D, MOD_C)], k!(Kc3), Some(0)),
            Combo::new([k!(R), mt!(F, MOD_S)], k!(Kc4), Some(0)),
            Combo::new([k!(T), k!(G)], k!(Kc5), Some(0)),
            Combo::new([k!(Y), k!(H)], k!(Kc6), Some(0)),
            Combo::new([k!(U), mt!(J, MOD_S)], k!(Kc7), Some(0)),
            Combo::new([k!(I), mt!(K, MOD_C)], k!(Kc8), Some(0)),
            Combo::new([k!(O), mt!(L, MOD_A)], k!(Kc9), Some(0)),
            Combo::new([k!(P), mt!(Quote, MOD_G)], k!(Kc0), Some(0)),
            Combo::new([mt!(A, MOD_G), k!(Z)], wm!(Kc1, MOD_S), Some(0)),
            Combo::new([mt!(S, MOD_A), k!(X)], wm!(Kc2, MOD_S), Some(0)),
            Combo::new([mt!(D, MOD_C), k!(C)], wm!(Kc3, MOD_S), Some(0)),
            Combo::new([mt!(F, MOD_S), k!(V)], wm!(Kc4, MOD_S), Some(0)),
            Combo::new([k!(G), k!(B)], wm!(Kc5, MOD_S), Some(0)),
            Combo::new([k!(H), k!(N)], wm!(Kc6, MOD_S), Some(0)),
            Combo::new([mt!(J, MOD_S), k!(M)], wm!(Kc7, MOD_S), Some(0)),
            Combo::new([mt!(K, MOD_C), k!(Comma)], wm!(Kc8, MOD_S), Some(0)),
            Combo::new([mt!(L, MOD_A), k!(Dot)], wm!(Kc9, MOD_S), Some(0)),
            Combo::new([mt!(Quote, MOD_G), k!(Slash)], wm!(Kc0, MOD_S), Some(0)),
        ]
        .into_iter()
        .collect(),
    }
}

pub fn get_morses() -> MorsesConfig {
    let morses = Vec::new();

    // let mut actions = Vec::new();
    // unwrap!(actions.push((
    //     MorsePattern::from_u16(0x10),
    //     Action::Key(KeyCode::Backspace),
    // )));
    // unwrap!(actions.push((MorsePattern::from_u16(0x11), Action::LayerOn(2),)));
    // unwrap!(morses.push(Morse {
    //     timeout_ms: 50u16,
    //     mode: MorseMode::HoldOnOtherPress,
    //     unilateral_tap: false,
    //     actions,
    // }));

    // let mut actions = Vec::new();
    // unwrap!(actions.push((MorsePattern::from_u16(0x11), Action::LayerOn(3),)));
    // unwrap!(actions.push((MorsePattern::from_u16(0x10), Action::Key(KeyCode::Space),)));
    // unwrap!(morses.push(Morse {
    //     timeout_ms: 50u16,
    //     mode: MorseMode::HoldOnOtherPress,
    //     unilateral_tap: false,
    //     actions,
    // }));
    // let mut actions = Vec::new();
    // unwrap!(actions.push((MorsePattern::from_u16(0x11), Action::LayerOn(4),)));
    // unwrap!(actions.push((MorsePattern::from_u16(0x10), Action::Key(KeyCode::Enter),)));
    // unwrap!(morses.push(Morse {
    //     timeout_ms: 50u16,
    //     mode: MorseMode::HoldOnOtherPress,
    //     unilateral_tap: false,
    //     actions,
    // }));

    MorsesConfig { morses }
}
