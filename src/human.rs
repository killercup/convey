use std::io;
use termcolor::{ColorChoice, StandardStream};
use Target;

pub fn stdout() -> Result<Target, io::Error> {
    Ok(Target::Human(Formatter {
        writer: StandardStream::stdout(ColorChoice::Auto),
    }))
}

pub struct Formatter {
    pub(crate) writer: StandardStream,
}

#[doc(hidden)]
#[macro_export]
macro_rules! __inner_span {
    ($span:ident, $attr:ident = $val:expr, $($token:tt)*) => {
        $span = $span.$attr($val)?;
        __inner_span!($span, $($token)*);
    };
    ($span:ident, [$($item:expr,)*]) => {
        $(
            $span = $span.add_item($item);
        )*
    };
}

#[macro_export]
macro_rules! span {
    ($($token:tt)*) => {
        {
            let mut the_span = span();
            __inner_span!(the_span, $($token)*);
            the_span
        }
    };
}

#[macro_export]
macro_rules! render_for_humans {
    ($this:ident -> [$($item:expr,)*]) => {
        fn render_for_humans(&self, fmt: &mut $crate::human::Formatter) -> Result<(), $crate::Error> {
            let $this = self;
            let span = span!([ $( $item, )* ]);
            span.render_for_humans(fmt)?;
            Ok(())
        }
    }
}
