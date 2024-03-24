
export module prelude;

export import <iostream>;

export import <string>;

#ifdef _UNICODE
export using xstr = std::wstring;
#else
export using xstr = std::string;
#endif
