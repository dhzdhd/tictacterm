use cursive::{self, views::{Dialog, LinearLayout, Button, PaddedView, NamedView}, Cursive, view::{Nameable}};

struct Row(String, String, String);

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
    let mut ttt_vec = [""].repeat(9);

    s.pop_layer();

    let mut body = LinearLayout::vertical();
    for i in 0..3 {
        let mut hort_layout = LinearLayout::horizontal();
        body.add_child(hort_layout);

        for j in 0..3 {
            // FIXME: :)))))))))))
            hort_layout.add_child(button(ttt_vec, format!("{}", i * j).as_str()))
        }
    }

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

                )
        )
    );
}

fn button(vec: Vec<&'static str>, label: &'static str) -> NamedView<Button> {
    return Button::new(" ", |s: &mut Cursive| {
        s.call_on_name(label, |button: &mut Button| { //FIXME:
            button.set_label(
                match vec.into_iter().filter(|e| *e != "").count() % 2 == 0 {
                    true => "x",
                    false => "o"
                }
            );
        }).unwrap()
    }).with_name(label);
}
