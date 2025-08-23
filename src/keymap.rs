use embassy_time::Duration;
use rmk::action::{EncoderAction, KeyAction};
use rmk::combo::Combo;
use rmk::config::CombosConfig;
use rmk::config::macro_config::KeyboardMacrosConfig;
use rmk::keyboard_macros::{MacroOperation, define_macro_sequences};
use rmk::keycode::{KeyCode, ModifierCombination};
use rmk::{a, encoder, k, lt, mt, wm};

pub(crate) const COL: usize = 14;
pub(crate) const ROW: usize = 4;
pub(crate) const NUM_LAYER: usize = 11;
pub(crate) const NUM_ENCODER: usize = 2;

const MOD_G: ModifierCombination = ModifierCombination::new_from(false, true, false, false, false);
const MOD_A: ModifierCombination = ModifierCombination::new_from(false, false, true, false, false);
const MOD_S: ModifierCombination = ModifierCombination::new_from(false, false, false, true, false);
const MOD_C: ModifierCombination = ModifierCombination::new_from(false, false, false, false, true);
const MOD_GA: ModifierCombination = ModifierCombination::new_from(false, true, true, false, false);
const MOD_GS: ModifierCombination = ModifierCombination::new_from(false, true, false, true, false);
const MOD_GC: ModifierCombination = ModifierCombination::new_from(false, true, false, false, true);
const MOD_AS: ModifierCombination = ModifierCombination::new_from(false, false, true, true, false);
const MOD_ASC: ModifierCombination = ModifierCombination::new_from(false, false, true, true, true);
const MOD_AC: ModifierCombination = ModifierCombination::new_from(false, false, true, false, true);

#[allow(dead_code)]
const LAYER_MEDIA: u8 = 1;
#[allow(dead_code)]
const LAYER_NAV: u8 = 2;
#[allow(dead_code)]
const LAYER_MOUSE: u8 = 3;
#[allow(dead_code)]
const LAYER_SYMBOL: u8 = 4;
#[allow(dead_code)]
const LAYER_NUMBER: u8 = 5;
#[allow(dead_code)]
const LAYER_FUNCTION: u8 = 6;

const CLPBRD: KeyAction = wm!(V, MOD_GS);
const OPWD: KeyAction = wm!(Slash, MOD_GA);
const SNPST_PIC: KeyAction = wm!(U, MOD_ASC);
const SNPST_VID: KeyAction = wm!(O, MOD_ASC);
const SNPST_REG: KeyAction = wm!(I, MOD_ASC);

const LMDA: KeyAction = lt!(1, Backspace);
const LNAV: KeyAction = lt!(2, Space);
const LMSE: KeyAction = lt!(3, Enter);
const LSYM: KeyAction = lt!(4, Enter);
const LNUM: KeyAction = lt!(5, Space);
const LFUN: KeyAction = lt!(6, Delete);

const AR1: KeyAction = wm!(Kc1, MOD_AC);
const AR2: KeyAction = wm!(Kc2, MOD_AC);
const AR3: KeyAction = wm!(Kc3, MOD_AC);
const AR4: KeyAction = wm!(Kc4, MOD_AC);
const AR5: KeyAction = wm!(Kc5, MOD_AC);
const AR6: KeyAction = wm!(Kc6, MOD_AC);
const AR_LW: KeyAction = wm!(H, MOD_AC);
const AR_RW: KeyAction = wm!(L, MOD_AC);
const AR_FSCR: KeyAction = wm!(Space, MOD_AS);
const AR_ACC: KeyAction = wm!(Slash, MOD_AC);
const AR_TIL: KeyAction = wm!(Comma, MOD_AC);

const MAC_SLP: KeyAction = wm!(Q, MOD_GC);

const ED_PREV: KeyAction = mt!(O, MOD_C);
const ED_NEXT: KeyAction = mt!(I, MOD_C);

const LG1: KeyAction = wm!(Kc1, MOD_G);
const LG2: KeyAction = wm!(Kc2, MOD_G);
const LG3: KeyAction = wm!(Kc3, MOD_G);
const LG4: KeyAction = wm!(Kc4, MOD_G);
const LG5: KeyAction = wm!(Kc5, MOD_G);

const PD_SPC: KeyAction = wm!(Space, MOD_ASC);
const PD_Q: KeyAction = wm!(Q, MOD_ASC);
const PD_W: KeyAction = wm!(W, MOD_ASC);
const PD_E: KeyAction = wm!(E, MOD_ASC);
const PD_R: KeyAction = wm!(R, MOD_ASC);
const PD_T: KeyAction = wm!(T, MOD_ASC);
const PD_A: KeyAction = wm!(A, MOD_ASC);
const PD_S: KeyAction = wm!(S, MOD_ASC);
const PD_D: KeyAction = wm!(D, MOD_ASC);
const PD_F: KeyAction = wm!(F, MOD_ASC);
const PD_G: KeyAction = wm!(G, MOD_ASC);
const WORD_BACKSPACE: KeyAction = wm!(Backspace, MOD_A);
const WORD_LEFT: KeyAction = wm!(Left, MOD_A);
const WORD_RIGHT: KeyAction = wm!(Right, MOD_A);

#[macro_export]
macro_rules! macro_ {
    ($n: expr) => {
        rmk::action::KeyAction::Single(rmk::action::Action::TriggerMacro($n))
    };
}

macro_rules! hrm {
    ($k: ident @ $m: expr) => {
        mt!($k, $m)
    };
}
#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        [
            [k!(Tab)   , k!(Q)          , k!(W)          , k!(E)          , k!(R)          , k!(T), a!(No), a!(No), k!(Y), k!(U)          , k!(I)          , k!(O)          , k!(P)              , a!(No)]       , 
            [k!(Escape), hrm!(A @ MOD_G), hrm!(S @ MOD_A), hrm!(D @ MOD_C), hrm!(F @ MOD_S), k!(G), a!(No), OPWD  , k!(H), hrm!(J @ MOD_S), hrm!(K @ MOD_C), hrm!(L @ MOD_A), hrm!(Quote @ MOD_G), k!(Semicolon)], 
            [k!(Up)    , k!(Z)          , k!(X)          , k!(C)          , k!(V)          , k!(B), CLPBRD, a!(No), k!(N), k!(M)          , k!(Comma)      , k!(Dot)        , k!(Slash)          , SNPST_PIC]    , 
            [k!(Down)  , k!(Left)       , k!(Right)      , LMDA           , LNAV           , LMSE , a!(No), a!(No), LSYM , LNUM           , LFUN           , k!(Left)       , k!(Down)           , k!(Right)]    , 
        ],
        // MEDIA LAYER
        [
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)       , SNPST_PIC         , SNPST_REG     , SNPST_VID , a!(No), WORD_BACKSPACE],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), WORD_LEFT    , k!(KbVolumeDown)  , k!(KbVolumeUp), WORD_RIGHT, a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)       , a!(No)            , a!(No)        , a!(No)    , a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), k!(MediaStop), k!(MediaPlayPause), k!(AudioMute) , a!(No)    , a!(No), a!(No)],
        ],

        // NAV LAYER
        [
            [PD_SPC , AR1       , AR2       , AR3       , AR4       , AR5       , a!(No), a!(No), AR6     , AR_LW       , AR_RW     , a!(No)   , a!(No)    , a!(No)], 
            [AR_FSCR, macro_!(0), macro_!(1), macro_!(2), macro_!(3), macro_!(4), a!(No), a!(No), k!(Left), k!(Down)    , k!(Up)    , k!(Right), macro_!(7), a!(No)], 
            [a!(No) , LG1       , LG2       , LG3       , LG4       , LG5       , a!(No), a!(No), k!(Home), k!(PageDown), k!(PageUp), k!(End)  , macro_!(6), a!(No)], 
            [a!(No) , a!(No)    , a!(No)    , a!(No)    , a!(No)    , a!(No)    , a!(No), a!(No), ED_PREV , ED_NEXT     , a!(No)    , a!(No)   , a!(No)    , a!(No)], 
        ],
        // MOUSE LAYER
        [
            [PD_SPC, PD_Q  , PD_W  , PD_E  , PD_R  , PD_T  , a!(No), a!(No), a!(No)        , a!(No)       ,a!(No)        ,a!(No)         , a!(No), WORD_BACKSPACE], 
            [a!(No), PD_A  , PD_S  , PD_D  , PD_F  , PD_G  , a!(No), a!(No), k!(MouseLeft) , k!(MouseDown), k!(MouseUp)  , k!(MouseRight), a!(No), a!(No)        ], 
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)        , a!(No)       , a!(No)       , a!(No)        , a!(No), a!(No)        ], 
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), WORD_BACKSPACE, k!(MouseBtn1), k!(MouseBtn2), a!(No)        , a!(No), a!(No)        ], 
        ],
        // SYM LAYER
        [
            [a!(No), wm!(LeftBracket, MOD_S), wm!(Kc1, MOD_S), wm!(Kc2, MOD_S), wm!(Kc3, MOD_S), wm!(RightBracket, MOD_S), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)], 
            [a!(No), wm!(Semicolon, MOD_S)  , wm!(Kc4, MOD_S), wm!(Kc5, MOD_S), wm!(Kc6, MOD_S), wm!(Equal, MOD_S)       , a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)], 
            [a!(No), wm!(Grave, MOD_S)      , wm!(Kc7, MOD_S), wm!(Kc8, MOD_S), wm!(Kc9, MOD_S), wm!(Backslash, MOD_S)   , a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)], 
            [a!(No), a!(No)                 , a!(No)         , wm!(Kc9, MOD_S), wm!(Kc0, MOD_S), wm!(Minus, MOD_S)       , a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)], 
        ],
        // NUM LAYER
        [
            [a!(No), k!(LeftBracket), k!(Kc1), k!(Kc2), k!(Kc3), k!(RightBracket), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)], 
            [a!(No), k!(Semicolon)  , k!(Kc4), k!(Kc5), k!(Kc6), k!(Equal)       , a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)], 
            [a!(No), k!(Grave)      , k!(Kc7), k!(Kc8), k!(Kc9), k!(Backslash)   , a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)], 
            [a!(No), a!(No)         , a!(No) , k!(Dot), k!(Kc0), k!(Minus)       , a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)], 
        ],
        // FN LAYER
        [
            [k!(User5), k!(F10), k!(F1), k!(F2), k!(F3), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), MAC_SLP],
            [k!(User0), k!(F11), k!(F4), k!(F5), k!(F6), AR_ACC, a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [k!(User1), k!(F12), k!(F7), k!(F8), k!(F9), AR_TIL, a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [k!(User2), a!(No) , a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
        ],[
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
        ],[
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
        ],[
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
        ],[
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
        ],
    ]
}

pub const fn get_default_encoder_map() -> [[EncoderAction; NUM_ENCODER]; NUM_LAYER] {
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
    KeyboardMacrosConfig::new(define_macro_sequences(&[
        [
            MacroOperation::Press(KeyCode::LAlt),
            MacroOperation::Delay(30),
            MacroOperation::Press(KeyCode::M),
            MacroOperation::Delay(30),
            MacroOperation::Release(KeyCode::M),
            MacroOperation::Release(KeyCode::LAlt),
            MacroOperation::Delay(30),
            MacroOperation::Tap(KeyCode::Kc1),
        ]
        .into_iter()
        .collect(),
        [
            MacroOperation::Press(KeyCode::M),
            MacroOperation::Delay(30),
            MacroOperation::Press(KeyCode::LAlt),
            MacroOperation::Delay(30),
            MacroOperation::Release(KeyCode::M),
            MacroOperation::Release(KeyCode::LAlt),
            MacroOperation::Delay(30),
            MacroOperation::Tap(KeyCode::Kc2),
        ]
        .into_iter()
        .collect(),
        [
            MacroOperation::Press(KeyCode::M),
            MacroOperation::Press(KeyCode::LAlt),
            MacroOperation::Delay(30),
            MacroOperation::Release(KeyCode::M),
            MacroOperation::Release(KeyCode::LAlt),
            MacroOperation::Delay(30),
            MacroOperation::Tap(KeyCode::Kc3),
        ]
        .into_iter()
        .collect(),
        [
            MacroOperation::Press(KeyCode::M),
            MacroOperation::Press(KeyCode::LAlt),
            MacroOperation::Delay(30),
            MacroOperation::Release(KeyCode::M),
            MacroOperation::Release(KeyCode::LAlt),
            MacroOperation::Delay(30),
            MacroOperation::Tap(KeyCode::Kc4),
        ]
        .into_iter()
        .collect(),
        [
            MacroOperation::Press(KeyCode::M),
            MacroOperation::Press(KeyCode::LAlt),
            MacroOperation::Delay(30),
            MacroOperation::Release(KeyCode::M),
            MacroOperation::Release(KeyCode::LAlt),
            MacroOperation::Delay(30),
            MacroOperation::Tap(KeyCode::Kc5),
        ]
        .into_iter()
        .collect(),
        [
            MacroOperation::Press(KeyCode::M),
            MacroOperation::Press(KeyCode::LAlt),
            MacroOperation::Delay(30),
            MacroOperation::Release(KeyCode::M),
            MacroOperation::Release(KeyCode::LAlt),
            MacroOperation::Delay(30),
            MacroOperation::Tap(KeyCode::Kc6),
        ]
        .into_iter()
        .collect(),
        [
            MacroOperation::Press(KeyCode::M),
            MacroOperation::Press(KeyCode::LAlt),
            MacroOperation::Delay(30),
            MacroOperation::Release(KeyCode::M),
            MacroOperation::Release(KeyCode::LAlt),
            MacroOperation::Delay(30),
            MacroOperation::Tap(KeyCode::L),
        ]
        .into_iter()
        .collect(),
        [
            MacroOperation::Press(KeyCode::M),
            MacroOperation::Press(KeyCode::LAlt),
            MacroOperation::Delay(30),
            MacroOperation::Release(KeyCode::M),
            MacroOperation::Release(KeyCode::LAlt),
            MacroOperation::Delay(30),
            MacroOperation::Tap(KeyCode::LeftBracket),
        ]
        .into_iter()
        .collect(),
    ]))
}

pub fn get_combos() -> CombosConfig {
    CombosConfig {
        timeout: Duration::from_millis(100),
        combos: [
            Combo::new([k!(Z), k!(X)], wm!(Z, MOD_G), Some(0)),
            Combo::new([k!(X), k!(C)], wm!(X, MOD_G), Some(0)),
            Combo::new([k!(C), k!(V)], wm!(C, MOD_G), Some(0)),
            Combo::new([k!(V), k!(B)], wm!(V, MOD_G), Some(0)),
            Combo::new([mt!(F, MOD_S), k!(G)], wm!(F, MOD_G), Some(0)),
            Combo::new([mt!(A, MOD_G), mt!(S, MOD_A)], wm!(A, MOD_G), Some(0)),
        ]
        .into_iter()
        .collect(),
    }
}
