
#include "basic.h"

import <string>;

import basic;

import winapi;

fn main() -> i32 {
  init win_factory = WinFactory();

  init desc = DescWinInit();

  wstr test = L"";

  desc.win_title = TEXT("WinAPI");

  init window = win_factory.create_window(desc);

  return 0;
}
