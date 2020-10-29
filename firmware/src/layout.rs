use keyberon::key_code::KeyCode::*;
use keyberon::action::{k, l, m, Action, Action::*};

macro_rules! s {
    ($k:ident) => {
        m(&[LShift, $k])
    };
}

const LCTL_ESC: Action = HoldTap {
    timeout: 200,
    hold: &k(LCtrl),
    tap: &k(Escape),
};

const RCTL_QUOT: Action = HoldTap {
    timeout: 200,
    hold: &k(RCtrl),
    tap: &k(Quote),
};

const LCTL_A: Action = HoldTap {
    timeout: 200,
    hold: &k(LCtrl),
    tap: &k(A),
};

const LSFT_Z: Action = HoldTap {
    timeout: 200,
    hold: &k(LShift),
    tap: &k(Z),
};

const RCTL_SCOL: Action = HoldTap {
    timeout: 300,
    hold: &k(RCtrl),
    tap: &k(SColon),
};

const RSFT_SLSH: Action = HoldTap {
    timeout: 200,
    hold: &k(RShift),
    tap: &k(Slash),
};

const NEXT_TAB: Action = m(&[LCtrl, PgDown]);
const PREV_TAB: Action = m(&[LCtrl, PgUp]);

#[rustfmt::skip]
pub static LAYERS: keyberon::layout::Layers = &[
    &[
        &[k(Tab),     k(Q),  k(W),   k(E),   k(R), k(T),    k(Y),    k(U), k(I),    k(O),    k(P),     k(BSpace)],
        &[LCTL_ESC,   LCTL_A,k(S),   k(D),   k(F), k(G),    k(H),    k(J), k(K),    k(L),    RCTL_SCOL,RCTL_QUOT],
        &[k(LShift),  LSFT_Z,k(X),   k(C),   k(V), k(B),    k(N),    k(M), k(Comma),k(Dot),  RSFT_SLSH, k(RShift)],
        &[Trans,      Trans, k(LGui),k(LAlt),l(1), k(Space),k(Enter),l(2), k(RAlt), k(BSpace),Trans,    Trans    ],
    ], &[
        &[s!(Grave), s!(Kb1),     s!(Kb2),     s!(Kb3),    s!(Kb4),    s!(Kb5),    s!(Kb6),   s!(Kb7),  s!(Kb8),  s!(Kb9), s!(Kb0), k(Delete)],
        &[Trans,     s!(LBracket),s!(RBracket),s!(Kb9),    s!(Kb0),    s!(Bslash), s!(Equal), k(Minus), s!(Minus),k(Equal),PREV_TAB,PREV_TAB ],
        &[Trans,     Trans,       Trans,       k(LBracket),k(RBracket),k(Bslash),  s!(SColon),k(Quote), s!(Quote),Trans,   NEXT_TAB,NEXT_TAB ],
        &[Trans,     Trans,       Trans,       Trans,      Trans,      Trans,      Trans,     Trans,    Trans,    Trans,   Trans,   Trans    ],
    ], &[
        &[k(Grave), k(Kb1), k(Kb2), k(Kb3), k(Kb4), k(Kb5),   k(Kb6), k(Kb7), k(Kb8),   k(Kb9),  k(Kb0), k(Insert)],
        &[Trans,    Trans,  Trans,  Trans,  Trans,  PREV_TAB, k(Left),k(Down),k(Up),    k(Right),Trans,  Trans    ],
        &[Trans,    Trans,  Trans,  Trans,  Trans,  NEXT_TAB, k(Home),k(PgUp),k(PgDown),k(End),  Trans,  Trans    ],
        &[Trans,    Trans,  Trans,  Trans,  Trans,  Trans,    Trans,  Trans,  Trans,    Trans,   Trans,  Trans    ],
    ], &[
        &[Trans,k(F1),k(F2),k(F3),k(F4),k(F5),k(F6),k(F7),k(F8),k(F9), k(F10),Trans],
        &[Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans,k(F11),k(F12),Trans],
        &[Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans, Trans, Trans],
        &[Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans, Trans, Trans],
    ],
];
