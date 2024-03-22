
export module primitive;

import <cstdint>;

export using i8    = std::int8_t;
export using i16   = std::int16_t;
export using i32   = std::int32_t;
export using i64   = std::int64_t;
export using isize = std::intptr_t;

export using u8    = std::uint8_t;
export using u16   = std::uint16_t;
export using u32   = std::uint32_t;
export using u64   = std::uint64_t;
export using usize = std::uintptr_t;

export using f32 = float;
export using f64 = double;

export using ch  = char;
export using wch = wchar_t;
export using c8  = char8_t;
export using c16 = char16_t;
export using c32 = char32_t;

#ifdef _UNICODE // unicode character set
export using xch = wch;
#else // _MBCS // multi-byte character set
export using xch = ch;
#endif
