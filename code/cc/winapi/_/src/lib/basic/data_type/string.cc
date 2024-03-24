
export module string;

import <string>;

export using str  = std::string;
export using wstr = std::wstring;
export using s8   = std::u8string;
export using s16  = std::u16string;
export using s32  = std::u32string;

export using str_v  = std::string_view;
export using wstr_v = std::wstring_view;
export using s8_v   = std::u8string_view;
export using s16_v  = std::u16string_view;
export using s32_v  = std::u32string_view;

#ifdef _UNICODE // unicode character set
export using xstr   = std::wstring;
export using xstr_v = std::wstring_view;
#else // _MBCS // multi-byte character set
export using xstr   = std::string;
export using xstr_v = std::string_view;
#endif
