
module;

#include "basic.h"

export module winapi;

import basic;

import <Windows.h>;
import wininstance;

export class WinFactory {
cpub(
  WinFactory() {
    self.hinst = GetModuleHandle(0);
  }
  ~WinFactory() {

  }
)
  type(HINSTANCE)
  hinst = {};
cpub(
  fn create_window(
    const DescWinInit& desc
  ) -> optional<WinInstance> {
    return WinInstance();
  }
)
};
