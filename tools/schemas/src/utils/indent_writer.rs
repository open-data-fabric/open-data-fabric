pub struct IndentWriter<W: ?Sized> {
    indent_str: &'static str,
    indent: u16,
    indent_next: bool,
    inner: W,
}

impl<W> IndentWriter<W> {
    pub fn new(inner: W, indent_str: &'static str) -> Self {
        Self {
            inner,
            indent_str,
            indent_next: true,
            indent: 0,
        }
    }
}

impl<W: ?Sized> IndentWriter<W> {
    pub fn inc(&mut self) {
        self.indent += 1;
    }

    pub fn dec(&mut self) {
        if self.indent == 0 {
            panic!("Indentation underflow");
        }
        self.indent -= 1;
    }

    pub fn indent<'a>(&'a mut self) -> IndentGuard<'a, W> {
        self.inc();
        IndentGuard(self)
    }
}

impl<W> std::io::Write for IndentWriter<W>
where
    W: std::io::Write,
    W: ?Sized,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if self.indent_next {
            for _ in 0..self.indent {
                write!(self.inner, "{}", self.indent_str)?;
            }
        }
        self.inner.write_all(buf)?;

        self.indent_next = buf.last() == Some(&b'\n');

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

pub struct IndentGuard<'a, W: ?Sized>(&'a mut IndentWriter<W>);

impl<'a, W: ?Sized> Drop for IndentGuard<'a, W> {
    fn drop(&mut self) {
        self.0.dec();
    }
}

impl<'a, W> std::io::Write for IndentGuard<'a, W>
where
    W: std::io::Write,
    W: ?Sized,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}
