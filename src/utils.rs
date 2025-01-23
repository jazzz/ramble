/// Synatic sugar for indenting strings. Specifically useful when generating formatted sourcecode.
///
/// # Examples
///
/// ```
/// let indented_field = indent!(1,"{}:{}","A","B")
/// ```
macro_rules! indent {
    ($indnt:expr, $fmt:tt, $($tts:tt)*) => { format!(concat!("{:indent$}",$fmt), "" , $($tts)*, indent=4*$indnt) } ;
    ($indnt:expr, $fmt:tt) => { format!("{:indent$}{}", "",$fmt, indent=4*$indnt) } ;
}

pub(crate) use indent;
