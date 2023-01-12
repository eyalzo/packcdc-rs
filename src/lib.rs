/// The PackCDC chunker implementation.
/// Use `new` to construct a new chunker per buffer/stream.
/// Iterate over the `Chunk`s via the `Iterator` trait.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PackCDC<'a> {
    source: &'a [u8],
    bytes_processed: usize,
    bytes_remaining: usize,
    min_size: usize,
    avg_size: usize,
    max_size: usize,
    mask_first: u64,
    mask_second: u64,
}

impl<'a> PackCDC<'a> {
    const WINDOW_BYTES_LEN: usize = 48;
    // Mask for 15 bits (32KB) where 7 bits from right and left are not in
    const MASK_15: u64 = 0x00008A3114583280u64;
    const MASK_14: u64 = 0x00008A3110583280u64;

    pub fn new(source: &'a [u8]) -> Self {
        PackCDC::new_configurable(source, 8 * 1024, 32 * 1024, 64 * 1024,
                                  PackCDC::MASK_15, PackCDC::MASK_14)
    }

    /// Construct a new `PackCDC` that will process the given slice of bytes.
    /// `min_size` - minimum chunk size.
    fn new_configurable(source: &'a [u8], min_size: usize, avg_size: usize, max_size: usize,
                        mask_first: u64, mask_second: u64) -> Self {
        Self {
            source,
            bytes_processed: 0,
            bytes_remaining: source.len(),
            min_size,
            avg_size,
            max_size,
            mask_first,
            mask_second,
        }
    }

    /// Returns the offset and size of the newly found chunk
    fn find_anchor(&mut self, mut source_offset: usize, mut source_size: usize) -> usize {
        // Not enough bytes for a legal chunk, so let the caller know it should wait for more source bytes
        if source_size < self.min_size {
            return 0;
        }

        if source_size > self.max_size {
            source_size = self.max_size;
        }
        let source_start: usize = source_offset;
        let source_len1: usize =
            source_offset + center_size(self.avg_size, self.min_size, source_size);
        let source_len2: usize = source_offset + source_size;
        let mut hash: u64 = 0;
        source_offset += self.min_size;
        // Start by using the "harder" chunking judgement to find chunks
        // that run smaller than the desired normal size.
        while source_offset < source_len1 {
            let index = self.source[source_offset] as usize;
            source_offset += 1;
            hash = (hash << 1) ^ index as u64;
            if (hash & self.mask_first) == 0 {
                return source_offset - source_start;
            }
        }

        // Fall back to using the "easier" chunking judgement to find chunks
        // that run larger than the desired normal size.
        while source_offset < source_len2 {
            let index = self.source[source_offset] as usize;
            source_offset += 1;
            hash = (hash << 1) ^ index as u64;
            if (hash & self.mask_second) == 0 {
                return source_offset - source_start;
            }
        }
        // If source is not the last buffer, we may yet find a larger chunk.
        // If sourceSize === maximum, we will not find a larger chunk and should emit.
        if source_size < self.max_size {
            return 0;
        }

        // All else fails, return the whole chunk. This will happen with
        // pathological data, such as all zeroes.
        return source_size;
    }
}


/// Represents a chunk, used by the iterator
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Chunk {
    /// Starting byte position within the original buffer
    pub offset: usize,
    /// Length of the chunk in bytes
    pub length: usize,
}

impl<'a> Iterator for PackCDC<'a> {
    type Item = Chunk;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bytes_remaining == 0 {
            None
        } else {
            let chunk_size = self.find_anchor(self.bytes_processed, self.bytes_remaining);
            if chunk_size == 0 {
                None
            } else {
                let chunk_start = self.bytes_processed;
                self.bytes_processed += chunk_size;
                self.bytes_remaining -= chunk_size;
                Some(Chunk {
                    offset: chunk_start,
                    length: chunk_size,
                })
            }
        }
    }
}

///
/// Integer division that rounds up instead of down.
///
fn ceil_div(x: usize, y: usize) -> usize {
    (x + y - 1) / y
}

///
/// Find the middle of the desired chunk size, or what the FastCDC paper refers
/// to as the "normal size".
///
fn center_size(average: usize, minimum: usize, source_size: usize) -> usize {
    let mut offset: usize = minimum + ceil_div(minimum, 2);
    if offset > average {
        offset = average;
    }
    let size: usize = average - offset;
    if size > source_size {
        source_size
    } else {
        size
    }
}
