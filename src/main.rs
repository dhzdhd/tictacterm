use cursive::{self, views::{TextView, Dialog, LinearLayout, Button, PaddedView}, Cursive, reexports::time::format_description::modifier::Padding, traits::Nameable};

fn main() {
    let mut siv = cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    siv.add_layer(Dialog::text("Welcome to terminal TicTacToe")
        .title("TicTacToe")
        .button("Play", game_view)
        .button("Quit", |s| s.quit()));

    siv.run();
}

fn game_view(s: &mut Cursive) {
    let mut is_x = false;

    s.pop_layer();
    s.add_layer(
        PaddedView::lrtb(
            10,
            10,
            5,
            5,
            LinearLayout::vertical()
                .child(LinearLayout::horizontal()
                    .child(Button::new(" ", |s| s.call_on_name("1", |button: &mut Button| button.set_label("X")).unwrap()).with_name("1"))
                    .child(Button::new(" ", |s| ()))
                    .child(Button::new(" ", |s| ()))
                )
                .child(LinearLayout::horizontal()
                    .child(Button::new(" ", |s| ()))
                    .child(Button::new(" ", |s| ()))
                    .child(Button::new(" ", |s| ()))
                )
                .child(LinearLayout::horizontal()
                    .child(Button::new(" ", |s| ()))
                    .child(Button::new(" ", |s| ()))
                    .child(button(is_x))
                )
        )
    );
}

fn button(is_x: bool) -> Button {
    Button::new(" ", |s: &mut Cursive| {
        s.call_on_name("", |button: &mut Button| {
            button.set_label(
                match is_x {
                    true => "x",
                    false => "o"
                }
            );
        }).unwrap()
    })
}
