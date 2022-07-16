use embedded_hal::digital::v2::InputPin;

/// Micro driver for Small-tact-10 for arduino uno
#[derive(Debug)]
pub struct DirectButton<B: InputPin> {
    input: B,
}

impl<B: InputPin> DirectButton<B> {
    pub fn new(input: B) -> Self {
        Self { input }
    }

    // Method allows us to know if the button is pushed or not. Buttons is an pull-up digital input. If we get an error it will just return an false boolean.
    pub fn pushed(&self) -> bool {
        if let Ok(output) = self.input.is_high() {
            return !output;
        }
        false
    }
}
