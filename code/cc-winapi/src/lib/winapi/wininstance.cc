
module;

#include "basic.h"

export module wininstance;

import <Windows.h>;

import basic;

export class WinInstance {
cpub(
  WinInstance() {

  }
  ~WinInstance() {

  }
)
  type(HWND)
  hwnd = {};
};

export struct DescWinInit {
  type(wstr)
  win_title;
  type(u32)
  inner_size_x;
  type(u32)
  inner_size_y;
};
