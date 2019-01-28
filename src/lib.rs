//! Adds convenience methods to `io::Seek` types via the [`SeekExt`] extension
//! trait.

use std::io::{Result, Seek, SeekFrom};


/// Adds convenience methods to all types that implement `io::Seek`.
///
/// This is an extension trait that has a blanket impl which implements this
/// trait for all `T where T: io::Seek`. You just need to import this trait
/// into scope and then you can use its methods on all `Seek` types.
pub trait SeekExt: Seek {
    /// Returns the length (in bytes) of this stream.
    ///
    /// This method is implemented using three seek operations. If this method
    /// returns successfully, the seek position is unchanged (i.e. the position
    /// before calling this method is the same as afterwards). However, if this
    /// method returns an error, the seek position is undefined.
    ///
    /// If you need to obtain the length of *many* streams and you don't care
    /// about the seek position afterwards, you can reduce the number of seek
    /// operations by simply calling `seek(SeekFrom::End(0))` and use its
    /// return value (it is also the stream length).
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use std::io::{Cursor, Seek, SeekFrom};
    /// use seek_ext::SeekExt;
    ///
    /// # fn main() -> Result<(), std::io::Error> {
    /// let mut c = Cursor::new(vec![0; 6]);
    /// let pos_before = c.seek(SeekFrom::Current(4))?;
    ///
    /// assert_eq!(c.stream_len()?, 6);
    /// assert_eq!(c.current_position()?, pos_before);
    /// # Ok(())
    /// # }
    /// ```
    fn stream_len(&mut self) -> Result<u64> {
        let old_pos = self.current_position()?;
        let len = self.seek(SeekFrom::End(0))?;
        self.seek(SeekFrom::Start(old_pos))?;
        Ok(len)
    }

    /// Returns the current seek position from the start of the stream.
    ///
    /// This is equivalent to `self.seek(SeekFrom::Current(0))`.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use std::io::{Cursor, Seek, SeekFrom};
    /// use seek_ext::SeekExt;
    ///
    /// # fn main() -> Result<(), std::io::Error> {
    /// let mut c = Cursor::new(vec![0; 6]);
    ///
    /// c.seek(SeekFrom::Current(4))?;
    /// assert_eq!(c.current_position()?, 4);
    ///
    /// c.seek(SeekFrom::Current(-3))?;
    /// assert_eq!(c.current_position()?, 1);
    /// # Ok(())
    /// # }
    /// ```
    fn current_position(&mut self) -> Result<u64> {
        self.seek(SeekFrom::Current(0))
    }
}

impl<T: Seek> SeekExt for T {}


#[cfg(test)]
mod tests {
    use std::io::{Cursor, Seek, SeekFrom};
    use super::SeekExt;

    #[test]
    fn stream_len() {
        let mut c = Cursor::new(vec![0; 15]);
        assert_eq!(c.stream_len().unwrap(), 15);

        c.seek(SeekFrom::End(0)).unwrap();
        let old_pos = c.current_position().unwrap();
        assert_eq!(c.stream_len().unwrap(), 15);
        assert_eq!(c.current_position().unwrap(), old_pos);

        c.seek(SeekFrom::Start(7)).unwrap();
        c.seek(SeekFrom::Current(2)).unwrap();
        let old_pos = c.current_position().unwrap();
        assert_eq!(c.stream_len().unwrap(), 15);
        assert_eq!(c.current_position().unwrap(), old_pos);
    }

    #[test]
    fn current_position() {
        // All `asserts` are duplicated here to make sure the method does not
        // change anything about the seek state.
        let mut c = Cursor::new(vec![0; 15]);
        assert_eq!(c.current_position().unwrap(), 0);
        assert_eq!(c.current_position().unwrap(), 0);

        c.seek(SeekFrom::End(0)).unwrap();
        assert_eq!(c.current_position().unwrap(), 15);
        assert_eq!(c.current_position().unwrap(), 15);


        c.seek(SeekFrom::Start(7)).unwrap();
        c.seek(SeekFrom::Current(2)).unwrap();
        assert_eq!(c.current_position().unwrap(), 9);
        assert_eq!(c.current_position().unwrap(), 9);

        c.seek(SeekFrom::End(-3)).unwrap();
        c.seek(SeekFrom::Current(1)).unwrap();
        c.seek(SeekFrom::Current(-5)).unwrap();
        assert_eq!(c.current_position().unwrap(), 8);
        assert_eq!(c.current_position().unwrap(), 8);
    }
}
