#[macro_export]
macro_rules! stmt {
    ( $i:expr, $cmd: expr ) => ({
        use nom::multispace;
        use ::parser::expressions::full_expression;
        chaining_parser!($i, 0usize,
            tag!("{%")                  ~
            l: opt!(tag!("-"))          ~
            many0!(multispace)          ~
            tag!($cmd)                  ~
            many0!(multispace)          ~
            e: opt!(chain!(f: full_expression ~ many0!(multispace), || f)) ~
            r: opt!(tag!("-"))          ~
            tag!("%}")                  ,
            || Statement { strip_left: l.is_some(), strip_right: r.is_some(), expression: e }
        )
    });
}


/// `take_till!(T -> bool) => &[T] -> IResult<&[T], &[T]>`
/// returns the longest list of bytes until the provided function succeeds
///
/// The argument is either a function `&[T] -> bool` or a macro returning a `bool
#[macro_export]
macro_rules! take_till_slc (
  ($input:expr, $submac:ident!( $($args:tt)* )) => (
    {
      use nom::InputLength;
      match $input.iter().enumerate().position(|(i, _)| $submac!(&$input[i..], $($args)*)) {
        Some(n) => IResult::Done(&$input[n..], &$input[..n]),
        None    => IResult::Done(&$input[($input).input_len()..], $input)
      }
    }
  );
  ($input:expr, $f:expr) => (
    take_till_slc!($input, call!($f));
  );
);


