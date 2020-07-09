use crate::{ColumnFamily, DBIterator, DBRawIterator, Direction, Error, IteratorMode, ReadOptions};

pub trait Iterate {
    fn get_raw_iter<'a: 'b, 'b>(&'a self, readopts: &ReadOptions) -> DBRawIterator<'b>;

    fn get_iter<'a: 'b, 'b>(
        &'a self,
        readopts: &ReadOptions,
        mode: IteratorMode<'_>,
    ) -> DBIterator<'b> {
        let mut rv = DBIterator {
            raw: self.get_raw_iter(readopts),
            direction: Direction::Forward, // blown away by set_mode()
            just_seeked: false,
        };
        rv.set_mode(mode);
        rv
    }

    fn iterator_opt<'a: 'b, 'b>(
        &'a self,
        mode: IteratorMode<'_>,
        readopts: &ReadOptions,
    ) -> DBIterator<'b> {
        self.get_iter(readopts, mode)
    }

    fn iterator<'a: 'b, 'b>(&'a self, mode: IteratorMode<'_>) -> DBIterator<'b> {
        let readopts = ReadOptions::default();
        self.iterator_opt(mode, &readopts)
    }

    /// Opens an interator with `set_total_order_seek` enabled.
    /// This must be used to iterate across prefixes when `set_memtable_factory` has been called
    /// with a Hash-based implementation.
    fn full_iterator<'a: 'b, 'b>(&'a self, mode: IteratorMode<'_>) -> DBIterator<'b> {
        let mut opts = ReadOptions::default();
        opts.set_total_order_seek(true);
        self.get_iter(&opts, mode)
    }

    fn prefix_iterator<'a: 'b, 'b>(&'a self, prefix: &[u8]) -> DBIterator<'b> {
        let mut opts = ReadOptions::default();
        opts.set_prefix_same_as_start(true);
        self.get_iter(&opts, IteratorMode::From(prefix, Direction::Forward))
    }

    fn raw_iterator<'a: 'b, 'b>(&'a self) -> DBRawIterator<'b> {
        let opts = ReadOptions::default();
        self.get_raw_iter(&opts)
    }
}

pub trait IterateCF: Iterate {
    fn get_raw_iter_cf<'a: 'b, 'b>(
        &'a self,
        cf_handle: &ColumnFamily,
        readopts: &ReadOptions,
    ) -> Result<DBRawIterator<'b>, Error>;

    fn get_iter_cf<'a: 'b, 'b>(
        &'a self,
        cf_handle: &ColumnFamily,
        readopts: &ReadOptions,
        mode: IteratorMode<'_>,
    ) -> Result<DBIterator<'b>, Error> {
        let mut rv = DBIterator {
            raw: self.get_raw_iter_cf(cf_handle, readopts)?,
            direction: Direction::Forward, // blown away by set_mode()
            just_seeked: false,
        };
        rv.set_mode(mode);
        Ok(rv)
    }

    /// Opens an interator using the provided ReadOptions.
    /// This is used when you want to iterate over a specific ColumnFamily with a modified ReadOptions
    fn iterator_cf_opt<'a: 'b, 'b>(
        &'a self,
        cf_handle: &ColumnFamily,
        mode: IteratorMode<'_>,
        readopts: &ReadOptions,
    ) -> Result<DBIterator<'b>, Error> {
        self.get_iter_cf(cf_handle, readopts, mode)
    }

    fn iterator_cf<'a: 'b, 'b>(
        &'a self,
        cf_handle: &ColumnFamily,
        mode: IteratorMode<'_>,
    ) -> Result<DBIterator<'b>, Error> {
        let opts = ReadOptions::default();
        self.get_iter_cf(cf_handle, &opts, mode)
    }

    fn full_iterator_cf<'a: 'b, 'b>(
        &'a self,
        cf_handle: &ColumnFamily,
        mode: IteratorMode<'_>,
    ) -> Result<DBIterator<'b>, Error> {
        let mut opts = ReadOptions::default();
        opts.set_total_order_seek(true);
        self.get_iter_cf(cf_handle, &opts, mode)
    }

    fn prefix_iterator_cf<'a: 'b, 'b>(
        &'a self,
        cf_handle: &ColumnFamily,
        prefix: &[u8],
    ) -> Result<DBIterator<'b>, Error> {
        let mut opts = ReadOptions::default();
        opts.set_prefix_same_as_start(true);
        self.get_iter_cf(
            cf_handle,
            &opts,
            IteratorMode::From(prefix, Direction::Forward),
        )
    }

    fn raw_iterator_cf<'a: 'b, 'b>(
        &'a self,
        cf_handle: &ColumnFamily,
    ) -> Result<DBRawIterator<'b>, Error> {
        let opts = ReadOptions::default();
        self.get_raw_iter_cf(cf_handle, &opts)
    }
}
