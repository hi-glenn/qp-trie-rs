use core::cmp;

// Get the "nybble index" corresponding to the `n`th nybble in the given slice.
//
// This is `1 + b` where `b` is the `n`th nybble, unless the given slice has less than `n / 2`
// elements, in which case `0` is returned.
// return diff nybble byte
#[inline]
pub fn nybble_index(n: usize, slice: &[u8]) -> u8 {
    // libc_print::libc_println!("nybble_index(); nybble_mismatch: {};", n);

    let byte_idx = n / 2;

    if byte_idx < slice.len() {
        let byte = slice[byte_idx];

        // If the index is even, return the lower nybble. Odd, the higher nybble.
        // In both cases, increment by one. The zero-index is reserved for the "head" of the sparse
        // array.
        if n & 1 == 0 {
            // nybble_mismatch 为偶数；取右 4 位
            libc_print::libc_println!("nybble_index(); n 为 偶数; index: 0b{:04b}; nybble_mismatch: {}; slice_len: {};", 1 + (byte & 0x0F), n, slice.len());
            1 + (byte & 0x0F)
        } else {
            // nybble_mismatch 为奇数；取左 4 位
            libc_print::libc_println!("nybble_index(); n 为 奇数; index: 0b{:04b}; nybble_mismatch: {}; slice_len: {};", 1 + (byte >> 4), n, slice.len());
            1 + (byte >> 4)
        }
    } else {
        // If the nybble is out-of-range, we return zero. This is not some sort of weird
        // convention which would be clearer served by an `Option`; instead, we're actually
        // returning the "head" index of the sparse array. In the case that our trie `Branch` node
        // here - say it's branching at the `nth` nybble - contains a single entry of exactly `n /
        // 2` bytes long, then we have to have someplace to put it - the head. Essentially the head
        // is where leaf nodes which do not live at the fringes of the tree are stored.
        libc_print::libc_println!("nybble_index(); 🐷; nybble_mismatch: {}; slice_len: {};", n, slice.len());
        0
    }
}

// Find the nybble at which the two provided slices mismatch. If no such nybble exists and the
// slices are the same length, `None` is returned; if no such nybble exists but the slices are
// *not* the same length, then the point at which one slice has a byte and the other has ended is
// considered the mismatch point.
#[inline]
pub fn nybble_mismatch(left: &[u8], right: &[u8]) -> Option<usize> {
    let mut difference;

    for (i, (l, r)) in left.iter().cloned().zip(right.iter().cloned()).enumerate() {
        difference = l ^ r;  // 比较有 4 种结果

        if difference != 0 {
            if difference & 0x0F == 0 {
            // if difference & 0xF0 > 0 {
                // 左 4 不同，右 4 相同；
                // 奇数

                libc_print::libc_println!("上 nybble_mismatch: {:?}", Some(1 + i * 2));

                return Some(1 + i * 2);
            } else {
                // 左 4 相同，右 4 不同；
                // 左 4 不同，右 4 不同；
                // 偶数

                libc_print::libc_println!("下 nybble_mismatch: {:?}", Some(i * 2));

                return Some(i * 2);
            }
        }

        // 左 4 相同，右 4 相同；啥也不做
    }

    if left.len() == right.len() {
        // 两 key 相同
        None
    } else {
        // "abc" 与 "abcd": 返回 "d" 第一个半字节的索引
        // "ab" 与 "ab`": 返回 "`" 第一个半字节的索引
        // 偶数

        Some(cmp::min(left.len(), right.len()) * 2)
    }
}

#[inline]
pub fn nybble_get_mismatch(left: &[u8], right: &[u8]) -> Option<(u8, usize)> {
    let mut difference;

    for (i, (l, r)) in left.iter().cloned().zip(right.iter().cloned()).enumerate() {
        difference = l ^ r;

        if difference != 0 {
            if difference & 0x0F == 0 {
                return Some((1 + (l >> 4), 1 + i * 2));
            } else {
                return Some((1 + (l & 0x0F), i * 2));
            }
        }
    }

    if left.len() == right.len() {
        None
    } else {
        let idx = cmp::min(left.len(), right.len()) * 2;

        Some((nybble_index(idx, left), idx))
    }
}

#[cfg(test)]
mod test {
    use alloc::vec::Vec;

    use quickcheck::TestResult;

    use super::*;

    quickcheck! {
        fn nybble(nybs: Vec<u8>) -> TestResult {
            for &nyb in &nybs {
                if nyb > 15 {
                    return TestResult::discard();
                }
            }

            let mut bytes = Vec::new();

            for chunk in nybs.chunks(2) {
                if chunk.len() == 2 {
                    bytes.push(chunk[0] | (chunk[1] << 4));
                } else {
                    bytes.push(chunk[0]);
                }
            }

            for (i, nyb) in nybs.into_iter().enumerate() {
                assert_eq!(nyb + 1, nybble_index(i, &bytes));
            }

            TestResult::passed()
        }
    }
}
