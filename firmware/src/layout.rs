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

const NEXT_TAB: Action = m(&[LCtrl, PgDown]);
const PREV_TAB: Action = m(&[LCtrl, PgUp]);

#[rustfmt::skip]
pub static LAYERS: keyberon::layout::Layers = &[
    &[
        &[k(Tab),     k(Q), k(W),   k(E),   k(R), k(T),    k(Y),    k(U), k(I),    k(O),    k(P),     k(BSpace)],
        &[LCTL_ESC,   k(A), k(S),   k(D),   k(F), k(G),    k(H),    k(J), k(K),    k(L),    k(SColon),RCTL_QUOT],
        &[k(LShift),  k(Z), k(X),   k(C),   k(V), k(B),    k(N),    k(M), k(Comma),k(Dot),  k(Slash), k(RShift)],
        &[Trans,      Trans,k(LGui),k(LAlt),l(1), k(Space),k(Enter),l(2), k(RAlt), k(BSpace),Trans,    Trans    ],
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
