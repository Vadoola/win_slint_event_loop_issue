use slint::{Timer, TimerMode};

slint::include_modules!();

fn main() {
    let appwin = AppWin::new().unwrap();

    let timer = Timer::default();
    let timer_handle = appwin.as_weak();
    appwin.on_start(move || {
        let timer_handle = timer_handle.unwrap();
        timer.start(
            TimerMode::Repeated,
            std::time::Duration::from_millis(250),
            move || {
                let val = timer_handle.get_value();
                if val < 100 {
                    timer_handle.set_value(val + 1 );
                } else {
                    timer_handle.set_value(0);
                }
            });
    });

    appwin.run().unwrap();
}
