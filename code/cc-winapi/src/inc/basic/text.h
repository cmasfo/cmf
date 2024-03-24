#pragma once

#ifndef TEXT
#ifdef _UNICODE
#define TEXT(P) P
#else
#define TEXT(P) L ## P
#endif
#endif
