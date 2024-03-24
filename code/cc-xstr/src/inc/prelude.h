#pragma once

#define COUT(P) std::cout << P << std::endl

#ifdef _UNICODE
#define TEXT(P) L ## P
#else
#define TEXT(P) P
#endif
