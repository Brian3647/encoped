#[macro_export]
macro_rules! concatln {
    ($($e:expr),* $(,)?) => {
			concat!($($e, "\n"),*)
		};
}
